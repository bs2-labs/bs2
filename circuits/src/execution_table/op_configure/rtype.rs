use core::marker::PhantomData;
use runtime::trace::Opcode;
use entry_builder::op_step::OpStep;

use halo2_proofs::arithmetic::FieldExt;
use halo2_proofs::circuit::{AssignedCell, Layouter, Value};
use halo2_proofs::plonk::*;
use halo2_proofs::poly::Rotation;
use runtime::trace::{BType, IType, InstructionType, JType, NoType, RType, SType, Step, UType};

#[derive(Debug, Clone)]
pub struct ACell<F: FieldExt>(pub AssignedCell<F, F>);

#[derive(Clone)]
pub struct RTypeGadget<F> {
    pub lhs_col: Column<Advice>,
    pub rhs_col: Column<Advice>,
    s_add: Selector,
    s_sub: Selector,
    s_subw: Selector,
    s_sll: Selector,
    s_srl: Selector,
    s_sra: Selector,
    s_slt: Selector,
    s_sltu: Selector,
    s_xor: Selector,
    s_or: Selector,
    s_and: Selector,
    s_mul: Selector,
    s_mulh: Selector,
    s_mulhu: Selector,
    s_mulhsu: Selector,
    s_div: Selector,
    s_divu: Selector,
    s_rem: Selector,
    s_remu: Selector,
    s_addw: Selector,
    s_sllw: Selector,
    s_srlw: Selector,
    s_sraw: Selector,
    _maker: PhantomData<F>,
}

