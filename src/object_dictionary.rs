use crate::{parameter_coder::*, pdo::PdoConfiguration};

#[derive(Clone, Copy)]
pub enum VariableType {
    Array(u8),
    Record(u8),
    Boolean(bool, &'static dyn BooleanCoder),
    Int8(i8, &'static dyn I8Coder),
    UInt8(u8, &'static dyn U8Coder),
    Int16(i16, &'static dyn I16Coder),
    UInt16(u16, &'static dyn U16Coder),
    Int32(i32, &'static dyn I32Coder),
    UInt32(u32, &'static dyn U32Coder),
    Int64(i64, &'static dyn I64Coder),
    UInt64(u64, &'static dyn U64Coder),
    Float32(f32, &'static dyn F32Coder),
    Float64(f64, &'static dyn F64Coder),
    RawBytes(usize),
}

impl VariableType {
    fn raw_size(&self) -> usize {
        match self {
            VariableType::Boolean(_, _)
            | VariableType::Int8(_, _)
            | VariableType::UInt8(_, _)
            | VariableType::Array(_)
            | VariableType::Record(_) => 1,
            VariableType::Int16(_, _) | VariableType::UInt16(_, _) => 2,
            VariableType::Int32(_, _)
            | VariableType::UInt32(_, _)
            | VariableType::Float32(_, _) => 4,
            VariableType::Int64(_, _)
            | VariableType::UInt64(_, _)
            | VariableType::Float64(_, _) => 8,
            VariableType::RawBytes(size) => *size,
        }
    }
}

#[derive(Clone, Copy)]
enum AccessType {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl AccessType {
    fn allows_writing(&self) -> bool {
        match self {
            Self::WriteOnly | Self::ReadWrite => true,
            Self::ReadOnly => false,
        }
    }

    fn allows_reading(&self) -> bool {
        match self {
            Self::ReadOnly | Self::ReadWrite => true,
            Self::WriteOnly => false,
        }
    }
}

#[derive(Clone, Copy)]
pub enum StorageLocation {
    NonVolatile,
    Ram,
}

#[derive(Clone, Copy)]
pub enum PdoMapability {
    All,
    Tpdo,
    Rpdo,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryId {
    index: u16,
    sub_index: u8,
}

#[derive(Clone, Copy)]
pub struct Variable {
    name: &'static str,
    storage_location: StorageLocation,
    data_type: VariableType,
    pdo_mapability: PdoMapability,
    access_type: AccessType,
    id: EntryId,
}

enum FrameId {
    Standard(u16),
    Extended(u32),
}

#[derive(Clone, Copy)]
pub struct CobId {
    guts: u32,
}

impl CobId {
    const DISABLED_ID: u32 = 0x8000_0000;
    pub fn new(enabled: bool, rtr_allowed: bool, frame_id: FrameId) -> Self {
        let raw_frame_id = match frame_id {
            FrameId::Standard(id) => (id & 0x7FF) as u32,
            FrameId::Extended(id) => (id & 0x1FFF_FFFF),
        };
        Self {
            guts: ((enabled as u32) << 31) + ((rtr_allowed as u32) << 30) + raw_frame_id,
        }
    }

    pub fn is_valid(&self) -> bool {
        (self.guts >> 31) & 0x1 == 0x0
    }

    pub fn rtr_allowed(&self) -> bool {
        (self.guts >> 30) & 0x1 == 0x0
    }

    fn is_extended_id(&self) -> bool {
        (self.guts >> 29) & 0x1 == 0x1
    }

    pub fn assigned_frame_id(&self) -> FrameId {
        if self.is_extended_id() {
            FrameId::Extended(self.guts & 0x1FFF_FFFF)
        } else {
            FrameId::Standard((self.guts & 0x7FF) as u16)
        }
    }
}

impl Default for CobId {
    fn default() -> Self {
        Self {
            guts: Self::DISABLED_ID,
        }
    }
}

pub struct ObjectDictionary<
    const ENTRY_COUNT: usize,
    const RPDO_COUNT: usize,
    const TPDO_COUNT: usize,
> {
    error_register: u8,
    manufacturer_status_register: u32,
    predefined_errors: [u32; 8],
    entries: heapless::Vec<Variable, ENTRY_COUNT>,
    tpdo_mappings: [PdoConfiguration; TPDO_COUNT],
    rpdo_mappings: [PdoConfiguration; RPDO_COUNT],
}

impl<const ENTRY_COUNT: usize, const RPDO_COUNT: usize, const TPDO_COUNT: usize>
    ObjectDictionary<ENTRY_COUNT, RPDO_COUNT, TPDO_COUNT>
{
    pub fn new(
        error_register: u8,
        manufacturer_status_register: u32,
        predefined_errors: [u32; 8],
        tpdo_mappings: [PdoConfiguration; TPDO_COUNT],
        rpdo_mappings: [PdoConfiguration; RPDO_COUNT],
        entries: heapless::Vec<Variable, ENTRY_COUNT>,
    ) -> Self {
        let mut e = entries;
        e.sort_by_key(|v| v.id);

        Self {
            error_register,
            manufacturer_status_register,
            predefined_errors,
            entries: e,
            tpdo_mappings,
            rpdo_mappings,
        }
    }

    pub fn get_mut_variable(&mut self, id: EntryId) -> Option<&mut Variable> {
        match self.entries.binary_search_by_key(&id, |v| v.id) {
            Ok(idx) => Some(self.entries.get_mut(idx).unwrap()),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use heapless::Vec;

    use crate::object_dictionary::ObjectDictionary;

    #[test]
    fn it_works() {
        let mut od = ObjectDictionary::new(
            0,
            0,
            [Default::default(); 8],
            [Default::default(); 8],
            [Default::default(); 8],
            Vec::<_, 0>::from_slice(&[]).unwrap(),
        );
    }
}
