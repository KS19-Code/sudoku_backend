pub fn validate_username(username: &str) -> Result<(), &'static str> {
    let name = username.trim();

    if name.is_empty() {
        return Err("Username cannot be empty");
    }

    if name.len() < 3 {
        return Err("Username must be at least 3 characters long");
    }

    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), &'static str> {
    let e = email.trim();

    if e.is_empty() {
        return Err("Email cannot be empty");
    }

    if !e.contains("@") || !e.contains(".") {
        return Err("Invalid email format");
    }

    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long");
    }

    Ok(())
}

