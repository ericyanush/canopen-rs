use embedded_can::{Frame, Id};
use heapless::Vec;

pub struct EncodedCANOpenFrame {
    id: Id,
    data: Vec<u8, 8>,
}

impl Frame for EncodedCANOpenFrame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        Some(Self {
            id: id.into(),
            data: Vec::from_slice(data).ok()?,
        })
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self> {
        todo!()
    }

    fn is_extended(&self) -> bool {
        match self.id {
            Id::Extended(_) => true,
            _ => false,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.data.len()
    }

    fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl EncodedCANOpenFrame {
    pub(crate) fn from_vec_data(id: impl Into<Id>, data: Vec<u8, 8>) -> Self {
        Self {
            id: id.into(),
            data: data,
        }
    }
}

#[cfg(test)]
impl EncodedCANOpenFrame {
    pub(crate) fn new(std_id: u16, data: &[u8]) -> Self {
        assert!(std_id <= 0x7FF, "Invalid STD ID");
        assert!(data.len() <= 8, "Invalid data length");
        Self {
            id: unsafe { Id::Standard(embedded_can::StandardId::new_unchecked(std_id)) },
            data: Vec::from_slice(data).unwrap(),
        }
    }
}
