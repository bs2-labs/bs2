use crate::execution_table::Entries;
use core::marker::PhantomData;

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
    ) -> Self {
        let lhs_col = cs.advice_column();
        let rhs_col = cs.advice_column();
        cs.enable_equality(lhs_col);
        cs.enable_equality(rhs_col);
        // todo: add selector for every gate
        let s_add = cs.selector();
        let s_sub = cs.selector();
        let s_subw = cs.selector();
        let s_sll = cs.selector();
        let s_srl = cs.selector();
        let s_sra = cs.selector();
        let s_slt = cs.selector();
        let s_sltu = cs.selector();
        let s_xor = cs.selector();
        let s_or = cs.selector();
        let s_and = cs.selector();
        let s_mul = cs.selector();
        let s_mulh = cs.selector();
        let s_mulhu = cs.selector();
        let s_mulhsu = cs.selector();
        let s_div = cs.selector();
        let s_divu = cs.selector();
        let s_rem = cs.selector();
        let s_remu = cs.selector();
        let s_addw = cs.selector();
        let s_sllw = cs.selector();
        let s_srlw = cs.selector();
        let s_sraw = cs.selector();

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

    pub fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        // step: &Step,
        _entries: &Entries,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "rtype",
            |mut region| {
                // rs1_value
                // let col0 = step.registers[step.instruction.op_b as usize];
                // // rs2_value
                // let col1 = step.registers[step.instruction.op_c as usize];
                // // rd_value at next row
                // let col0_next = step.registers[step.instruction.op_a as usize];
                // let col0 = self.col0;
                // let col1 = self.col1;
                let rtype: RType = RType::ADD;

                match rtype {
                    RType::ADD => {
                        let selector = self.s_add;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(120)),
                        )?;
                    }
                    RType::SUB => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::SUBW => {
                        let selector = self.s_subw;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::SLL => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::SRL => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::SRA => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::SLT => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::SLTU => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::XOR => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::OR => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::AND => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::MUL => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::MULH => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::MULHU => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::MULHSU => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::DIV => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::DIVU => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::REM => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::REMU => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::ADDW => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::SLLW => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::SRLW => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                    RType::SRAW => {
                        let selector = self.s_sub;
                        selector.enable(&mut region, 0)?;

                        region.assign_advice(
                            || "lhs",
                            self.lhs_col,
                            0,
                            || Value::known(F::from(100)),
                        )?;

                        region.assign_advice(
                            || "rhs",
                            self.rhs_col,
                            0,
                            || Value::known(F::from(20)),
                        )?;

                        region.assign_advice(
                            || "output",
                            self.lhs_col,
                            1,
                            || Value::known(F::from(80)),
                        )?;
                    }
                };
                // todo: assign selector value to some position

                Ok(())
            },
        )
    }
}