impl<F: FieldExt> RTypeGadget<F> {
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        lhs_col: Column<Advice>,
        rhs_col: Column<Advice>,
        s_add: Selector,
        s_sub: Selector,
        s_subw: Selector,
        s_sll: Selector,
        s_srl: Selector,
        s_sra: Selector,
        s_slt: Selector,
        s_sltu: Selector,
        s_xor: Selector,
        s_or: Selector,
        s_and: Selector,
        s_mul: Selector,
        s_mulh: Selector,
        s_mulhu: Selector,
        s_mulhsu: Selector,
        s_div: Selector,
        s_divu: Selector,
        s_rem: Selector,
        s_remu: Selector,
        s_addw: Selector,
        s_sllw: Selector,
        s_srlw: Selector,
        s_sraw: Selector,
    ) -> Self {
        // let lhs_col = cs.advice_column();
        // let rhs_col = cs.advice_column();
        cs.enable_equality(lhs_col);
        cs.enable_equality(rhs_col);

        // todo: constrain selector: s1 + s1 + .. + sn = 1

        cs.create_gate("RType::ADD", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_add);
            // let (value, _) = rs1_value.overflowing_sub(rs2_value);
            vec![s * (lhs + rhs - out)]
        });

        cs.create_gate("RType::SUB", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // let (value, _) = rs1_value.overflowing_sub(rs2_value);
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::SUBW", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);

            // RType::SUBW => {
            //     let (value, _) = rs1_value.overflowing_sub(rs2_value);
            //     value.sign_extend(&32)
            // }
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::SLL", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);

            // RType::SLL => {
            //     let shift_value = rs2_value.clone() & SHIFT_MASK;
            //     rs1_value.clone() << shift_value
            // }

            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::SRL", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::SRL => {
            //     let shift_value = rs2_value.clone() & SHIFT_MASK;
            //     rs1_value.clone() >> shift_value
            // }
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::SRA", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::SRA => rs1_value.clone() >> rs2_value,
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::SLT", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);

            // RType::SLT => (rs1_value < rs2_value).into(),
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::SLTU", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::SLTU => (rs1_value < rs2_value).into(),
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::XOR", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::XOR => rs1_value ^ rs2_value,
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::OR", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);

            // RType::OR => rs1_value | rs2_value,
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::AND", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::AND => rs1_value & rs2_value,
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::MUL", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::MUL => rs1_value.overflowing_mul(rs2_value).0,

            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::MULH", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::MULH => {
            //     let a = i128::from(rs1_value as i64);
            //     let b = i128::from(rs2_value as i64);
            //     let (value, _) = a.overflowing_mul(b);
            //     (value >> 64) as u64
            // }

            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::MULHU", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::MULHU => {
            //     let a = u128::from(rs1_value);
            //     let b = u128::from(rs2_value);
            //     let (value, _) = a.overflowing_mul(b);
            //     (value >> 64) as u64
            // }

            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::MULHSU", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::MULHSU => {
            //     let a = i128::from(rs1_value as i64);
            //     let b = i128::from(rs2_value);
            //     let (value, _) = a.overflowing_mul(b);
            //     (value >> 64) as u64
            // }

            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::DIV", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);

            // RType::DIV => {
            //     // rs1_value.overflowing_div_signed(rs2_value);
            //     if rs1_value == 0 {
            //         u64::max_value()
            //     } else {
            //         rs1_value.overflowing_div(rs2_value).0
            //     }
            // }

            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::DIVU", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::DIVU => {
            //     // rs1_value.overflowing_div(rs2_value);
            //     if rs2_value == 0 {
            //         (-1i64) as u64
            //     } else {
            //         let (v, o) = (rs1_value as i64).overflowing_div(rs2_value as i64);
            //         if o {
            //             // -2**(L-1) implemented using (-1) << (L - 1)
            //             ((-1i64) as u64) << (64 - 1)
            //         } else {
            //             v as u64
            //         }
            //     }
            // }
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::REM", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);

            // RType::REM => {
            //     // rs1_value.overflowing_rem_signed(rs2_value);
            //     if rs2_value == 0 {
            //         rs1_value
            //     } else {
            //         (rs1_value).overflowing_rem(rs2_value).0
            //     }
            // }
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::REMU", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);

            // RType::REMU => {
            //     // rs1_value.overflowing_rem(rs2_value);
            //     if rs2_value == 0 {
            //         rs1_value
            //     } else {
            //         let (v, o) = (rs1_value as i64).overflowing_rem(rs2_value as i64);
            //         if o {
            //             0
            //         } else {
            //             v as u64
            //         }
            //     }
            // }
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::ADDW", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::ADDW => (rs1_value + rs2_value).sign_extend(&32),
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::SLLW", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);

            // RType::SLLW => {
            //     let shift_value = rs2_value.clone() & SHIFT_MASK;
            //     let result = rs1_value.clone() << shift_value;
            //     result.sign_extend(&32)
            // }
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::SRLW", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);

            // RType::SRLW => {
            //     let shift_value = rs2_value.clone() & SHIFT_MASK;
            //     let result = rs1_value.clone() >> shift_value;
            //     result.sign_extend(&32)
            // }
            vec![s * (lhs - rhs - out)]
        });

        cs.create_gate("RType::SRAW", |vc| {
            let lhs = vc.query_advice(lhs_col, Rotation::cur());
            let rhs = vc.query_advice(rhs_col, Rotation::cur());
            let out = vc.query_advice(lhs_col, Rotation::next());
            let s = vc.query_selector(s_sub);
            // RType::SRAW => {
            //     let result = rs1_value.clone() >> rs2_value;
            //     result.sign_extend(&32)
            // }
            vec![s * (lhs - rhs - out)]
        });

        Self {
            lhs_col,
            rhs_col,
            s_add,
            s_sub,
            s_subw,
            s_sll,
            s_srl,
            s_sra,
            s_slt,
            s_sltu,
            s_xor,
            s_or,
            s_and,
            s_mul,
            s_mulh,
            s_mulhu,
            s_mulhsu,
            s_div,
            s_divu,
            s_rem,
            s_remu,
            s_addw,
            s_sllw,
            s_srlw,
            s_sraw,
            _maker: PhantomData::default(),
        }
    }

    pub fn assign(&self, layouter: &mut impl Layouter<F>, step: &OpStep) -> Result<(), Error> {
        layouter.assign_region(
            || "rtype",
            |mut region| {
                let rs1 = step.instruction.op_b;
                let rs2 = step.instruction.op_c;
                let rd = step.instruction.op_a;
                let rs1_value = step.register_indexes.unwrap().read(rs1).unwrap();
                let rs2_value = step.register_indexes.unwrap().read(rs2).unwrap();
                let rd_value = step.register_indexes.unwrap().read(rd).unwrap();

                region.assign_advice(
                    || "lhs",
                    self.lhs_col,
                    0,
                    || Value::known(F::from(rs1_value)),
                )?;

                region.assign_advice(
                    || "rhs",
                    self.rhs_col,
                    0,
                    || Value::known(F::from(rs2_value)),
                )?;

                region.assign_advice(
                    || "output",
                    self.lhs_col,
                    1,
                    || Value::known(F::from(rd_value)),
                )?;

                match step.instruction.opcode.into() {
                    Opcode::ADD => self.s_add.enable(&mut region, 0)?,
                    Opcode::SUB => self.s_sub.enable(&mut region, 0)?,
                    Opcode::SUBW => self.s_subw.enable(&mut region, 0)?,
                    Opcode::SLL => self.s_sll.enable(&mut region, 0)?,
                    Opcode::SRL => self.s_srl.enable(&mut region, 0)?,
                    Opcode::SRA => self.s_sra.enable(&mut region, 0)?,
                    Opcode::SLT => self.s_slt.enable(&mut region, 0)?,
                    Opcode::SLTU => self.s_sltu.enable(&mut region, 0)?,
                    Opcode::XOR => self.s_xor.enable(&mut region, 0)?,
                    Opcode::OR => self.s_or.enable(&mut region, 0)?,
                    Opcode::AND => self.s_and.enable(&mut region, 0)?,
                    Opcode::MUL => self.s_mul.enable(&mut region, 0)?,
                    Opcode::MULH => self.s_mulh.enable(&mut region, 0)?,
                    Opcode::MULHU => self.s_mulhu.enable(&mut region, 0)?,
                    Opcode::MULHSU => self.s_mulhsu.enable(&mut region, 0)?,
                    Opcode::DIV => self.s_div.enable(&mut region, 0)?,
                    Opcode::DIVU => self.s_divu.enable(&mut region, 0)?,
                    Opcode::REM => self.s_rem.enable(&mut region, 0)?,
                    Opcode::REMU => self.s_remu.enable(&mut region, 0)?,
                    Opcode::ADDW => self.s_addw.enable(&mut region, 0)?,
                    Opcode::SLLW => self.s_sllw.enable(&mut region, 0)?,
                    Opcode::SRLW => self.s_srlw.enable(&mut region, 0)?,
                    Opcode::SRAW => self.s_sraw.enable(&mut region, 0)?,
                    _ => panic!("Not implemented {:?}", step.instruction.opcode),
                };
                Ok(())
            },
        )
    }
}
