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
pub struct OthersTypeGadget<F> {
    pub lhs_col: Column<Advice>,
    pub rhs_col: Column<Advice>,
    s_fence: Selector,
    _maker: PhantomData<F>,
}

impl<F: FieldExt> OthersTypeGadget<F> {
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        lhs_col: Column<Advice>,
        rhs_col: Column<Advice>,
        s_fence: Selector,
    ) -> Self {
        // let lhs_col = cs.advice_column();
        // let rhs_col = cs.advice_column();
        cs.enable_equality(lhs_col);
        cs.enable_equality(rhs_col);

        // todo: constrain selector: s1 + s1 + .. + sn = 1

        Self {
            lhs_col,
            rhs_col,
            s_fence,
            _maker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, step: &OpStep) -> Result<(), Error> {
        layouter.assign_region(
            || "OthersType",
            |mut region| {
                match step.instruction.opcode.into() {
                    Opcode::FENCE => self.s_fence.enable(&mut region, 0)?,
                    _ => {
                        return Ok(());
                    },
                };
                Ok(())
            },
        )
    }
}
