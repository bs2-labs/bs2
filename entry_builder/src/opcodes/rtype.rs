use crate::opcodes::OpcodeFn;
use crate::opcodes::RwContainer;
use crate::rw_container::RwOp;
use crate::rw_container::RW;
use core::fmt::Error;
use runtime::trace::Step;

#[derive(Debug, Copy, Clone)]
pub(crate) struct RType;

impl OpcodeFn for RType {
    fn gen_associated_ops(rw_contaienr: &mut RwContainer, step: &Step) -> Result<(), Error> {
        // read rs1
        rw_contaienr.push_read_op(
            step.global_clk,
            step.instruction.op_b,
            step.registers[step.instruction.op_b as usize],
        );

        // read rs2
        rw_contaienr.push_read_op(
            step.global_clk,
            step.instruction.op_c,
            step.registers[step.instruction.op_c as usize],
        );

        // write rd
        rw_contaienr.push_write_op(
            step.global_clk,
            step.instruction.op_a,
            step.registers[step.instruction.op_a as usize],
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::EntryBuilder;
    use runtime::trace::Trace;
    use crate::opcodes::rtype::RW;

    #[test]
    fn rtype_gen() {
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

        let mut builder = EntryBuilder::new();
        builder.build(&trace).expect("build entries failed");

        assert_eq!(
            builder.rw_contaienr,
            RwContainer {
                rw_ops: vec![
                    RwOp {
                        global_clk: 0,
                        rwc: 0,
                        rw: RW::READ,
                        address: 1,
                        value: 0
                    },
                    RwOp {
                        global_clk: 0,
                        rwc: 1,
                        rw: RW::READ,
                        address: 3,
                        value: 0
                    },
                    RwOp {
                        global_clk: 0,
                        rwc: 2,
                        rw: RW::WRITE,
                        address: 31,
                        value: 0
                    }
                ]
            }
        );
    }
}
