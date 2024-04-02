use serde::{Deserialize, Serialize};
use std::vec::Vec;

// TODO: relay on ckb opcode
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    // Arithmetic instructions.
    ADD = 0,
    SUB = 1,
    XOR = 2,
    OR = 3,
    AND = 4,
    SLL = 5,
    SRL = 6,
    SRA = 7,
    SLT = 8,
    SLTU = 9,

    // Load instructions.
    LB = 10,
    LH = 11,
    LW = 12,
    LBU = 13,
    LHU = 14,

    // Store instructions.
    SB = 15,
    SH = 16,
    SW = 17,

    // Branch instructions.
    BEQ = 18,
    BNE = 19,
    BLT = 20,
    BGE = 21,
    BLTU = 22,
    BGEU = 23,

    // Jump instructions.
    JAL = 24,
    JALR = 25,
    AUIPC = 27,

    // System instructions.
    ECALL = 28,
    EBREAK = 29,

    // Multiplication instructions.
    MUL = 30,
    MULH = 31,
    MULHU = 32,
    MULHSU = 33,
    DIV = 34,
    DIVU = 35,
    REM = 36,
    REMU = 37,

    LUI = 38,

    ADDI = 39,
    SLTI = 40,
    SLTIU = 41,
    XORI = 42,
    ORI = 43,
    ANDI = 44,
    SLLI = 45,
    SRLI = 46,
    SRAI = 47,

    FENCE = 48,
    SUBW = 49,
    // Miscellaneaous instructions.
    UNIMP = 255,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0 => Opcode::ADD,
            1 => Opcode::SUB,
            2 => Opcode::XOR,
            3 => Opcode::OR,
            4 => Opcode::AND,
            5 => Opcode::SLL,
            6 => Opcode::SRL,
            7 => Opcode::SRA,
            8 => Opcode::SLT,
            9 => Opcode::SLTU,
            10 => Opcode::LB,
            11 => Opcode::LH,
            12 => Opcode::LW,
            13 => Opcode::LBU,
            14 => Opcode::LHU,
            15 => Opcode::SB,
            16 => Opcode::SH,
            17 => Opcode::SW,
            18 => Opcode::BEQ,
            19 => Opcode::BNE,
            20 => Opcode::BLT,
            21 => Opcode::BGE,
            22 => Opcode::BLTU,
            23 => Opcode::BGEU,
            24 => Opcode::JAL,
            25 => Opcode::JALR,
            27 => Opcode::AUIPC,
            28 => Opcode::ECALL,
            29 => Opcode::EBREAK,
            30 => Opcode::MUL,
            31 => Opcode::MULH,
            32 => Opcode::MULHU,
            33 => Opcode::MULHSU,
            34 => Opcode::DIV,
            35 => Opcode::DIVU,
            36 => Opcode::REM,
            37 => Opcode::REMU,
            38 => Opcode::LUI,
            39 => Opcode::ADDI,
            40 => Opcode::SLTI,
            41 => Opcode::SLTIU,
            42 => Opcode::XORI,
            43 => Opcode::ORI,
            44 => Opcode::ANDI,
            45 => Opcode::SLLI,
            46 => Opcode::SRLI,
            47 => Opcode::SRAI,
            48 => Opcode::FENCE,
            49 => Opcode::SUBW,

            255 => Opcode::UNIMP,
            _ => panic!("Invalid opcode value"),
        }
    }
}

impl Opcode {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "ADD" => Some(Opcode::ADD),
            "SUB" => Some(Opcode::SUB),
            "XOR" => Some(Opcode::XOR),
            "OR" => Some(Opcode::OR),
            "AND" => Some(Opcode::AND),
            "SLL" => Some(Opcode::SLL),
            "SRL" => Some(Opcode::SRL),
            "SRA" => Some(Opcode::SRA),
            "SLT" => Some(Opcode::SLT),
            "SLTU" => Some(Opcode::SLTU),
            "LB" => Some(Opcode::LB),
            "LH" => Some(Opcode::LH),
            "LW" => Some(Opcode::LW),
            "LBU" => Some(Opcode::LBU),
            "LHU" => Some(Opcode::LHU),
            "SB" => Some(Opcode::SB),
            "SH" => Some(Opcode::SH),
            "SW" => Some(Opcode::SW),
            "BEQ" => Some(Opcode::BEQ),
            "BNE" => Some(Opcode::BNE),
            "BLT" => Some(Opcode::BLT),
            "BGE" => Some(Opcode::BGE),
            "BLTU" => Some(Opcode::BLTU),
            "BGEU" => Some(Opcode::BGEU),
            "JAL" => Some(Opcode::JAL),
            "JALR" => Some(Opcode::JALR),
            "AUIPC" => Some(Opcode::AUIPC),
            "ECALL" => Some(Opcode::ECALL),
            "EBREAK" => Some(Opcode::EBREAK),
            "MUL" => Some(Opcode::MUL),
            "MULH" => Some(Opcode::MULH),
            "MULHU" => Some(Opcode::MULHU),
            "MULHSU" => Some(Opcode::MULHSU),
            "DIV" => Some(Opcode::DIV),
            "DIVU" => Some(Opcode::DIVU),
            "REM" => Some(Opcode::REM),
            "REMU" => Some(Opcode::REMU),
            "LUI" => Some(Opcode::LUI),
            "ADDI" => Some(Opcode::ADDI),
            "SLTI" => Some(Opcode::SLTI),
            "SLTIU" => Some(Opcode::SLTIU),
            "XORI" => Some(Opcode::XORI),
            "ORI" => Some(Opcode::ORI),
            "ANDI" => Some(Opcode::ANDI),
            "SLLI" => Some(Opcode::SLLI),
            "SRLI" => Some(Opcode::SRLI),
            "SRAI" => Some(Opcode::SRAI),
            "FENCE" => Some(Opcode::FENCE),
            "SUBW" => Some(Opcode::SUBW),

