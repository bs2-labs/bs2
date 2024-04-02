use crate::execution_table::Entries;
use crate::execution_table::{self, ExecutionTable};
use halo2_proofs::arithmetic::{FieldExt, Field};
use halo2_proofs::halo2curves::bn256::Fr;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{Circuit, Column, ConstraintSystem, Error, Instance},
};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct MainConfig<F> {
    pub execution_table: ExecutionTable<F>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> MainConfig<F> {
    fn configure(cs: &mut ConstraintSystem<F>) -> Self {
        let execution_table = ExecutionTable::configure(cs);

        Self {
            execution_table,
            _marker: PhantomData::default(),
        }
    }

    fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        entries: &Entries,
    ) -> Result<(), Error> {
        self.execution_table.assign(layouter, entries);
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct MainCircuit<F> {
    entries: Entries,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> MainCircuit<F> {
    pub fn new() -> Self {
        Self {
            entries: Entries {},
            _marker: PhantomData::default(),
        }
    }

    pub fn instance(&self) -> Vec<Vec<F>> {
        let mut instance = Vec::new();

        instance
    }
}

impl<F: FieldExt> Circuit<F> for MainCircuit<F> {
    type Config = MainConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(cs: &mut ConstraintSystem<F>) -> Self::Config {
        // let log_circuit_info = |meta: &ConstraintSystem<F>, tag: &str| {
        //     log::debug!("circuit info after {}: {:#?}", tag, circuit_stats(meta));
        // };
        MainConfig::configure(cs)
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        log::debug!("assigning state_circuit");

        config.assign(&mut layouter, &self.entries);

        Ok(())
    }
}
