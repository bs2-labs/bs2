use entry_builder::entries::Entries;
use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::{
    circuit::Layouter,
    plonk::{Circuit, ConstraintSystem, Error},
};
use runtime::trace::{InstructionType, Opcode};
use core::marker::PhantomData;

pub mod op_configure;
use op_configure::btype::BTypeGadget;
use op_configure::itype::ITypeGadget;
use op_configure::rtype::RTypeGadget;
use op_configure::utype::UTypeGadget;
use op_configure::jtype::JTypeGadget;
use op_configure::stype::STypeGadget;
use op_configure::others::OthersTypeGadget;

#[derive(Clone)]
pub struct ExecutionTable<F> {
    pub btype: BTypeGadget<F>,
    pub rtype: RTypeGadget<F>,
    pub itype: ITypeGadget<F>,
    pub utype: UTypeGadget<F>,
    pub jtype: JTypeGadget<F>,
    pub stype: STypeGadget<F>,
    pub others: OthersTypeGadget<F>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> ExecutionTable<F> {
    pub fn configure(cs: &mut ConstraintSystem<F>) -> Self {
        // Common 
        let lhs_col = cs.advice_column();
        let rhs_col = cs.advice_column();
        let s_overflowing = cs.advice_column();

        // RType selector
        let s_add = cs.selector();
        let s_sub = cs.selector();
        
        // BType selector
        let s_beq = cs.selector();

        // IType selector
        let s_addi = cs.selector();

        // JType selector
        let s_jal = cs.selector();

        // SType selector
        let s_sw = cs.selector();

        // UType selector
        let s_lui = cs.selector();

        // Others selector
        let s_lw = cs.selector();

        Self {
            btype: BTypeGadget::configure(cs, lhs_col, rhs_col, s_beq),
            itype: ITypeGadget::configure(cs, lhs_col, rhs_col, s_overflowing, s_addi),
            jtype: JTypeGadget::configure(cs, lhs_col, rhs_col, s_jal),
            rtype: RTypeGadget::configure(cs, lhs_col, rhs_col, s_add, s_sub),
            stype: STypeGadget::configure(cs, lhs_col, rhs_col, s_sw),
            utype: UTypeGadget::configure(cs, lhs_col, rhs_col, s_lui),
            others: OthersTypeGadget::configure(cs, lhs_col, rhs_col, s_lw),
            _marker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, entries: &Entries) -> Result<(), Error> {
        let op_steps = entries.get_op_steps();

        for (index, op_step) in op_steps.iter().enumerate() {
            match op_step.instruction.opcode.into() {
                InstructionType::BType(_) => self.btype.assign(layouter, op_step),
                InstructionType::IType(_) => self.itype.assign(layouter, op_step),
                InstructionType::RType(_) => self.rtype.assign(layouter, op_step),
                InstructionType::SType(_) => self.stype.assign(layouter, op_step),
                InstructionType::UType(_) => {
                    let slice = &op_steps[index..=index+1];
                    self.utype.assign(layouter, slice)
                },
                InstructionType::JType(_) => self.jtype.assign(layouter, op_step),
                InstructionType::NoType(_) => self.others.assign(layouter, op_step),
                _ => panic!("Not implemented {:?}", op_step.instruction.opcode),
            }?;
        }

        Ok(())
    }
}
