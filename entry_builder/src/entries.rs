use crate::{op_step::OpStep, Register};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use core::fmt::Error;
use runtime::trace::{BType, IType, InstructionType, JType, NoType, RType, SType, Step, UType};
use std::{
    io::{Cursor, Seek, SeekFrom},
    ops::Shr,
};

/// Marker that defines whether an Operation performs a `READ` or a `WRITE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RW {
    /// Marks op as READ.
    READ,
    /// Marks op as WRITE.
    WRITE,
}

const SHIFT_MASK: u64 = 0x3f;

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
pub struct RegisterOp {
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
pub struct MemoryOp {
    pub global_clk: u64,
    pub rw: RW,
    /// Memory address
    pub address: u64,
    /// Value
    pub value: u64,
    /// Width
    pub width: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entries {
    /// Operations of memory
    pub memory_ops: Vec<MemoryOp>,
    /// Operations of register
    pub register_ops: Vec<RegisterOp>,
    /// Opereation contexts of each step
    pub op_steps: Vec<OpStep>,

    /// Memory values to faciliate the memory operations
    /// Default to 32MB memory as ckb-vm may have flexible memory size.
    pub memory_buffer: Vec<u8>,
    pub register_buffer: Vec<u64>,

    pub should_copy_registers: bool,

    /// Temporary register to store the counter for operations within an instruction.
    pub rwc: u64,
}

impl Default for Entries {
    fn default() -> Self {
        Self::new()
    }
}

impl Entries {
    pub fn new() -> Self {
        Self {
            memory_ops: Vec::new(),
            register_ops: Vec::new(),
            op_steps: Vec::new(),
            // Some registers has initial state, so we need to copy them at first.
            should_copy_registers: true,
            memory_buffer: [0; 1024 * 1024 * 32].to_vec(),
            register_buffer: [0; 32].to_vec(),
            rwc: 0,
        }
    }

    pub fn update_pc_register(&mut self, _gc: u64, _value: u64) {
        // TODO: save pc register in a separated table.
    }

    pub fn read_register(&mut self, gc: u64, index: u64, value: u64) {
        assert_eq!(
            self.register_buffer[index as usize], value,
            "Read the wrong register value"
        );
        let read_op = RegisterOp {
            global_clk: gc,
            rwc: self.rwc,
            rw: RW::READ,
            index,
            value,
        };
        self.register_ops.push(read_op);
        self.rwc += 1;
    }

    pub fn write_register(&mut self, gc: u64, index: u64, value: u64) {
        if index != 0 {
            self.register_buffer[index as usize] = value;
        }
        let write_op = RegisterOp {
            global_clk: gc,
            rwc: self.rwc,
            rw: RW::WRITE,
            index,
            value,
        };
        self.register_ops.push(write_op);
        self.rwc += 1;
    }

    pub fn read_memory(&mut self, gc: u64, address: u64, width: u8) -> u64 {
        let mut reader = Cursor::new(&self.memory_buffer);
        reader.seek(SeekFrom::Start(address)).unwrap();
        let value = match width {
            8 => reader.read_u8().unwrap() as u64,
            16 => reader.read_u16::<LittleEndian>().unwrap() as u64,
            32 => reader.read_u32::<LittleEndian>().unwrap() as u64,
            64 => reader.read_u64::<LittleEndian>().unwrap() as u64,
            _ => panic!("Not implemented {:?}", width),
        };

        let read_op = MemoryOp {
            global_clk: gc,
            rw: RW::READ,
            address,
            value,
            width,
        };
        self.memory_ops.push(read_op);
        value
    }

    pub fn write_memory(&mut self, gc: u64, address: u64, value: u64, width: u8) {
        let write_op = MemoryOp {
            global_clk: gc,
            rw: RW::WRITE,
            address,
            value,
            width,
        };
        self.memory_ops.push(write_op);
        let mut writer = Cursor::new(&mut self.memory_buffer);
        writer.seek(SeekFrom::Start(address)).unwrap();
        match width {
            8 => writer.write_u8(value as u8).unwrap(),
            16 => writer.write_u16::<LittleEndian>(value as u16).unwrap(),
            32 => writer.write_u32::<LittleEndian>(value as u32).unwrap(),
            64 => writer.write_u64::<LittleEndian>(value as u64).unwrap(),
            _ => panic!("Not implemented {:?}", width),
        }
    }

