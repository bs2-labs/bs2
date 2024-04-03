use entry_builder::entries::Entries;
use entry_builder::op_step::OpStep;
use runtime::trace::{InstructionType, Opcode};
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::{
    circuit::Layouter,
    plonk::{Circuit, ConstraintSystem, Error},
};
use std::marker::PhantomData;

pub mod op_configure;
use op_configure::itype::ITypeGadget;
use op_configure::rtype::RTypeGadget;

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
        let op_steps = entries.get_op_steps();

        for (_index, op_step) in op_steps.iter().enumerate() {
            let instruction_type: InstructionType = op_step.instruction.opcode.into();
            match instruction_type {
                InstructionType::BType(_) => self.rtype.assign(layouter, op_step),
                InstructionType::IType(_) => self.itype.assign(layouter, op_step),
                InstructionType::SType(_) => self.rtype.assign(layouter, op_step),
                InstructionType::UType(_) => self.rtype.assign(layouter, op_step),
                InstructionType::JType(_) => self.rtype.assign(layouter, op_step),
                InstructionType::NoType(_) => self.rtype.assign(layouter, op_step),
                _ => {
                    unimplemented!("unimplement instruction type");
                },
            };
        }

        Ok(())
    }
}
