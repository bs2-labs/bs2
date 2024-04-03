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
    s_addi: Selector,
    _maker: PhantomData<F>,
}

impl<F: FieldExt> ITypeGadget<F> {
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        lhs_col: Column<Advice>,
        rhs_col: Column<Advice>,
        s_addi: Selector,
    ) -> Self {
        // let lhs_col = cs.advice_column();
        // let rhs_col = cs.advice_column();
        cs.enable_equality(lhs_col);
        cs.enable_equality(rhs_col);

        // todo: constrain selector: s1 + s1 + .. + sn = 1

        cs.create_gate("IType::ADDI", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_addi);
            // let (value, _) = rs1_value.overflowing_sub(rs2_value);
            vec![s * (lhs + rhs - out)]
        });

        Self {
            lhs_col,
            rhs_col,
            s_addi,
            _maker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, step: &OpStep) -> Result<(), Error> {
        layouter.assign_region(
            || "IType",
            |mut region| {
                let rd = step.instruction.op_a;
                let rs1 = step.instruction.op_b;
                let imm = step.instruction.op_c;

                dbg!(step, rd, rs1, imm);
                let rd_value = step.register_indexes.unwrap().write(rd).unwrap();
                let rs1_value = step.register_indexes.unwrap().read(rs1).unwrap();
                // todo: whether to ignore it

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

                match step.instruction.opcode.into() {
                    Opcode::ADDI => self.s_addi.enable(&mut region, 0)?,
                    _ => panic!("Not implemented {:?}", step.instruction.opcode),
                };
                Ok(())
            },
        )
    }
}
