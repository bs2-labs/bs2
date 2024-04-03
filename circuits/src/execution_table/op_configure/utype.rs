use core::marker::PhantomData;
use runtime::trace::Opcode;
use entry_builder::op_step::OpStep;

use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::circuit::{AssignedCell, Layouter, Value};
use halo2_proofs::plonk::*;
use halo2_proofs::poly::Rotation;

#[derive(Debug, Clone)]
pub struct ACell<F: FieldExt>(pub AssignedCell<F, F>);

#[derive(Clone)]
pub struct UTypeGadget<F> {
    pub lhs_col: Column<Advice>,
    pub rhs_col: Column<Advice>,
    s_lui: Selector,
    _maker: PhantomData<F>,
}

impl<F: FieldExt> UTypeGadget<F> {
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        lhs_col: Column<Advice>,
        rhs_col: Column<Advice>,
        s_lui: Selector,
    ) -> Self {
        // let lhs_col = cs.advice_column();
        // let rhs_col = cs.advice_column();
        cs.enable_equality(lhs_col);
        cs.enable_equality(rhs_col);

        // todo: constrain selector: s1 + s1 + .. + sn = 1

        cs.create_gate("UType::LUI", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_lui);
            // let (value, _) = rs1_value.overflowing_sub(rs2_value);
            vec![s * (lhs + rhs - out)]
        });

        Self {
            lhs_col,
            rhs_col,
            s_lui,
            _maker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, step: &OpStep) -> Result<(), Error> {
        layouter.assign_region(
            || "UType",
            |mut region| {
                // todo
                let rs1 = step.instruction.op_b;
                let rs2 = step.instruction.op_c;
                let rd = step.instruction.op_a;
                let rs1_value = step.register_indexes.unwrap().read(rs1).unwrap();
                let rs2_value = step.register_indexes.unwrap().read(rs2).unwrap();
                let rd_value = step.register_indexes.unwrap().read(rd).unwrap();

                region.assign_advice(
                    || "lhs",
                    self.lhs_col,
                    0,
                    || Value::known(F::from(rs1_value)),
                )?;

                region.assign_advice(
                    || "rhs",
                    self.rhs_col,
                    0,
                    || Value::known(F::from(rs2_value)),
                )?;

                region.assign_advice(
                    || "output",
                    self.lhs_col,
                    1,
                    || Value::known(F::from(rd_value)),
                )?;

                match step.instruction.opcode.into() {
                    Opcode::LUI => self.s_lui.enable(&mut region, 0)?,
                    _ => panic!("Not implemented {:?}", step.instruction.opcode),
                };
                Ok(())
            },
        )
    }
}
