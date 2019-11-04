use crate::{ByteInstruction};
use crate::utils::bits::{bitset, matches};
use super::{ItPos, InstructionContext};
use super::opcode::{Opcode};
use super::tag;

type Context = InstructionContext;

pub fn decode_thumb_wide(word: u32, c: InstructionContext) -> ByteInstruction {
    // A5.3
    assert!(matches(word, 29, 0b111, 0b111));
    let op2 = (word >> 20) & 0b111_1111;
    return match (word >> 27) & 0b11 {
        0b01 => {
            if bitset(op2, 6) {
                id_coprocessor_instr(word, c)
            } else if bitset(op2, 5) {
                id_data_processing_shifted_register(word, c)
            } else if bitset(op2, 2) {
                id_ldr_str_dual(word, c)
            } else {
                id_ldr_str_multiple(word, c)
            }
        }
        0b10 => {
            if bitset(word, 15) {
                id_branch_and_misc(word, c)
            } else if bitset(op2, 5) {
                id_data_proc_plain_binary_immediate(word, c)
            } else {
                id_data_proc_modified_immediate(word, c)
            }
        }
        0b11 => {
            if bitset(op2, 6) {
                id_coprocessor_instr(word, c)
            } else if bitset(op2, 5) {
                if bitset(op2, 4) {
                    if bitset(op2, 3) {
                        id_long_multiply_div(word, c)
                    } else {
                        id_multiply_diff(word, c)
                    }
                } else {
                    id_data_proc_register(word, c)
                }
            } else if (op2 & 0b1110001) == 0 {
                id_store_single(word, c)
            } else {
                match op2 & 0b111 {
                    0b001 => id_load_byte(word, c),
                    0b011 => id_load_half_word(word, c),
                    0b101 => id_load_word(word, c),
                    _ => tag::get_undefined_wide(c, word),
                }
            }
        }
        _ => unreachable!(), // 0b00 would be a narrow instruction
    };
}

fn id_coprocessor_instr(word: u32, c: Context) -> ByteInstruction {
    panic!();
}

fn id_data_processing_shifted_register(word: u32, c: Context) -> ByteInstruction {
    panic!();
}

fn id_ldr_str_dual(word: u32, c: Context) -> ByteInstruction {
    return tag::get_undefined_wide(c, word);
}

fn id_ldr_str_multiple(word: u32, c: Context) -> ByteInstruction {
    return tag::get_undefined_wide(c, word);
}

fn id_branch_and_misc(word: u32, c: Context) -> ByteInstruction {
    assert!(matches(word, 15, 0b111_11_0000000_0000_1, 0b111_10_0000000_0000_1));
    let op1 = (word >> 12) & 0b111;

    if bitset(word, 12) {
        return if bitset(word, 14) {
            let mut mid = (word & (1 << 13)) << 9 | (word & (1 << 11)) << 10;
            if !bitset(word, 26) {
                mid = !mid & (0b11 << 21);
            }
            let imm24 = word & 0x7FF | (word & (0x3FF << 16)) >> 5 | (word & (1 << 26)) >> 3 | mid;
            let instr = tag::get_wide(Opcode::Bl, c, 0, imm24);
            if c.it_pos == ItPos::Within {
                tag::as_unpred_it_w(instr)
            } else {
                instr
            }
        } else {
            tag::get_undefined_wide(c, word)
        };
    }

    return tag::get_undefined_wide(c, word);
}

fn id_data_proc_plain_binary_immediate(word: u32, c: Context) -> ByteInstruction {
    return tag::get_undefined_wide(c, word);
}

fn thumb_expand_imm_c_alt(word: u32) -> (u32, u32) {
    let full;
    let mut spill = 0;
    let base = word & 0xFF;
    if !bitset(word, 26) && !bitset(word, 14) {
        full = match (word >> 12) & 0b11 {
            0b00 => base,
            0b01 => base | base << 16,
            0b10 => (base | base << 16) << 8,
            0b11 => base | base << 8 | base << 16 | base << 24,
            _ => unreachable!(),
        };
    } else {
        let encoded_shift = (word & (1 << 26)) >> 21 | (word & (0b111 << 12)) >> 11 | (word & (1 << 7)) >> 7;
        full = base << (0x20 - encoded_shift);
        spill |= 0b1000;
        if bitset(full, 31) {
            spill |= 0b0100;
        }
    };
    spill |= full >> 30;
    return (spill, full & !(0b11 << 30));
}

