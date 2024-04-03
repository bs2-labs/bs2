use core::marker::PhantomData;
use entry_builder::entries::Entries;
use entry_builder::op_step::OpStep;

use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::circuit::{AssignedCell, Layouter, Value};
use halo2_proofs::plonk::*;
use halo2_proofs::poly::Rotation;

#[derive(Debug, Clone)]
pub struct ACell<F: FieldExt>(pub AssignedCell<F, F>);

#[derive(Clone)]
pub struct UTypeGadget<F> {
    pub lhs_col: Column<Advice>,
    pub rhs_col: Column<Advice>,
    s_add: Selector,
    s_sub: Selector,
    _maker: PhantomData<F>,
}

impl<F: FieldExt> UTypeGadget<F> {
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        lhs_col: Column<Advice>,
        rhs_col: Column<Advice>,
        s_add: Selector,
        s_sub: Selector,
    ) -> Self {
        cs.enable_equality(lhs_col);
        cs.enable_equality(rhs_col);

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

    pub fn assign(&self, layouter: &mut impl Layouter<F>, step: &OpStep) -> Result<(), Error> {
        layouter.assign_region(
            || "utype",
            |mut region| {
                let s_add = self.s_add;
                s_add.enable(&mut region, 0)?;

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
