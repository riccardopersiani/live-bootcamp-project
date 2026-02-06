#[derive(Debug, Clone)]
pub enum AuthAPIError {
    IncorrectCredentials,
    InvalidCredentials,
    InvalidToken,
    MalformedToken,
    MissingToken,
    UnexpectedError,
    UserAlreadyExists,
}
