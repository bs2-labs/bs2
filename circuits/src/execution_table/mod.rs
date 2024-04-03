use entry_builder::entries::Entries;
use halo2_proofs::arithmetic::FieldExt;

use halo2_proofs::{
    circuit::Layouter,
    plonk::{Circuit, ConstraintSystem, Error},
};
use std::marker::PhantomData;

pub mod op_configure;
use op_configure::rtype::RTypeGadget;
use op_configure::itype::ITypeGadget;

#[derive(Clone)]
pub struct ExecutionTable<F> {
    pub rtype: RTypeGadget<F>,
    pub itype: ITypeGadget<F>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> ExecutionTable<F> {
    pub fn configure(cs: &mut ConstraintSystem<F>) -> Self {
        let lhs_col = cs.advice_column();
        let rhs_col = cs.advice_column();
        let s_add = cs.selector();
        let s_sub = cs.selector();

        Self {
            rtype: RTypeGadget::configure(cs, lhs_col, rhs_col, s_add, s_sub),
            itype: ITypeGadget::configure(cs, lhs_col, rhs_col, s_add, s_sub),
            _marker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, entries: &Entries) -> Result<(), Error> {
        self.rtype.assign(layouter, entries)?;
        self.itype.assign(layouter, entries)?;

        Ok(())
    }
}