fn id_data_proc_modified_immediate(word: u32, c: Context) -> ByteInstruction {
    // A5.3.1
    assert!(matches(word, 15, 0b111_11_0_1_00000_0000_1, 0b111_10_0_0_00000_0000_0));
    let rn = (word >> 16) & 0xF;
    let rd = (word >> 8) & 0xF;
    let setflags = bitset(word, 20);
    let (spill, extra) = thumb_expand_imm_c_alt(word);
    let pro_spill = (word & (0xF << 8)) >> 4 | (word & (0xF << 16)) >> 8 | (word & (1 << 20)) >> 7 | spill;
    return match (word >> 21) & 0b1111 {
        0b0000 => {
            let base = if rd == 15 {
                tag::get_wide(Opcode::TstImm, c, (word & (0xF << 16)) >> 12 | spill, extra) // A7.7.188
            } else {
                tag::get_wide(Opcode::AndImm, c, pro_spill, extra) // A7.7.8 T1
            };
            return if rn == 13 || rn == 15 || rd == 13 || (rd == 15 && !setflags) {
                tag::as_unpred_w(base)
            } else {
                base
            }
        }
        0b0001 => {
            let base = tag::get_wide(Opcode::BicImm, c, pro_spill, extra); // A7.7.15 T1
            if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                tag::as_unpred_w(base)
            } else {
                base
            }
        }
        0b0010 => {
            let base = if rn == 15 {
                tag::get_wide(Opcode::MovImm, c, (word & (1 << 20)) >> 12 | (word & (0xF << 8)) >> 4 | spill, extra) // A7.7.76 T2
            } else {
                tag::get_wide(Opcode::OrrImm, c, pro_spill, extra) // A7.7.91 T1
            };
            if rd == 13 || rd == 15 || rn == 13 {
                tag::as_unpred_w(base)
            } else {
                base
            }
        }
        0b0011 => {
            let base = if rn == 15 {
                tag::get_wide(Opcode::MvnImm, c, (word & (1 << 20)) >> 12 | (word & (0xF << 8)) >> 4 | spill, extra) // A7.7.85 T1
            } else {
                tag::get_wide(Opcode::OrnImm, c, pro_spill, extra) // A7.7.89 T1
            };
            if rd == 13 || rd == 15 || rn == 13 {
                tag::as_unpred_w(base)
            } else {
                base
            }
        }
        0b0100 => {
            let base = if rd == 15 {
                tag::get_wide(Opcode::TeqImm, c, (word & (0xF << 16)) >> 12 | spill, extra) // A7.7.186 T1
            } else {
                tag::get_wide(Opcode::EorImm, c, pro_spill, extra) // A7.7.35 T1
            };
            if rn == 13 || rn == 15 || rd == 13 || (rd == 15 && !setflags) {
                tag::as_unpred_w(base)
            } else {
                base
            }
        }
        0b1000 => {
            let base = if rd == 15 {
                tag::get_wide(Opcode::CmnImm, c, (word & (0xF << 16)) >> 12 | spill, extra) // A7.7.25 T1
            } else {
                tag::get_wide(Opcode::AddImm, c, pro_spill, extra) // A7.7.3 T3
            };
            if rn == 15 || rd == 13 || (rd == 15 && !setflags) {
                tag::as_unpred_w(base)
            } else {
                base
            }
        }
        0b1010 => {
            let base = tag::get_wide(Opcode::AdcImm, c, pro_spill, extra); // A7.7.1 T1
            if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                tag::as_unpred_w(base)
            } else {
                base
            }
        }
        0b1011 => {
            let base = tag::get_wide(Opcode::SbcImm, c, pro_spill, extra); // A7.7.124 T1
            if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                tag::as_unpred_w(base)
            } else {
                base
            }
        }
        0b1101 => {
            let base = if rd == 15 {
                tag::get_wide(Opcode::CmpImm, c, (word & (0xF << 16)) >> 12 | spill, extra) // A7.7.27 T2
            } else {
                tag::get_wide(Opcode::SubImm, c, pro_spill, extra) // A7.7.174 T3
            };
            if rn == 15 || rd == 13 || (rd == 15 && !setflags) {
                tag::as_unpred_w(base)
            } else {
                base
            }
        }
        0b1110 => {
            let base = tag::get_wide(Opcode::RsbImm, c, pro_spill, extra); // A7.7.119 T2
            if rd == 13 || rd == 15 || rn == 13 || rn == 15 {
                tag::as_unpred_w(base)
            } else {
                base
            }
        }
        _ => tag::get_undefined_wide(c, word),
    };
}

