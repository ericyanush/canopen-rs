use embedded_can::{Frame, Id};
use heapless::Vec;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::{node::NodeId, object_dictionary::EntryId};

#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SdoAbortCode {
    ToggleBitNotAlternated = 0x0503_0000,
    SDOProtocolTimedOut = 0x0504_0000,
    CommandSpecifierError = 0x0504_0001,
    InvalidBlockSize = 0x0504_0002,
    InvalidSequenceNumber = 0x0504_0003,
    CRCError = 0x0504_0004,
    OutOfMemory = 0x0504_0005,
    UnsupportedAccess = 0x0601_0000,
    WriteOnlyError = 0x0601_0001,
    ReadOnlyError = 0x0601_0002,
    ObjectDoesNotExist = 0x0602_0000,
    ObjectCannotBeMapped = 0x0604_0041,
    PDOOverflow = 0x0604_0042,
    ParameterIncompatibility = 0x0604_0043,
    InternalIncompatibility = 0x0604_0047,
    HardwareError = 0x0606_0000,
    WrongLength = 0x0607_0010,
    TooLong = 0x0607_0012,
    TooShort = 0x0607_0013,
    SubindexDoesNotExist = 0x0609_0011,
    InvalidValue = 0x0609_0030,
    ValueTooHigh = 0x0609_0031,
    ValueTooLow = 0x0609_0032,
    MaxLessThanMin = 0x0609_0036,
    ResourceNotAvailable = 0x060A_0023,
    GeneralError = 0x0800_0000,
    TransferOrStorageError = 0x0800_0020,
    LocalControlError = 0x0800_0021,
    DeviceStateError = 0x0800_0022,
    DictionaryError = 0x0800_0023,
    NoDataAvailable = 0x0800_0024,
}

