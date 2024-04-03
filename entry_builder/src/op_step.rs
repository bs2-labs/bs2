use runtime::trace::Instruction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpStep {
    pub global_clk: u64,
    pub pc: u64,
    pub instruction: Instruction,
    pub register_indexes: Vec<u32>,
    pub memory_indexes: Vec<u32>,
}
