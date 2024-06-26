use alloc::{format, string::String, vec::Vec};
use core::panic;
use serde::{Deserialize, Serialize};

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

    // Riscv64 instructions
    LWU = 49,
    LD = 50,

    SD = 51,

    ADDIW = 52,
    SLLIW = 53,
    SRLIW = 54,
    SRAIW = 55,
    ADDW = 56,
    SUBW = 57,
    SLLW = 58,
    SRLW = 59,
    SRAW = 60,

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
            49 => Opcode::LWU,
            50 => Opcode::LD,
            51 => Opcode::SD,
            52 => Opcode::ADDIW,
            53 => Opcode::SLLIW,
            54 => Opcode::SRLIW,
            55 => Opcode::SRAIW,
            56 => Opcode::ADDW,
            57 => Opcode::SUBW,
            58 => Opcode::SLLW,
            59 => Opcode::SRLW,
            60 => Opcode::SRAW,

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
            "LWU" => Some(Opcode::LWU),
            "LD" => Some(Opcode::LD),
            "SD" => Some(Opcode::SD),
            "ADDIW" => Some(Opcode::ADDIW),
            "SLLIW" => Some(Opcode::SLLIW),
            "SRLIW" => Some(Opcode::SRLIW),
            "SRAIW" => Some(Opcode::SRAIW),
            "ADDW" => Some(Opcode::ADDW),
            "SUBW" => Some(Opcode::SUBW),
            "SLLW" => Some(Opcode::SLLW),
            "SRLW" => Some(Opcode::SRLW),
            "SRAW" => Some(Opcode::SRAW),

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
            Opcode::LWU => "LWU",
            Opcode::LD => "LD",
            Opcode::SD => "SD",
            Opcode::ADDIW => "ADDIW",
            Opcode::SLLIW => "SLLIW",
            Opcode::SRLIW => "SRLIW",
            Opcode::SRAIW => "SRAIW",
            Opcode::ADDW => "ADDW",
            Opcode::SUBW => "SUBW",
            Opcode::SLLW => "SLLW",
            Opcode::SRLW => "SRLW",
            Opcode::SRAW => "SRAW",
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
        if let Ok(r) = RType::try_from(*self) {
            return InstructionType::RType(r);
        }
        if let Ok(r) = BType::try_from(*self) {
            return InstructionType::BType(r);
        }
        if let Ok(r) = IType::try_from(*self) {
            return InstructionType::IType(r);
        }
        if let Ok(r) = UType::try_from(*self) {
            return InstructionType::UType(r);
        }
        if let Ok(r) = SType::try_from(*self) {
            return InstructionType::SType(r);
        }
        if let Ok(r) = JType::try_from(*self) {
            return InstructionType::JType(r);
        }
        if let Ok(r) = NoType::try_from(*self) {
            return InstructionType::NoType(r);
        }
        unreachable!("Pattern matching should be exhaustive")
    }
}

impl From<Opcode> for InstructionType {
    fn from(opcode: Opcode) -> Self {
        opcode.instruction_type()
    }
}

#[macro_export]
macro_rules! sub_enum {
    ($sub_enum_name:ident of $super_enum_name:ty {
        $($variant:ident),* $(,)?
    }) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
        pub enum $sub_enum_name {
            $($variant,)*
        }

        impl From<$sub_enum_name> for $super_enum_name {
            fn from(val: $sub_enum_name) -> $super_enum_name {
                match val {
                    $(<$sub_enum_name>::$variant => <$super_enum_name>::$variant,)*
                }
            }
        }

        impl TryFrom<$super_enum_name> for $sub_enum_name {
            type Error = ();
            fn try_from(val: $super_enum_name) -> Result<Self, Self::Error> {
                match val {
                    $(<$super_enum_name>::$variant => Ok(Self::$variant),)*
                    _ => Err(())
                }
            }
        }
    }
}

sub_enum!(RType of Opcode {
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    MUL,
    MULH,
    MULHU,
    MULHSU,
    DIV,
    DIVU,
    REM,
    REMU,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,
});

sub_enum!(IType of Opcode {
    JALR,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADDIW,
    SLLIW,
    SRLIW,
    SRAIW,

    LB,
    LH,
    LW,
    LD,
    LBU,
    LHU,
    LWU
});

sub_enum!(BType of Opcode {
    BEQ,
    BNE,
    BGE,
    BGEU,
    BLT,
    BLTU,
});

sub_enum!(SType of Opcode {
    SB,
    SH,
    SW,
    SD,
});

sub_enum!(JType of Opcode {
    JAL,
});

sub_enum!(UType of Opcode {
    LUI,
    AUIPC,
});

sub_enum!(NoType of Opcode {
    FENCE,
    ECALL,
    EBREAK,
    UNIMP,
});

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum InstructionType {
    RType(RType),
    IType(IType),
    SType(SType),
    BType(BType),
    UType(UType),
    JType(JType),
    NoType(NoType),
}

/// An instruction specifies an operation to execute and the operands.
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub length: u64,
    pub op_a: u64,
    pub op_b: u64,
    pub op_c: u64,
    pub imm_b: bool,
    pub imm_c: bool,
}

impl Instruction {
    pub fn get_instruction_length(&self) -> u64 {
        self.length
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
    pub cycles: u64,
    pub return_value: u8,
    pub steps: Vec<Step>,
}