impl SdoAbortCode {
    pub fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 4 {
            return None;
        }
        FromPrimitive::from_u32(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn to_le_bytes(&self) -> [u8; 4] {
        let mut val = [0; 4];
        val.copy_from_slice(&(*self as u32).to_le_bytes());
        val
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdoFrame {
    UploadRequest {
        id: EntryId,
    },
    ExpeditedDownloadRequest {
        id: EntryId,
        payload: Vec<u8, 4>,
    },
    ExpeditedDownloadResponse {
        id: EntryId,
    },
    ExpeditedUploadResponse {
        id: EntryId,
        payload: Vec<u8, 4>,
    },
    SegmentedUploadInitiateResponse {
        id: EntryId,
        size: u32,
    },
    SegmentedUploadRequest {
        toggle: bool,
    },
    SegmentedUploadResponse {
        payload: Vec<u8, 7>,
    },
    SegmentedDownloadInitiateRequest {
        id: EntryId,
        size: u32,
    },
    SegmentedDownloadInitiateResponse {
        id: EntryId,
    },
    SegmentedDownloadRequest {
        toggle: bool,
        last: bool,
        payload: Vec<u8, 7>,
    },
    SegmentedDownloadResponse {
        toggle: bool,
    },
    Abort {
        id: EntryId,
        code: SdoAbortCode,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClientCommand {
    ExpeditedDownload {
        length: u8,
    },
    InitiateSegmentedDownload,
    DownloadSegmentRequest {
        toggle: bool,
        length: u8,
        last_seg: bool,
    },
    InitiateUpload,
    UploadSegmentRequest {
        toggle: bool,
    },
    Abort,
}

struct InvalidCommandCode;
impl TryFrom<u8> for ClientCommand {
    type Error = InvalidCommandCode;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value >> 5 {
            0 => Ok(ClientCommand::DownloadSegmentRequest {
                toggle: (value >> 4) & 0b1 == 0b1,
                length: 7 - ((value >> 1) & 0b111),
                last_seg: value & 0b1 == 0b1,
            }),
            1 if (value >> 1) & 0b1 == 0b1 => Ok(ClientCommand::ExpeditedDownload {
                length: 4 - ((value >> 2) & 0b11),
            }),
            1 => Ok(ClientCommand::InitiateSegmentedDownload),
            2 => Ok(ClientCommand::InitiateUpload),
            3 => Ok(ClientCommand::UploadSegmentRequest {
                toggle: (value >> 4) & 0b1 == 0b1,
            }),
            4 => Ok(ClientCommand::Abort),
            _ => Err(InvalidCommandCode),
        }
    }
}

enum ServerCommand {
    InitiateDownloadResponse,
    DownloadSegmentResponse(bool),
    UploadInitiateExpeditedResponse(u8),
    UploadInitiateSegmentedResponse,
    UploadSegmentResponse {
        toggle: bool,
        length: u8,
        last: bool,
    },
    Abort,
}

impl Into<u8> for ServerCommand {
    fn into(self) -> u8 {
        match self {
            Self::InitiateDownloadResponse => 3 << 5,
            Self::DownloadSegmentResponse(toggle) => 1 << 5 | (toggle as u8) << 4,
            Self::UploadInitiateExpeditedResponse(length) => {
                2 << 5 | (4 - length) << 2 | 0b1 << 1 | 0b1
            }
            Self::UploadInitiateSegmentedResponse => 2 << 5 | 0b1,
            Self::UploadSegmentResponse {
                toggle,
                length,
                last,
            } => 0 << 5 | (toggle as u8) << 4 | (7 - length) << 1 | (last as u8),
            Self::Abort => 4 << 5,
        }
    }
}

pub(crate) struct SDOCoder;

impl SDOCoder {
    const RX_ID_OFFSET: u16 = 0x600;
    const TX_ID_OFFSET: u16 = 0x580;

    fn try_decode_rx_frame(self_node_id: NodeId, frame: &impl Frame) -> Option<SdoFrame> {
        match frame.id() {
            Id::Standard(std) => {
                if std.as_raw() != (self_node_id.raw() as u16 + Self::RX_ID_OFFSET) {
                    return None;
                }
            }
            Id::Extended(_) => return None,
        }

        if frame.dlc() != 8 {
            return None;
        }

        let frame_data = frame.data();
        return match ClientCommand::try_from(frame_data[0]) {
            Err(_) => None,
            Ok(ClientCommand::ExpeditedDownload { length }) => {
                Some(SdoFrame::ExpeditedDownloadRequest {
                    id: EntryId::from_bytes(frame_data[1..4].try_into().unwrap()),
                    payload: Vec::<u8, 4>::from_slice(&frame_data[4..(4 + length as usize)])
                        .unwrap(),
                })
            }
            Ok(ClientCommand::InitiateSegmentedDownload) => {
                Some(SdoFrame::SegmentedDownloadInitiateRequest {
                    id: EntryId::from_bytes(frame_data[1..4].try_into().unwrap()),
                    size: u32::from_le_bytes(frame_data[4..8].try_into().unwrap()),
                })
            }
            Ok(ClientCommand::DownloadSegmentRequest {
                toggle,
                length,
                last_seg,
            }) => Some(SdoFrame::SegmentedDownloadRequest {
                toggle: toggle,
                last: last_seg,
                payload: Vec::<u8, 7>::from_slice(
                    frame_data[1..(1 + length as usize)].try_into().unwrap(),
                )
                .unwrap(),
            }),
            Ok(ClientCommand::InitiateUpload) => Some(SdoFrame::UploadRequest {
                id: EntryId::from_bytes(frame_data[1..4].try_into().unwrap()),
            }),
            Ok(ClientCommand::UploadSegmentRequest { toggle }) => {
                Some(SdoFrame::SegmentedUploadRequest { toggle: toggle })
            }
            Ok(ClientCommand::Abort) => match SdoAbortCode::from_le_bytes(&frame_data[4..8]) {
                None => None,
                Some(code) => Some(SdoFrame::Abort {
                    id: EntryId::from_bytes(frame_data[1..4].try_into().unwrap()),
                    code: code,
                }),
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::node::NodeId;
    use crate::object_dictionary::EntryId;
    use crate::sdo::{SDOCoder, SdoFrame};
    use crate::test_types::TestFrame;

    #[test]
    fn test_rx_decode_wrong_node_id() {
        let frame = TestFrame::new(0x606, &[2 << 5, 0x00, 0x20, 0x01, 0, 0, 0, 0]);
        let decoded = SDOCoder::try_decode_rx_frame(NodeId::new(5).unwrap(), &frame);
        assert!(decoded.is_none());
    }

    #[test]
    fn test_rx_decode_wrong_frame_offset() {
        let frame = TestFrame::new(0x585, &[2 << 5, 0x00, 0x20, 0x01, 0, 0, 0, 0]);
        let decoded = SDOCoder::try_decode_rx_frame(NodeId::new(5).unwrap(), &frame);
        assert!(decoded.is_none());
    }

    #[test]
    fn test_rx_decode_upload_req() {
        let frame = TestFrame::new(0x605, &[2 << 5, 0x00, 0x20, 0x01, 0, 0, 0, 0]);
        let decoded = SDOCoder::try_decode_rx_frame(NodeId::new(5).unwrap(), &frame);
        assert!(decoded.is_some());
        let sdo = decoded.unwrap();
        assert_eq!(
            sdo,
            SdoFrame::UploadRequest {
                id: EntryId::new(0x2000, 0x01)
            }
        )
    }
}