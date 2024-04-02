use crate::execution_table::op_configure::rtype::RTypeGadget;
use halo2_proofs::arithmetic::{FieldExt, Field};
use halo2_proofs::halo2curves::bn256::Fr;
use halo2_proofs::plonk::Column;
use halo2_proofs::plonk::Instance;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{Circuit, ConstraintSystem, Error},
};
use std::marker::PhantomData;

pub mod op_configure;

#[derive(Default, Clone)]
pub struct Entries {}

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

    pub fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        entries: &Entries,
    ) -> Result<(), Error> {
        self.rtype.assign(layouter, entries)?;

        Ok(())
    }
}