    pub fn step_rtype(&mut self, rtype: RType, step: &Step) -> Result<(), Error> {
        let rs1_value = step.registers[step.instruction.op_b as usize];
        let rs2_value = step.registers[step.instruction.op_c as usize];
        let result = match rtype {
            RType::ADD => rs1_value + rs2_value,
            RType::SUB => rs1_value - rs2_value,
            RType::SUBW => {
                let (value, _) = rs1_value.overflowing_sub(rs2_value);
                value.sign_extend(&32)
            }
            RType::SLL => {
                let shift_value = rs2_value.clone() & SHIFT_MASK;
                rs1_value.clone() << shift_value
            }
            RType::SRL => {
                let shift_value = rs2_value.clone() & SHIFT_MASK;
                rs1_value.clone() >> shift_value
            }
            RType::SRA => rs1_value.clone() >> rs2_value,
            RType::SLT => (rs1_value < rs2_value).into(),
            RType::SLTU => (rs1_value < rs2_value).into(),
            RType::XOR => rs1_value ^ rs2_value,
            RType::OR => rs1_value | rs2_value,
            RType::AND => rs1_value & rs2_value,
            RType::MUL => rs1_value.overflowing_mul(rs2_value).0,
            RType::MULH => {
                let a = i128::from(rs1_value as i64);
                let b = i128::from(rs2_value as i64);
                let (value, _) = a.overflowing_mul(b);
                (value >> 64) as u64
            }
            RType::MULHU => {
                let a = u128::from(rs1_value);
                let b = u128::from(rs2_value);
                let (value, _) = a.overflowing_mul(b);
                (value >> 64) as u64
            }
            RType::MULHSU => {
                let a = i128::from(rs1_value as i64);
                let b = i128::from(rs2_value);
                let (value, _) = a.overflowing_mul(b);
                (value >> 64) as u64
            }
            RType::DIV => {
                // rs1_value.overflowing_div_signed(rs2_value);
                if rs1_value == 0 {
                    u64::max_value()
                } else {
                    rs1_value.overflowing_div(rs2_value).0
                }
            }
            RType::DIVU => {
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
            }
            RType::REM => {
                // rs1_value.overflowing_rem_signed(rs2_value);
                if rs2_value == 0 {
                    rs1_value
                } else {
                    (rs1_value).overflowing_rem(rs2_value).0
                }
            }
            RType::REMU => {
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
            }
            RType::ADDW => (rs1_value + rs2_value).sign_extend(&32),
            RType::SLLW => {
                let shift_value = rs2_value.clone() & SHIFT_MASK;
                let result = rs1_value.clone() << shift_value;
                result.sign_extend(&32)
            }
            RType::SRLW => {
                let shift_value = rs2_value.clone() & SHIFT_MASK;
                let result = rs1_value.clone() >> shift_value;
                result.sign_extend(&32)
            }
            RType::SRAW => {
                let result = rs1_value.clone() >> rs2_value;
                result.sign_extend(&32)
            }
        };
        // read rs1
        self.read_register(step.global_clk, step.instruction.op_b, rs1_value);

        // read rs2
        self.read_register(step.global_clk, step.instruction.op_c, rs2_value);

        // write rd
        self.write_register(step.global_clk, step.instruction.op_a, result);

        Ok(())
    }

    pub fn step_stype_or_btype(&mut self, step: &Step) -> (u64, u64, u64) {
        let rs1 = step.registers[step.instruction.op_a as usize];
        let rs2 = step.registers[step.instruction.op_b as usize];
        let imm = step.instruction.op_c as u64;
        self.read_register(step.global_clk, step.instruction.op_a, rs1);
        self.read_register(step.global_clk, step.instruction.op_b, rs2);
        (rs1, rs2, imm)
    }

