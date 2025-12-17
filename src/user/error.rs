use std::fmt;

#[derive(Debug)]
pub enum AuthError {
    UserNotFound,
    InvalidPassword,
    UsernameExists,
    EmailExists,
    PasswordHashingFailed,
    SessionExpired,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::InvalidPassword => write!(f, "Invalid password"),
            AuthError::UsernameExists => write!(f, "Username already exists"),
            AuthError::EmailExists => write!(f, "Email already exists"),
            AuthError::PasswordHashingFailed => write!(f, "Password hashing failed"),
            AuthError::SessionExpired => write!(f, "Session has expired"),
        }
    }
}
pub type AuthResult<T> = Result<T, AuthError>;