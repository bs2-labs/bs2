use crate::opcodes::gen_associated_ops;
use crate::rw_container::RwContainer;
use core::fmt::Error;
use runtime::trace::Trace;

pub struct EntryBuilder {
    pub rw_contaienr: RwContainer,
}

impl Default for EntryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EntryBuilder {
    pub fn new() -> EntryBuilder {
        Self {
            rw_contaienr: RwContainer::new(),
        }
    }

    pub fn build(&mut self, trace: &Trace) -> Result<(), Error> {
        for (_index, step) in trace.steps.iter().enumerate() {
            // match step.instruction.opcode
            // TODO: store rw operations to container
            gen_associated_ops(step.instruction.opcode, &mut self.rw_contaienr, step)?;
        }

        Ok(())
    }
}
