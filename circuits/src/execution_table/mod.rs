use entry_builder::entries::Entries;
use entry_builder::op_step::OpStep;
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::{
    circuit::Layouter,
    plonk::{Circuit, ConstraintSystem, Error},
};
use runtime::trace::{InstructionType, Opcode};
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
        // RType selector
        let s_add = cs.selector();
        let s_sub = cs.selector();
        let s_subw = cs.selector();
        let s_sll = cs.selector();
        let s_srl = cs.selector();
        let s_sra = cs.selector();
        let s_slt = cs.selector();
        let s_sltu = cs.selector();
        let s_xor = cs.selector();
        let s_or = cs.selector();
        let s_and = cs.selector();
        let s_mul = cs.selector();
        let s_mulh = cs.selector();
        let s_mulhu = cs.selector();
        let s_mulhsu = cs.selector();
        let s_div = cs.selector();
        let s_divu = cs.selector();
        let s_rem = cs.selector();
        let s_remu = cs.selector();
        let s_addw = cs.selector();
        let s_sllw = cs.selector();
        let s_srlw = cs.selector();
        let s_sraw = cs.selector();

        Self {
            rtype: RTypeGadget::configure(
                cs, lhs_col, rhs_col, s_add, s_sub, s_subw, s_sll, s_srl, s_sra,
                s_slt, s_sltu, s_xor, s_or, s_and, s_mul, s_mulh, s_mulhu, s_mulhsu, s_div, s_divu,
                s_rem, s_remu, s_addw, s_sllw, s_srlw, s_sraw
            ),
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
                }
            };
        }

        Ok(())
    }
}
