use embedded_can::{Frame, Id, StandardId};
use heapless::Vec;

pub(crate) struct TestFrame {
    id: embedded_can::Id,
    data: heapless::Vec<u8, 8>,
    remote: bool,
    dlc: usize,
}

impl TestFrame {
    pub(crate) fn new(std_id: u16, data: &[u8]) -> Self {
        assert!(std_id <= 0x7FF, "Invalid STD ID");
        assert!(data.len() <= 8, "Invalid data length");
        Self {
            id: unsafe { Id::Standard(StandardId::new_unchecked(std_id)) },
            data: Vec::from_slice(data).unwrap(),
            remote: false,
            dlc: data.len(),
        }
    }
}

impl Frame for TestFrame {
    fn new(id: impl Into<embedded_can::Id>, data: &[u8]) -> Option<Self> {
        Some(Self {
            id: id.into(),
            data: Vec::from_slice(data).ok()?,
            remote: false,
            dlc: data.len(),
        })
    }

    fn new_remote(id: impl Into<embedded_can::Id>, dlc: usize) -> Option<Self> {
        Some(Self {
            id: id.into(),
            data: Vec::new(),
            remote: true,
            dlc: dlc,
        })
    }

    fn is_extended(&self) -> bool {
        return match self.id {
            embedded_can::Id::Extended(_) => true,
            embedded_can::Id::Standard(_) => false,
        };
    }

    fn is_remote_frame(&self) -> bool {
        self.remote
    }

    fn id(&self) -> embedded_can::Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.dlc
    }

    fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}
