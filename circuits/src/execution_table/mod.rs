use crate::execution_table::op_configure::rtype::RTypeGadget;
use entry_builder::entries::Entries;
use halo2_proofs::arithmetic::FieldExt;

use halo2_proofs::{
    circuit::Layouter,
    plonk::{Circuit, ConstraintSystem, Error},
};
use std::marker::PhantomData;

pub mod op_configure;

#[derive(Clone)]
pub struct ExecutionTable<F> {
    pub rtype: RTypeGadget<F>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> ExecutionTable<F> {
    pub fn configure(cs: &mut ConstraintSystem<F>) -> Self {
        Self {
            rtype: RTypeGadget::configure(cs),
            _marker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, entries: &Entries) -> Result<(), Error> {
        self.rtype.assign(layouter, entries)?;

        Ok(())
    }
}