            "UNIMP" => Some(Opcode::UNIMP),
            _ => None,
        }
    }
}

impl Serialize for Opcode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Opcode::ADD => "ADD",
            Opcode::SUB => "SUB",
            Opcode::XOR => "XOR",
            Opcode::OR => "OR",
            Opcode::AND => "AND",
            Opcode::SLL => "SLL",
            Opcode::SRL => "SRL",
            Opcode::SRA => "SRA",
            Opcode::SLT => "SLT",
            Opcode::SLTU => "SLTU",
            Opcode::LB => "LB",
            Opcode::LH => "LH",
            Opcode::LW => "LW",
            Opcode::LBU => "LBU",
            Opcode::LHU => "LHU",
            Opcode::SB => "SB",
            Opcode::SH => "SH",
            Opcode::SW => "SW",
            Opcode::BEQ => "BEQ",
            Opcode::BNE => "BNE",
            Opcode::BLT => "BLT",
            Opcode::BGE => "BGE",
            Opcode::BLTU => "BLTU",
            Opcode::BGEU => "BGEU",
            Opcode::JAL => "JAL",
            Opcode::JALR => "JALR",
            Opcode::AUIPC => "AUIPC",
            Opcode::ECALL => "ECALL",
            Opcode::EBREAK => "EBREAK",
            Opcode::MUL => "MUL",
            Opcode::MULH => "MULH",
            Opcode::MULHU => "MULHU",
            Opcode::MULHSU => "MULHSU",
            Opcode::DIV => "DIV",
            Opcode::DIVU => "DIVU",
            Opcode::REM => "REM",
            Opcode::REMU => "REMU",
            Opcode::UNIMP => "UNIMP",
            Opcode::LUI => "LUI",
            Opcode::ADDI => "ADDI",
            Opcode::SLTI => "SLTI",
            Opcode::SLTIU => "SLTIU",
            Opcode::XORI => "XORI",
            Opcode::ORI => "ORI",
            Opcode::ANDI => "ANDI",
            Opcode::SLLI => "SLLI",
            Opcode::SRLI => "SRLI",
            Opcode::SRAI => "SRAI",
            Opcode::FENCE => "FENCE",
            Opcode::SUBW => "SUBW",
        })
    }
}

impl<'de> Deserialize<'de> for Opcode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let opcode_str = String::deserialize(deserializer)?;
        match Opcode::from_str(&opcode_str) {
            Some(opcode) => Ok(opcode),
            None => Err(serde::de::Error::custom(format!(
                "Invalid opcode value: {}",
                opcode_str
            ))),
        }
    }
}

impl Opcode {
    fn instruction_type(&self) -> InstructionType {
        match self {
            Opcode::ADD
            | Opcode::SUB
            | Opcode::SLL
            | Opcode::SLT
            | Opcode::SLTU
            | Opcode::XOR
            | Opcode::SRL
            | Opcode::SRA
            | Opcode::OR
            | Opcode::AND
            | Opcode::MUL
            | Opcode::MULH
            | Opcode::MULHU
            | Opcode::MULHSU
            | Opcode::DIV
            | Opcode::DIVU
            | Opcode::REM
            | Opcode::REMU
            | Opcode::SUBW => InstructionType::RType,
            Opcode::BEQ | Opcode::BNE | Opcode::BGE | Opcode::BGEU | Opcode::BLT | Opcode::BLTU => {
                InstructionType::BType
            }
            Opcode::JAL | Opcode::LB | Opcode::LH | Opcode::LW | Opcode::LBU | Opcode::LHU => {
                InstructionType::JType
            }
            Opcode::JALR
            | Opcode::ADDI
            | Opcode::SLTI
            | Opcode::SLTIU
            | Opcode::XORI
            | Opcode::ORI
            | Opcode::ANDI
            | Opcode::SLLI
            | Opcode::SRLI
            | Opcode::SRAI => InstructionType::IType,
            Opcode::LUI | Opcode::AUIPC => InstructionType::UType,
            Opcode::SB | Opcode::SH | Opcode::SW => InstructionType::SType,

            Opcode::FENCE => InstructionType::NOTYPE,
            Opcode::ECALL | Opcode::EBREAK => InstructionType::NOTYPE,
            Opcode::UNIMP => InstructionType::NOTYPE,
        }
    }
}

impl From<Opcode> for InstructionType {
    fn from(opcode: Opcode) -> Self {
        opcode.instruction_type()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum InstructionType {
    RType,
    IType,
    SType,
    BType,
    UType,
    JType,
    NOTYPE,
}

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

impl Instruction {
    pub fn get_instruction_length(&self) -> u64 {
        4
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Step {
    pub global_clk: u64,
    pub pc: u64,
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
                "instruction": {
                        "opcode": "DIVU",
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
                    instruction: Instruction {
                        opcode: Opcode::DIVU,
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