    pub fn step_stype(&mut self, stype: SType, step: &Step) -> Result<(), Error> {
        let (rs1, rs2, imm) = self.step_stype_or_btype(step);

        let (addr, _) = (rs1 as i64).overflowing_add(imm as i64);
        let addr = addr as u64;
        let value = rs2;
        dbg!(stype);
        match stype {
            SType::SB => {
                self.write_memory(step.global_clk, addr, value, 8);
            }
            SType::SH => {
                self.write_memory(step.global_clk, addr, value, 16);
            }
            SType::SW => {
                self.write_memory(step.global_clk, addr, value, 32);
            }
            SType::SD => {
                self.write_memory(step.global_clk, addr, value, 64);
            }
            _ => panic!("Not implemented {:?}", step.instruction.opcode),
        }
        Ok(())
    }

    pub fn step_itype(&mut self, itype: IType, step: &Step) -> Result<(), Error> {
        dbg!(step);
        let rd_index = step.instruction.op_a;
        let rs1 = step.registers[step.instruction.op_b as usize];
        let imm = step.instruction.op_c;

        let addr = Register::overflowing_add(&rs1, &u64::from_i32(imm as i32));

        // TODO: we didn't consider word width while doing some arithematic operation.
        match itype {
            IType::JALR => {
                let result = step.pc + step.instruction.get_instruction_length();
                let next_pc = rs1 + imm;
                let next_pc = next_pc;
                self.write_register(step.global_clk, rd_index, result);
                self.update_pc_register(step.global_clk, next_pc);
            }
            IType::ADDI => {
                let rs1 = rs1 as i64;
                let imm = imm.sign_extend(&32) as i64;
                let result = rs1 + imm;
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::SLTI => {
                let result = (rs1 as i64) < (imm as i64);
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::SLTIU => {
                let result = (rs1 as u64) < (imm as u64);
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::XORI => {
                let rs1 = rs1 as i64;
                let imm = imm as i64;
                let result = rs1 ^ imm;
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::ORI => {
                let rs1 = rs1 as i64;
                let imm = imm as i64;
                let result = rs1 | imm;
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::ANDI => {
                let rs1 = rs1 as i64;
                let imm = imm as i64;
                let result = rs1 & imm;
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::SLLI => {
                let rs1 = rs1 as u64;
                let imm = imm as u64;
                let result = rs1 << imm;
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::SRLI => {
                let rs1 = rs1 as u64;
                let imm = imm as u64;
                let result = rs1 >> imm;
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::SRAI => {
                let rs1 = rs1 as i64;
                let imm = imm as u64;
                let result = rs1.shr(imm);
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::SLLIW => {
                let rs1 = rs1 as u64;
                let imm = imm as u64;
                let result = rs1 << imm;
                let result = result.sign_extend(&32);
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::ADDIW => {
                let rs1 = rs1 as i64;
                let imm = imm as i64;
                let result = rs1 + imm;
                let result = (result as u64).sign_extend(&32);
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::SRLIW => {
                let rs1 = rs1 as u64;
                let imm = imm as u64;
                let result = rs1 >> imm;
                let result = (result as u64).sign_extend(&32);
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::SRAIW => {
                let rs1 = rs1 as i64;
                let imm = imm as u64;
                let result = rs1.shr(imm);
                let result = (result as u64).sign_extend(&32);
                self.write_register(step.global_clk, rd_index, result as u64);
            }
            IType::LB | IType::LBU => {
                self.should_copy_registers = true;
                self.read_memory(step.global_clk, addr, 8);
            }
            IType::LH | IType::LHU => {
                self.should_copy_registers = true;
                self.read_memory(step.global_clk, addr, 16);
            }
            IType::LW | IType::LWU => {
                self.should_copy_registers = true;
                self.read_memory(step.global_clk, addr, 32);
            }
            IType::LD => {
                self.should_copy_registers = true;
                self.read_memory(step.global_clk, addr, 64);
            }
        }
        Ok(())
    }

    pub fn step_jtype(&mut self, jtype: JType, step: &Step) -> Result<(), Error> {
        let rd_index = step.instruction.op_a;
        let imm = step.instruction.op_b as i32;

        match jtype {
            JType::JAL => {
                let result = step.pc + step.instruction.get_instruction_length();
                let next_pc = Register::overflowing_add(&step.pc, &u64::from_i32(imm.clone()));
                self.write_register(step.global_clk, rd_index, result);
                self.update_pc_register(step.global_clk, next_pc);
            }
        }
        Ok(())
    }

    pub fn step_btype(&mut self, btype: BType, step: &Step) -> Result<(), Error> {
        let (rs1, rs2, imm) = self.step_stype_or_btype(step);

        let new_pc = if match btype {
            BType::BEQ => rs1 as i64 == rs2 as i64,
            BType::BNE => rs1 as i64 != rs2 as i64,
            BType::BGE => rs1 as i64 >= rs2 as i64,
            BType::BGEU => rs1 as u64 >= rs2 as u64,
            BType::BLT => (rs1 as i64) < (rs2 as i64),
            BType::BLTU => (rs1 as u64) < (rs2 as u64),
        } {
            step.pc + imm as u64
        } else {
            step.pc + step.instruction.get_instruction_length() as u64
        };
        self.update_pc_register(step.global_clk, new_pc);

        Ok(())
    }

    pub fn step_utype(&mut self, u: UType, step: &Step) -> Result<(), Error> {
        let imm = step.instruction.op_b as u64;

        let result = match u {
            UType::LUI => imm as u64,
            UType::AUIPC => {
                let new_pc = (step.pc + imm) as u64;
                self.update_pc_register(step.global_clk, new_pc);
                new_pc
            }
        };

        // write rd
        self.write_register(step.global_clk, step.instruction.op_a, result);

        Ok(())
    }

    pub fn step_notype(&mut self, n: NoType, _step: &Step) -> Result<(), Error> {
        match n {
            NoType::FENCE => (),
            NoType::ECALL => {
                self.should_copy_registers = true;
            }
            NoType::EBREAK => todo!(),
            NoType::UNIMP => todo!(),
        };

        Ok(())
    }

    pub fn step(&mut self, step: &Step) -> Result<(), Error> {
        dbg!(step);
        self.rwc = 0;
        let opcode = step.instruction.opcode;
        if self.should_copy_registers {
            self.register_buffer = step.registers.clone();
            self.should_copy_registers = false;
        } else {
            for (index, (left, right)) in
                step.registers.iter().zip(self.register_buffer.clone()).enumerate()
            {
                assert_eq!(
                    *left, right,
                    "Register {} is not the same in step {}",
                    index, step.global_clk
                )
            }
        }

        match opcode.into() {
            InstructionType::RType(r) => self.step_rtype(r, step),
            InstructionType::BType(b) => self.step_btype(b, step),
            InstructionType::SType(s) => self.step_stype(s, step),
            InstructionType::IType(i) => self.step_itype(i, step),
            InstructionType::JType(j) => self.step_jtype(j, step),
            InstructionType::UType(u) => self.step_utype(u, step),
            InstructionType::NoType(n) => self.step_notype(n, step),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::EntryBuilder;
    use crate::entries::{RegisterOp, RW};
    use runtime::trace::Trace;

    #[test]
    fn btype_gen() {
        let trace_json = r#"
        {
          "cycles": 26809,
          "return_value": 0,
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
            builder.rw_container.register_ops,
            vec![
                RegisterOp {
                    global_clk: 0,
                    rwc: 0,
                    rw: RW::READ,
                    index: 1,
                    value: 0
                },
                RegisterOp {
                    global_clk: 0,
                    rwc: 1,
                    rw: RW::READ,
                    index: 3,
                    value: 0
                },
                RegisterOp {
                    global_clk: 0,
                    rwc: 2,
                    rw: RW::WRITE,
                    index: 31,
                    value: 0
                }
            ],
        );
    }

    #[test]
    fn rtype_gen() {
        let trace_json = r#"
        {
          "cycles": 26809,
          "return_value": 0,
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
            builder.rw_container.register_ops,
            vec![
                RegisterOp {
                    global_clk: 0,
                    rwc: 0,
                    rw: RW::READ,
                    index: 1,
                    value: 0
                },
                RegisterOp {
                    global_clk: 0,
                    rwc: 1,
                    rw: RW::READ,
                    index: 3,
                    value: 0
                },
                RegisterOp {
                    global_clk: 0,
                    rwc: 2,
                    rw: RW::WRITE,
                    index: 31,
                    value: 0
                }
            ],
        );
    }
}
