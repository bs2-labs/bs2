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
                // TODO

                Ok(())
            },
        )
    }
}
