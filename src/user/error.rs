use std::fmt;

#[derive(Debug)]
pub enum AuthError {
    //Validation fehler
    InvalidPassword,
    InvalidUsername,
    InvalidEmail,

    //Login/Sec Fehler
    UserNotFound,
    InvalidPasswordLogin,
    SessionExpired,

    //Existence fehler
    UsernameExists,
    EmailExists,

    //Technische fehler
    PasswordHashingFailed,
    
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::InvalidPassword => write!(f, "Invalid password"),
            AuthError::InvalidUsername => write!(f, "Username format is invalid"),
            AuthError::InvalidEmail => write!(f, "Email format is invalid"),
            AuthError::UsernameExists => write!(f, "Username already exists"),
            AuthError::EmailExists => write!(f, "Email already exists"),
            AuthError::PasswordHashingFailed => write!(f, "Password hashing failed"),
            AuthError::SessionExpired => write!(f, "Session has expired"),
            AuthError::InvalidPasswordLogin => write!(f, "Password is incorrect"),
        
        }
    }
}
pub type AuthResult<T> = Result<T, AuthError>;