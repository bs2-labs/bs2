use crate::execution_table::Entries;
use core::marker::PhantomData;

use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::circuit::{AssignedCell, Layouter, Value};

use halo2_proofs::plonk::*;
use halo2_proofs::poly::Rotation;

#[derive(Debug, Clone)]
pub struct ACell<F: FieldExt>(pub AssignedCell<F, F>);

#[derive(Clone)]
pub struct RTypeGadget<F> {
    pub lhs_col: Column<Advice>,
    pub rhs_col: Column<Advice>,
    s_add: Selector,
    s_sub: Selector,
    _maker: PhantomData<F>,
}

impl<F: FieldExt> RTypeGadget<F> {
    pub fn configure(cs: &mut ConstraintSystem<F>) -> Self {
        let lhs_col = cs.advice_column();
        let rhs_col = cs.advice_column();
        // todo: add selector for every gate
        let s_add = cs.selector();
        let s_sub = cs.selector();
        cs.enable_equality(lhs_col);
        cs.enable_equality(rhs_col);

        // todo: constrain selector: s1 + s1 + .. + sn = 1
        cs.create_gate("add", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_add);
            vec![s * (lhs + rhs - out)]
        });

        cs.create_gate("sub", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            vec![s * (lhs - rhs - out)]
        });

        Self {
            lhs_col,
            rhs_col,
            s_add,
            s_sub,
            _maker: PhantomData::default(),
        }
    }

    pub fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        // step: &Step,
        _entries: &Entries,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "rtype",
            |mut region| {
                // rs1_value
                // let col0 = step.registers[step.instruction.op_b as usize];
                // // rs2_value
                // let col1 = step.registers[step.instruction.op_c as usize];
                // // rd_value at next row
                // let col0_next = step.registers[step.instruction.op_a as usize];
                // let col0 = self.col0;
                // let col1 = self.col1;
                // todo: assign selector value to some position
                let s_add = self.s_add;
                // let s_sub = self.s_sub;
                s_add.enable(&mut region, 0)?;
                // s_sub.enable(&mut region, 0)?;

                region.assign_advice(|| "lhs", self.lhs_col, 0, || Value::known(F::from(100)))?;

                region.assign_advice(|| "rhs", self.rhs_col, 0, || Value::known(F::from(20)))?;

                region.assign_advice(
                    || "output",
                    self.lhs_col,
                    1,
                    || Value::known(F::from(120)),
                )?;

                Ok(())
            },
        )
    }
}
