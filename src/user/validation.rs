pub fn validate_username(username: &str) -> Result<(), &'static str> {
    let name = username.trim();

    if name.is_empty() {
        return Err("Username cannot be empty");
    }

    if name.len() < 3 || name.len() > 20 {
        return Err("Username must be at between 3 and 20 characters long");
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == '_' ) {
        return Err("Username may only contain letter, numbers and underscores");
    }

    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), &'static str> {
    let e = email.trim();

    if e.is_empty() {
        return Err("Email cannot be empty");
    }

    let pattern = regex::Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$")
        .expect("Invalid regex");

    if !pattern.is_match(e) {
        return Err("Email format is invalid");
    }

    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long");
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        return Err("Password must contain at least one uppercase letter");
    }

    if !password.chars().any(|c| c.is_numeric()) {
        return Err("Password must contain at least one number");
    }

    if !password.chars().any(|c| !c.is_alphanumeric()) {
        return Err("Password must contain at least one special character");
    }

    Ok(())
}

