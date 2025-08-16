#[repr(i8)]
pub enum StatusCode {
    UnknownError = -1,
    Success = 0,
    InvalidArgument = 1,
    FlatBufferError = 2,
    FileNotFound = 3,
    IoError = 4,
}
