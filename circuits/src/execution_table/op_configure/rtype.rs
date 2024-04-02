use crate::execution_table::Entries;
use core::marker::PhantomData;
use halo2_proofs::arithmetic::{FieldExt, Field};
use halo2_proofs::circuit::{AssignedCell, Value, Layouter, SimpleFloorPlanner};
use halo2_proofs::halo2curves::bn256::Fr;
use halo2_proofs::plonk::*;
use halo2_proofs::poly::Rotation;

#[derive(Debug, Clone)]
pub struct ACell<F: FieldExt>(pub AssignedCell<F, F>);

#[derive(Clone)]
pub struct RTypeGadget<F> {
    pub col0: Column<Advice>,
    pub col1: Column<Advice>,
    selector: Selector,
    _maker: PhantomData<F>,
}

impl<F: FieldExt> RTypeGadget<F> {
    pub fn configure(cs: &mut ConstraintSystem<F>) -> Self {
        let col0 = cs.advice_column();
        let col1 = cs.advice_column();
        let selector = cs.selector();
        cs.enable_equality(col0);
        cs.enable_equality(col1);

        cs.create_gate("add", |cs| {
            let lhs = cs.query_advice(col0, Rotation::cur());
            let rhs = cs.query_advice(col1, Rotation::cur());
            let out = cs.query_advice(col0, Rotation::next());
            let s = cs.query_selector(selector);
            vec![s * (lhs + rhs - out)]
        });

        Self {
            col0,
            col1,
            selector,
            _maker: PhantomData::default(),
        }
    }

    pub fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        entries: &Entries,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "fibo",
            |mut region| {
                let selector = self.selector;
                selector.enable(&mut region, 0)?;

                region.assign_advice(
                    || "lhs",
                    self.col0,
                    0,
                    || Value::known(F::from(100)),
                )?;

                region.assign_advice(
                    || "private input",
                    self.col1,
                    0,
                    || Value::known(F::from(20)),
                )?;

                region.assign_advice(
                    || "private input",
                    self.col0,
                    1,
                    || Value::known(F::from(120)),
                )?;

                Ok(())
            }
        )
    }
}
