use runtime::trace::Instruction;

use crate::entries::{MemoryOp, RegisterOps};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpStep<'a> {
    pub global_clk: u64,
    pub pc: u64,
    pub instruction: &'a Instruction,
    pub register_indexes: Option<&'a RegisterOps>,
    pub memory_address: Option<&'a MemoryOp>,
}