fn id_long_multiply_div(word: u32, c: Context) -> ByteInstruction {
    return tag::get_undefined_wide(c, word);
}

fn id_multiply_diff(word: u32, c: Context) -> ByteInstruction {
    return tag::get_undefined_wide(c, word);
}

fn id_data_proc_register(word: u32, c: Context) -> ByteInstruction {
    return tag::get_undefined_wide(c, word);
}

fn id_store_single(word: u32, c: Context) -> ByteInstruction {
    assert!(matches(word, 20, 0b111_1111_1_000_1, 0b111_1100_0_000_0));

    let op2 = bitset(word, 11);

    let rn = ((word >> 16) & 0b1111) as u8;
    let rt = ((word >> 12) & 0b1111) as u8;

    return match (word >> 21) & 0b111 {
        0b110 => tag::get_wide(Opcode::StrImm, c, (word >> 12) & 0xFF, word & 0xFFF | 1 << 14), // A7.7.161 T3
        0b010 if op2 => {
            // p381 T4
            let imm13 = if bitset(word, 9) {
                word & 0xFF
            } else {
                (!(word & 0xFF) + 1) & 0x1FFF
            };
            println!("{:#010X}", imm13 | (word & (1 << 10)) << 4 | (word & (1 << 8)) << 5);
            tag::get_wide(Opcode::StrImm, c, (word >> 12) & 0xFF, imm13 | (word & (1 << 10)) << 4 | (word & (1 << 8)) << 5)
        }
        _ => tag::get_undefined_wide(c, word),
    };
}

fn id_load_byte(word: u32, c: Context) -> ByteInstruction {
    return tag::get_undefined_wide(c, word);
}

fn id_load_half_word(word: u32, c: Context) -> ByteInstruction {
    return tag::get_undefined_wide(c, word);
}

fn id_load_word(word: u32, c: Context) -> ByteInstruction {
    let op1 = (word >> 23) & 0b11;
    let op2 = (word >> 6) & 0x3F;
    let rn = (word >> 16) & 0xF;
    let rt = (word >> 12) & 0xF;
    let offset = word & 0xFFF;

    if op1 > 1 {
        return tag::get_undefined_wide(c, word);
    }

    if rn == 15 {
        let imm13 = if bitset(word, 23) {
            word & 0xFFF
        } else {
            (!(word & 0xFFF) + 1) & 0x1FFF
        };
        let instr = tag::get_wide(Opcode::LdrLit, c, rt, imm13);
        return if c.it_pos == ItPos::Within {
            tag::as_unpred_it_w(instr)
        } else {
            instr
        }
    }

    // if op1 == 0b01 {
    //     // p243 T3
    //     return Instruction::LdrImm {
    //         rn,
    //         rt,
    //         offset,
    //         index: true,
    //         wback: false,
    //     };
    // }
    //
    // if op2 == 0 {
    //     let rm = (word & 0b1111) as u8;
    //     let shift_n = ((word >> 4) & 0b11) as u32;
    //     return Instruction::LdrReg { rn, rt, rm, shift: Shift {shift_t: ShiftType::LSL, shift_n}};
    // }
    //
    // let op3 = op2 >> 2;
    // let mut offset8 = (word & 0b1111_1111) as i32;
    //
    // if op3 == 0b1100 || (op3 & 0b1001) == 0b1001 {
    //     // p243 T4
    //     if !bitset(word, 9) {
    //         offset8 = -offset8;
    //     }
    //     return Instruction::LdrImm {
    //         rn,
    //         rt,
    //         offset: offset8,
    //         index: bitset(word, 10),
    //         wback: bitset(word, 8),
    //     };
    // }
    //
    // if op3 == 0b1110 {
    //     return Instruction::Ldrt {
    //         rn,
    //         rt,
    //         offset: offset8,
    //     };
    // }

    return tag::get_undefined_wide(c, word);
}
