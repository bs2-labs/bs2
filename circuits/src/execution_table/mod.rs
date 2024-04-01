use halo2_proofs::arithmetic::Field;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{Circuit, ConstraintSystem, Error},
};

use std::marker::PhantomData;

#[derive(Clone)]
pub struct ExecutionTableConfig<F> {
    pub a: u32,
    _marker: PhantomData<F>,
}

#[derive(Default, Clone)]
pub struct ExecutionTable<F> {
    pub b: u32,
    _marker: PhantomData<F>,
}

impl<F: Field> ExecutionTable<F> {
    pub fn new() -> Self {
        Self {
            b: 0,
            _marker: PhantomData::default(),
        }
    }

    pub fn instance(&self) -> Vec<Vec<F>> {
        let mut instance = Vec::new();

        instance
    }
}

impl<F: Field> Circuit<F> for ExecutionTable<F> {
    type Config = ExecutionTableConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // let log_circuit_info = |meta: &ConstraintSystem<F>, tag: &str| {
        //     log::debug!("circuit info after {}: {:#?}", tag, circuit_stats(meta));
        // };
        ExecutionTableConfig {
            a: 0,
            _marker: PhantomData::default(),
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        log::debug!("assigning state_circuit");

        Ok(())
    }
}
