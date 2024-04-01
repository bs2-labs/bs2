use runtime::trace::{InstructionType, Opcode, Step};

use core::fmt::Error;

/// Marker that defines whether an Operation performs a `READ` or a `WRITE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RW {
    /// Marks op as READ.
    READ,
    /// Marks op as WRITE.
    WRITE,
}

impl RW {
    /// Returns true if the RW corresponds internally to a [`READ`](RW::READ).
    pub const fn is_read(&self) -> bool {
        matches!(self, RW::READ)
    }
    /// Returns true if the RW corresponds internally to a [`WRITE`](RW::WRITE).
    pub const fn is_write(&self) -> bool {
        matches!(self, RW::WRITE)
    }
}

// new Memory or Regisger ops for u64 value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RwRegisterOp {
    pub global_clk: u64,
    pub rwc: u64,
    pub rw: RW,
    /// Register index
    pub index: u64,
    /// Value
    pub value: u64,
}

// new Memory or Regisger ops for u64 value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RwMemoryOp {
    pub global_clk: u64,
    pub rw: RW,
    /// Memory address
    pub address: u64,
    /// Value
    pub value: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RwContainer {
    /// Operations of memory and register
    pub rw_memory_ops: Vec<RwMemoryOp>,
    /// Operations of memory and register
    pub rw_register_ops: Vec<RwRegisterOp>,
}

