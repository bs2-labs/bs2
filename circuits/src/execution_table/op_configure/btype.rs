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
pub struct BTypeGadget<F> {
    pub lhs_col: Column<Advice>,
    pub rhs_col: Column<Advice>,
    s_beq: Selector,
    _maker: PhantomData<F>,
}

impl<F: FieldExt> BTypeGadget<F> {
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        lhs_col: Column<Advice>,
        rhs_col: Column<Advice>,
        s_beq: Selector,
    ) -> Self {
        // let lhs_col = cs.advice_column();
        // let rhs_col = cs.advice_column();
        cs.enable_equality(lhs_col);
        cs.enable_equality(rhs_col);

        // todo: constrain selector: s1 + s1 + .. + sn = 1

        // cs.create_gate("BType::BEQ", |vc| {
        //     let lhs = vc.query_advice(lhs_col, Rotation::cur());
        //     let rhs = vc.query_advice(rhs_col, Rotation::cur());
        //     // let out = vc.query_advice(lhs_col, Rotation::next());
        //     let s = vc.query_selector(s_beq);
        //     vec![s * (lhs - rhs)]
        // });

        Self {
            lhs_col,
            rhs_col,
            s_beq,
            _maker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, step: &OpStep) -> Result<(), Error> {
        layouter.assign_region(
            || "BType",
            |mut region| {
                // todo
                // let rs1 = step.instruction.op_a;
                // let rs2 = step.instruction.op_b;
                // let imm = step.instruction.op_c;
                // let rs1_value = step.register_indexes.unwrap().read(rs1).unwrap();
                // let rs2_value = step.register_indexes.unwrap().read(rs2).unwrap();
                // // todo: how to use it?
                // let imm_value = step.register_indexes.unwrap().read(imm).unwrap();

                // region.assign_advice(
                //     || "lhs",
                //     self.lhs_col,
                //     0,
                //     || Value::known(F::from(rs1_value)),
                // )?;

                // region.assign_advice(
                //     || "rhs",
                //     self.rhs_col,
                //     0,
                //     || Value::known(F::from(rs2_value)),
                // )?;

                // match step.instruction.opcode.into() {
                //     Opcode::BEQ => self.s_beq.enable(&mut region, 0)?,
                //     _ => panic!("Not implemented {:?}", step.instruction.opcode),
                // };
                Ok(())
            },
        )
    }
}
