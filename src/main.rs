mod user;

use user::repository::UserRepository;
use user::auth::{hash_password, verify_password};
use user::model::User;

use uuid::Uuid;
use chrono::Utc;   

fn main() {
    let mut repo = UserRepository::new();

    println!("=== User Registration ===");

    //Registrierung ausprobieren 
    let username = "kelvin";
    let email = "kelvin@example.com";
    let password = "securepassword";

    match register_user(&mut repo, username, email, password) {
        Ok(_) => println!("Registration successful"),
        Err(err) => println!("Registration failed: {}", err),
    }
    

    //login ausprobieren
    match login_user(&repo, username, password) {
        Ok(_) => println!("Login successful"),
        Err(err) => println!("Login failed: {}", err),
    }
}

fn register_user(repo: &UserRepository, username: &str, email: &str, password: &str) -> Result<(), String> {
    let user = repo 
        .find_by_username(username)
        .ok_or("Username already exists")?;

    let ok = verify_password(password, &user.password_hash)
        .map_err(|_| "Password verification failed")?;

    if ok {
        Ok(())
    } else {
        Err("Invalid password".to_string())
    }
}

fn login_user(repo: &UserRepository, username: &str, password: &str) -> Result<(), String> {
    let user = repo 
        .find_by_username(username)
        .ok_or("User not found")?;

    let ok = verify_password(password, &user.password_hash)
        .map_err(|_| "Password verification failed")?;

    if ok {
        Ok(())
    } else {
        Err("Invalid password".to_string())
    }
}


  