#[derive(Serialize, Deserialize, Debug,)]
pub enum ErrorCode {
    BAD_REQUEST,
    NOT_FOUND,
    INTERNAL
}