impl RwContainer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update_pc_register(&mut self, _gc: u64, _value: u64) {
        // TODO: save pc register in a separated table.
    }

    pub fn push_read_register_op(&mut self, gc: u64, rwc: u64, index: u64, value: u64) {
        let read_op = RwRegisterOp {
            global_clk: gc,
            rwc,
            rw: RW::READ,
            index,
            value,
        };
        self.rw_register_ops.push(read_op);
    }

    pub fn push_write_register_op(&mut self, gc: u64, rwc: u64, index: u64, value: u64) {
        let write_op = RwRegisterOp {
            global_clk: gc,
            rwc,
            rw: RW::WRITE,
            index,
            value,
        };
        self.rw_register_ops.push(write_op);
    }

    pub fn push_read_memory_op(&mut self, gc: u64, address: u64, value: u8) {
        let read_op = RwMemoryOp {
            global_clk: gc,
            rw: RW::READ,
            address,
            value,
        };
        self.rw_memory_ops.push(read_op);
    }

    pub fn push_write_memory_op(&mut self, gc: u64, memory: u64, value: u8) {
        let write_op = RwMemoryOp {
            global_clk: gc,
            rw: RW::WRITE,
            address: memory,
            value,
        };
        self.rw_memory_ops.push(write_op);
    }

    pub fn step_rtype(&mut self, step: &Step) -> Result<(), Error> {
        let opcode = step.instruction.opcode;
        let rs1_value = step.registers[step.instruction.op_b as usize];
        let rs2_value = step.registers[step.instruction.op_c as usize];
        let result = match opcode {
            Opcode::ADD => rs1_value + rs2_value,
            Opcode::SUB => rs1_value - rs2_value,
            Opcode::SLL => {
                let shift_value = rs2_value.clone() & SHIFT_MASK;
                rs1_value.clone() << shift_value
            },
            Opcode::SRL => {
                let shift_value = rs2_value.clone() & SHIFT_MASK;
                rs1_value.clone() >> shift_value
            },
            Opcode::SRA => {
                rs1_value.clone() >> rs2_value
            },
            Opcode::SLT => {
                (rs1_value < rs2_value).into()
            },
            Opcode::SLTU => {
                (rs1_value < rs2_value).into()
            },
            Opcode::XOR => {
                rs1_value ^ rs2_value
            },
            Opcode::OR => {
                rs1_value | rs2_value
            },
            Opcode::AND => {
                rs1_value & rs2_value
            },
            Opcode::MUL => {
                rs1_value.overflowing_mul(rs2_value).0
            },
            Opcode::MULH => {
                let a = i128::from(rs1_value as i64);
                let b = i128::from(rs2_value as i64);
                let (value, _) = a.overflowing_mul(b);
                (value >> 64) as u64
            },
            Opcode::MULHU => {
                let a = u128::from(rs1_value);
                let b = u128::from(rs2_value);
                let (value, _) = a.overflowing_mul(b);
                (value >> 64) as u64
            },
            Opcode::MULHSU => {
                let a = i128::from(rs1_value as i64);
                let b = i128::from(rs2_value);
                let (value, _) = a.overflowing_mul(b);
                (value >> 64) as u64
            },
            Opcode::DIV => {
                // rs1_value.overflowing_div_signed(rs2_value);
                if rs1_value == 0 {
                    u64::max_value()
                } else {
                    rs1_value.overflowing_div(rs2_value).0
                }
            },
            Opcode::DIVU => {
                // rs1_value.overflowing_div(rs2_value);
                if rs2_value == 0 {
                    (-1i64) as u64
                } else {
                    let (v, o) = (rs1_value as i64).overflowing_div(rs2_value as i64);
                    if o {
                        // -2**(L-1) implemented using (-1) << (L - 1)
                        ((-1i64) as u64) << (64 - 1)
                    } else {
                        v as u64
                    }
                }
            },
            Opcode::REM => {
                // rs1_value.overflowing_rem_signed(rs2_value);
                if rs2_value == 0 {
                    rs1_value
                } else {
                    (rs1_value).overflowing_rem(rs2_value).0
                }
            },
            Opcode::REMU => {
                // rs1_value.overflowing_rem(rs2_value);
                if rs2_value == 0 {
                    rs1_value
                } else {
                    let (v, o) = (rs1_value as i64).overflowing_rem(rs2_value as i64);
                    if o {
                        0
                    } else {
                        v as u64
                    }
                }
            },
            _ => unimplemented!("Not implemented {:?}", step.instruction.opcode),
        };
        // read rs1
        self.push_read_register_op(step.global_clk, 0, step.instruction.op_b, b);

        // read rs2
        self.push_read_register_op(step.global_clk, 1, step.instruction.op_c, c);

        // write rd
        self.push_write_register_op(step.global_clk, 2, step.instruction.op_a, result);

        Ok(())
    }

    pub fn step_btype(&mut self, step: &Step) -> Result<(), Error> {
        let opcode = step.instruction.opcode;
        let rs1 = step.registers[step.instruction.op_a as usize];
        let rs2 = step.registers[step.instruction.op_b as usize];
        let imm = step.registers[step.instruction.op_c as usize];

        let new_pc = if match opcode {
            Opcode::BEQ => rs1 as i64 == rs2 as i64,
            Opcode::BNE => rs1 as i64 != rs2 as i64,
            Opcode::BGE => rs1 as i64 >= rs2 as i64,
            Opcode::BGEU => rs1 as u64 >= rs2 as u64,
            Opcode::BLT => (rs1 as i64) < (rs2 as i64),
            Opcode::BLTU => (rs1 as u64) < (rs2 as u64),
            _ => unimplemented!("Not implemented {:?}", step.instruction.opcode),
        } {
            step.pc + imm as u64
        } else {
            step.pc + step.instruction.get_instruction_length() as u64
        };
        self.push_read_register_op(step.global_clk, step.instruction.op_a, 0, rs1);

        self.push_read_register_op(step.global_clk, step.instruction.op_b, 1, rs2);

        self.update_pc_register(step.global_clk, new_pc);

        Ok(())
    }

    pub fn step(&mut self, step: &Step) -> Result<(), Error> {
        let opcode = step.instruction.opcode;
        match opcode.into() {
            InstructionType::RType => self.step_rtype(step),
            InstructionType::BType => self.step_btype(step),
            _ => {
                unimplemented!("Not implemented {:?}", step.instruction.opcode);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::EntryBuilder;
    use crate::rw_container::{RwRegisterOp, RW};
    use runtime::trace::Trace;

    #[test]
    fn btype_gen() {
        let trace_json = r#"
        {
          "circles": 26809,
          "failed": false,
          "returnValue": "0",
          "steps": [
              {
                "global_clk": 0,
                "pc": 65772,
                "inst_type": "BType",
                "instruction": {
                        "opcode": "ADD",
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
            builder.rw_container,
            RwContainer {
                rw_register_ops: vec![
                    RwRegisterOp {
                        global_clk: 0,
                        rwc: 0,
                        rw: RW::READ,
                        index: 1,
                        value: 0
                    },
                    RwRegisterOp {
                        global_clk: 0,
                        rwc: 1,
                        rw: RW::READ,
                        index: 3,
                        value: 0
                    },
                    RwRegisterOp {
                        global_clk: 0,
                        rwc: 2,
                        rw: RW::WRITE,
                        index: 31,
                        value: 0
                    }
                ],
                ..Default::default()
            }
        );
    }

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
                "inst_type": "RType",
                "instruction": {
                        "opcode": "ADD",
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
            builder.rw_container,
            RwContainer {
                rw_register_ops: vec![
                    RwRegisterOp {
                        global_clk: 0,
                        rwc: 0,
                        rw: RW::READ,
                        index: 1,
                        value: 0
                    },
                    RwRegisterOp {
                        global_clk: 0,
                        rwc: 1,
                        rw: RW::READ,
                        index: 3,
                        value: 0
                    },
                    RwRegisterOp {
                        global_clk: 0,
                        rwc: 2,
                        rw: RW::WRITE,
                        index: 31,
                        value: 0
                    }
                ],
                ..Default::default()
            }
        );
    }
}
