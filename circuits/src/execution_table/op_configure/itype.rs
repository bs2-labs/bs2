use core::marker::PhantomData;
use entry_builder::op_step::OpStep;
use runtime::trace::Opcode;

use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::circuit::{AssignedCell, Layouter, Value};
use halo2_proofs::plonk::*;
use halo2_proofs::poly::Rotation;

#[derive(Debug, Clone)]
pub struct ACell<F: FieldExt>(pub AssignedCell<F, F>);

#[derive(Clone)]
pub struct ITypeGadget<F> {
    pub lhs_col: Column<Advice>,
    pub rhs_col: Column<Advice>,
    pub s_overflowing: Column<Advice>,
    s_addi: Selector,
    _maker: PhantomData<F>,
}

impl<F: FieldExt> ITypeGadget<F> {
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        lhs_col: Column<Advice>,
        rhs_col: Column<Advice>,
        s_overflowing: Column<Advice>,
        s_addi: Selector,
    ) -> Self {
        cs.enable_equality(lhs_col);
        cs.enable_equality(rhs_col);

        // todo: constrain selector: s1 + s1 + .. + sn = 1

        cs.create_gate("IType::ADDI", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s_overflowing = vc.query_advice(s_overflowing, Rotation::cur());
            let s_addi = vc.query_selector(s_addi);
            vec![s_addi * (lhs + rhs - out - s_overflowing * Expression::Constant(F::from(u64::max_value())))]
        });

        Self {
            lhs_col,
            rhs_col,
            s_overflowing,
            s_addi,
            _maker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, step: &OpStep) -> Result<(), Error> {
        layouter.assign_region(
            || "IType",
            |mut region| {
                match step.instruction.opcode.into() {
                    Opcode::ADDI => self.s_addi.enable(&mut region, 0)?,
                    _ => {
                        return Ok(());
                    }
                };
                let rd = step.instruction.op_a;
                let rs1 = step.instruction.op_b;
                let imm = step.instruction.op_c;

                dbg!(step, rd, rs1, imm);
                let rd_value = step.register_indexes.unwrap().write(rd).unwrap();
                let rs1_value = step.register_indexes.unwrap().read(rs1).unwrap();

                let (_, is_overflowing) = rs1_value.overflowing_add(imm);

                dbg!(rd_value, rs1_value);
                region.assign_advice(
                    || "lhs",
                    self.lhs_col,
                    0,
                    || Value::known(F::from(rs1_value)),
                )?;

                region.assign_advice(|| "rhs", self.rhs_col, 0, || Value::known(F::from(imm)))?;

                region.assign_advice(
                    || "output",
                    self.lhs_col,
                    1,
                    || Value::known(F::from(rd_value)),
                )?;

                region.assign_advice(
                    || "is_overflowing",
                    self.s_overflowing,
                    0,
                    || Value::known(F::from(is_overflowing)),
                )?;

                Ok(())
            },
        )
    }
}
