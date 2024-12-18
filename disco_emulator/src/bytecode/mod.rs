use crate::ByteInstruction;
use crate::utils::bits::is_wide_thumb;
use std::fmt;

mod narrow;
use narrow::decode_thumb_narrow;

mod wide;
use wide::decode_thumb_wide;

pub mod opcode;
pub mod tag;

pub struct InstructionCache {
    cache: Box<[u32]>,
}

impl fmt::Debug for InstructionCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "<instruction_cache>");
    }
}

impl InstructionCache {
    pub fn new() -> InstructionCache {
        return InstructionCache {
            cache: vec![tag::reset_value(); 1024 * 1024].into_boxed_slice(),
        };
    }

    pub fn get_cached(&self, address: u32) -> Result<(u32, u32), String> {
        if address as usize <= self.cache.len() - 2 {
            let base = address as usize;
            return Ok((self.cache[base], self.cache[base + 1]));
        }
        return Err(String::from("Out of bounds access"));
    }

    pub fn write_cache_narrow(&mut self, address: u32, value: ByteInstruction) {
        assert!(address as usize <= self.cache.len() - 1);
        self.cache[address as usize] = value.0;
    }

    pub fn write_cache_wide(&mut self, address: u32, value: ByteInstruction) {
        assert!(address as usize <= self.cache.len() - 1);
        self.cache[address as usize] = value.0;
        self.cache[(address + 1) as usize] = value.1;
    }
}

/**
 * The position of the instruction within an IT block
 */
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ItPos {
    None,
    Within,
    Last,
}

/**
 * Context relevant to deciding if an instruction is unpredictable
 */
#[derive(Debug, Copy, Clone)]
pub struct InstructionContext {
    pc: u32,
    it_pos: ItPos,
}

impl InstructionContext {
    pub fn new(pc: u32, it_pos: ItPos) -> InstructionContext {
        return InstructionContext {
            pc,
            it_pos,
        }
    }
}

pub fn decode_thumb(thumb: u32, context: InstructionContext) -> (ByteInstruction, bool) {
    return if is_wide_thumb(thumb) {
        (decode_thumb_wide(thumb, context), true)
    } else {
        ((decode_thumb_narrow((thumb >> 16) as u16, context), tag::reset_value()), false)
    };
}
