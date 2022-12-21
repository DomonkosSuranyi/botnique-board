#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum StreamId {
    InputState,
}

impl From<StreamId> for Option<u8> {
    fn from(id: StreamId) -> Option<u8> {
        Some(id as u8)
    }
}
