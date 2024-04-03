use entry_builder::entries::Entries;
use entry_builder::op_step::OpStep;
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::circuit::Value;
use halo2_proofs::plonk::Advice;
use halo2_proofs::plonk::Column;
use halo2_proofs::plonk::Selector;
use halo2_proofs::{
    circuit::Layouter,
    plonk::{Circuit, ConstraintSystem, Error},
};
use runtime::trace::{InstructionType, Opcode};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct MemoryTable<F> {
    address_col: Column<Advice>,
    gc_col: Column<Advice>,
    value_col: Column<Advice>,
    rw_col: Column<Advice>,
    s: Selector,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> MemoryTable<F> {
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        address_col: Column<Advice>,
        gc_col: Column<Advice>,
        value_col: Column<Advice>,
        rw_col: Column<Advice>,
        s: Selector,
    ) -> Self {
        let lhs_col = cs.advice_column();
        let rhs_col = cs.advice_column();

        Self {
            address_col,
            gc_col,
            value_col,
            rw_col,
            s,
            _marker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, entries: &Entries) -> Result<(), Error> {
        let mut memory_ops: Vec<_> = entries.memory_ops.iter().collect();
        memory_ops.sort_by(|&(gc1, m1), &(gc2, m2)| {
            m1.address.cmp(&m2.address).then_with(|| gc1.cmp(gc2))
        });

        layouter.assign_region(
            || "memory table",
            |mut region| {
                self.s.enable(&mut region, 0)?;
                for (index, &(gc, memory_op)) in memory_ops.iter().enumerate() {
                    region.assign_advice(
                        || "address",
                        self.address_col,
                        index,
                        || Value::known(F::from(memory_op.address)),
                    )?;
                    region.assign_advice(
                        || "gc",
                        self.gc_col,
                        index,
                        || Value::known(F::from(memory_op.global_clk)),
                    )?;
                    region.assign_advice(
                        || "value",
                        self.value_col,
                        index,
                        || Value::known(F::from(memory_op.value)),
                    )?;
                    region.assign_advice(
                        || "rw",
                        self.rw_col,
                        index,
                        || Value::known(F::from(memory_op.rw.is_write())),
                    )?;
                }

                Ok(())
            },
        )
    }
}
