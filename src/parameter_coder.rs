pub trait BooleanCoder {
    fn from_raw(&self, raw: u8) -> bool;
    fn to_raw(&self, value: bool) -> u8;
}

pub trait I8Coder {
    fn from_raw(&self, raw: u8) -> i8;
    fn to_raw(&self, value: i8) -> u8;
}

pub trait U8Coder {
    fn from_raw(&self, raw: u8) -> u8;
    fn to_raw(&self, value: u8) -> u8;
}

pub trait I16Coder {
    fn from_raw(&self, raw: [u8; 2]) -> i16;
    fn to_raw(&self, value: i16) -> [u8; 2];
}

pub trait U16Coder {
    fn from_raw(&self, raw: [u8; 2]) -> u16;
    fn to_raw(&self, value: u16) -> [u8; 2];
}

pub trait I32Coder {
    fn from_raw(&self, raw: [u8; 4]) -> i32;
    fn to_raw(&self, value: i32) -> [u8; 4];
}

pub trait U32Coder {
    fn from_raw(&self, raw: [u8; 4]) -> u32;
    fn to_raw(&self, value: u32) -> [u8; 4];
}

pub trait F32Coder {
    fn from_raw(&self, raw: [u8; 4]) -> f32;
    fn to_raw(&self, value: f32) -> [u8; 4];
}

pub trait F64Coder {
    fn from_raw(&self, raw: [u8; 8]) -> f64;
    fn to_raw(&self, value: f64) -> [u8; 8];
}

pub trait I64Coder {
    fn from_raw(&self, raw: [u8; 8]) -> i64;
    fn to_raw(&self, value: i64) -> [u8; 8];
}

pub trait U64Coder {
    fn from_raw(&self, raw: [u8; 8]) -> u64;
    fn to_raw(&self, value: u64) -> [u8; 8];
}

pub struct DefaultBooleanCoder;
pub struct DefaultI8Coder;
pub struct DefaultU8Coder;
pub struct DefaultI16Coder;
pub struct DefaultU16Coder;
pub struct DefaultU32Coder;
pub struct DefaultI32Coder;
pub struct DefaultF32Coder;
pub struct DefaultI64Coder;
pub struct DefaultU64Coder;
pub struct DefaultF64Coder;

impl BooleanCoder for DefaultBooleanCoder {
    fn from_raw(&self, raw: u8) -> bool {
        if raw == 0x0 {
            false
        } else {
            true
        }
    }

    fn to_raw(&self, value: bool) -> u8 {
        if value {
            0x1
        } else {
            0x0
        }
    }
}

impl I8Coder for DefaultI8Coder {
    fn from_raw(&self, raw: u8) -> i8 {
        raw as i8
    }

    fn to_raw(&self, value: i8) -> u8 {
        value as u8
    }
}

impl U8Coder for DefaultU8Coder {
    fn from_raw(&self, raw: u8) -> u8 {
        raw
    }

    fn to_raw(&self, value: u8) -> u8 {
        value
    }
}

impl I16Coder for DefaultI16Coder {
    fn from_raw(&self, raw: [u8; 2]) -> i16 {
        i16::from_le_bytes(raw)
    }

    fn to_raw(&self, value: i16) -> [u8; 2] {
        value.to_le_bytes()
    }
}

impl U16Coder for DefaultU16Coder {
    fn from_raw(&self, raw: [u8; 2]) -> u16 {
        u16::from_le_bytes(raw)
    }

    fn to_raw(&self, value: u16) -> [u8; 2] {
        value.to_le_bytes()
    }
}

impl I32Coder for DefaultI32Coder {
    fn from_raw(&self, raw: [u8; 4]) -> i32 {
        i32::from_le_bytes(raw)
    }

    fn to_raw(&self, value: i32) -> [u8; 4] {
        value.to_le_bytes()
    }
}

impl U32Coder for DefaultU32Coder {
    fn from_raw(&self, raw: [u8; 4]) -> u32 {
        u32::from_le_bytes(raw)
    }

    fn to_raw(&self, value: u32) -> [u8; 4] {
        value.to_le_bytes()
    }
}

impl F32Coder for DefaultF32Coder {
    fn from_raw(&self, raw: [u8; 4]) -> f32 {
        f32::from_le_bytes(raw)
    }

    fn to_raw(&self, value: f32) -> [u8; 4] {
        value.to_le_bytes()
    }
}

impl I64Coder for DefaultI64Coder {
    fn from_raw(&self, raw: [u8; 8]) -> i64 {
        i64::from_le_bytes(raw)
    }

    fn to_raw(&self, value: i64) -> [u8; 8] {
        value.to_le_bytes()
    }
}

impl U64Coder for DefaultU64Coder {
    fn from_raw(&self, raw: [u8; 8]) -> u64 {
        u64::from_le_bytes(raw)
    }

    fn to_raw(&self, value: u64) -> [u8; 8] {
        value.to_le_bytes()
    }
}

impl F64Coder for DefaultF64Coder {
    fn from_raw(&self, raw: [u8; 8]) -> f64 {
        f64::from_le_bytes(raw)
    }

    fn to_raw(&self, value: f64) -> [u8; 8] {
        value.to_le_bytes()
    }
}
