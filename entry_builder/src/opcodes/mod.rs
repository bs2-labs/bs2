use runtime::trace::Step;
use runtime::trace::Opcode;
use std::fmt::Debug;
use core::fmt::Error;
use crate::rw_container::RwContainer;

mod rtype;

use rtype::RType;

pub trait OpcodeFn: Debug {
    fn gen_associated_ops(
        rw_contaienr: &mut RwContainer,
        step: &Step,
    ) -> Result<(), Error>;
}

#[derive(Debug, Copy, Clone)]
struct Dummy;

impl OpcodeFn for Dummy {
    fn gen_associated_ops(
        rw_contaienr: &mut RwContainer,
        step: &Step,
    ) -> Result<(), Error> {
        Ok(())
    }
}

type FnGenAssociatedOps = fn(
    rw_contaienr: &mut RwContainer,
    steps: &Step,
) -> Result<(), Error>;

// TODO:
// R-type : add rd, rs1, rs2
// I-type : addi rd, rs1, imm
// S-type : sw rs2, offset(rs1)
// B-type : beq rs1, rs2, offset
// U-type : lui rd, imm
// J-type : jal rd, offset
fn fn_gen_associated_ops(opcode: Opcode) -> FnGenAssociatedOps {
    match opcode {
        // TODO: use ckb opcode 
        35 => RType::gen_associated_ops,
        _ => {
            log::debug!("Using dummy gen_associated_ops for opcode {:?}", opcode);
            Dummy::gen_associated_ops
        }
    }
}

pub fn gen_associated_ops(
    opcode: Opcode,
    rw_contaienr: &mut RwContainer,
    step: &Step,
) -> Result<(), Error> {
    // if no errors, continue as normal
    let fn_gen_associated_ops = fn_gen_associated_ops(opcode);
    fn_gen_associated_ops(rw_contaienr, step)
}
