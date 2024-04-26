use crate::object_dictionary::CobId;

#[derive(Clone, Copy)]
pub enum PdoTransmissionType {
    Synchronous,
    EventDriven,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PdoEntryMapping {
    index: u16,
    sub_index: u8,
    length: u8,
}

impl Default for PdoEntryMapping {
    fn default() -> Self {
        Self {
            index: 0x0,
            sub_index: 0x0,
            length: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct PdoConfiguration {
    cob_id: CobId,
    transmission_type: PdoTransmissionType,
    number_of_map_values: u8,
    entry_mapping: [PdoEntryMapping; 8],
}

impl PdoConfiguration {
    pub fn new(
        id: CobId,
        transmission_type: PdoTransmissionType,
        mapped_val_count: u8,
        entry_mapping: [PdoEntryMapping; 8],
    ) -> Self {
        Self {
            cob_id: id,
            transmission_type: transmission_type,
            number_of_map_values: mapped_val_count,
            entry_mapping: entry_mapping,
        }
    }
}

impl Default for PdoConfiguration {
    fn default() -> Self {
        Self {
            cob_id: Default::default(),
            transmission_type: PdoTransmissionType::EventDriven,
            number_of_map_values: 0,
            entry_mapping: Default::default(),
        }
    }
}
