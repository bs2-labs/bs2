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
            vec![s * (lhs + rhs - out)]
        });

        Self {
            lhs_col,
            rhs_col,
            s_lui,
            _maker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, steps: &[OpStep]) -> Result<(), Error> {
        layouter.assign_region(
            || "UType",
            |mut region| {
                let cur_step = &steps[0];
                let imm = cur_step.instruction.op_b as u64;
                let pc = cur_step.pc;
                let out = steps[1].pc;

                region.assign_advice(
                    || "lhs",
                    self.lhs_col,
                    0,
                    || Value::known(F::from(imm)),
                )?;

                region.assign_advice(
                    || "rhs",
                    self.rhs_col,
                    0,
                    || Value::known(F::from(pc)),
                )?;

                region.assign_advice(
                    || "output",
                    self.lhs_col,
                    1,
                    || Value::known(F::from(out)),
                )?;

                match cur_step.instruction.opcode.into() {
                    Opcode::LUI => self.s_lui.enable(&mut region, 0)?,
                    _ => {
                        // TODO: handle other opcodes
                    }
                };
                Ok(())
            },
        )
    }
}
