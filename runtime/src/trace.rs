use serde::{Deserialize, Serialize};
use std::vec::Vec;

// TODO: relay on ckb opcode
pub type Opcode = u32;

// TODO: use enum
type InstructionType = u32;

/// An instruction specifies an operation to execute and the operands.
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub op_a: u64,
    pub op_b: u64,
    pub op_c: u64,
    pub imm_b: bool,
    pub imm_c: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Step {
    pub global_clk: u64,
    pub pc: u64,
    pub inst_type: InstructionType,
    pub instruction: Instruction,
    pub registers: Vec<u64>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Trace {
    pub circles: u64,
    pub failed: bool,
    #[serde(rename = "returnValue")]
    pub return_value: String,
    pub steps: Vec<Step>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_trace() {
        let trace_json = r#"
        {
          "circles": 26809,
          "failed": false,
          "returnValue": "0",
          "steps": [
              {
                "global_clk": 0,
                "pc": 65772,
                "inst_type": 0,
                "instruction": {
                        "opcode": 35,
                        "op_a": 31,
                        "op_b": 1,
                        "op_c": 3,
                        "imm_b": true,
                        "imm_c": true
                },
                "registers": [ 0, 0, 494288, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]
              }
          ]
        }"#;
        let trace: Trace = serde_json::from_str(trace_json).expect("json-deserialize Trace failed");
        assert_eq!(
            trace,
            Trace {
                circles: 26809,
                failed: false,
                return_value: "0".into(),
                steps: vec![Step {
                    global_clk: 0,
                    pc: 65772,
                    inst_type: 0,
                    instruction: Instruction {
                        opcode: 35,
                        op_a: 31,
                        op_b: 1,
                        op_c: 3,
                        imm_b: true,
                        imm_c: true,
                    },
                    registers: vec![
                        0, 0, 494288, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0
                    ],
                }]
            }
        )
    }
}
