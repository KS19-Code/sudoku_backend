mod user;

use user::repository::UserRepository;
use user::auth::{hash_password, verify_password};
use user::model::User;
use user::validation::{validate_username, validate_email, validate_password};
use user::error::{AuthError, AuthResult};

use uuid::Uuid;
use crate::user::session::Session;
use crate::user::session_repository::SessionRepository;
use chrono::{Utc, Duration};   

fn main() {
    let mut repo = UserRepository::new();
    let mut session_repo = SessionRepository::new();

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
    match login_user(&repo, &mut session_repo, username, password) {
        Ok(session_id) => {
            println!("Login successful, session ID: {}", session_id);

            //check: ist user eingeloggt?
            if is_logged_in(&session_repo, &session_id) {
                println!("User is logged in.");
            } else {
                println!("User is not logged in.");
            }

            // === User holenn==)
            println!("=== testing user from session ===");
            if let Some(user) = get_user_from_session(&session_repo, &repo, &session_id) {
                println!("User from session: {:?}", user.username);
            } else {
                println!("No valid user found for session.");
            }
            
            //LOGOUT TEST 
            println! ("=== Logging out user ===");
            logout_user(&mut session_repo, &session_id);

            //check: ist user jetzt ausgeloggt??
            if is_logged_in(&session_repo, &session_id) {
                println!("User is STILL logged in.");
            } else {
                println!("User is logged out.");
            }
        }
        Err(err) => println!("Login failed: {}", err),
    }  

} 

fn register_user(repo: &mut UserRepository, username: &str, email: &str, password: &str) -> AuthResult<()> {
    
    validate_username(username).map_err(|_| AuthError::InvalidUsername)?;
    validate_email(email).map_err(|_| AuthError::InvalidEmail)?;
    validate_password(password).map_err(|_| AuthError::InvalidPassword)?;

    if repo.find_by_username(username).is_some() {
        return Err(AuthError::UsernameExists);
    }

    if repo.find_by_email(email).is_some() {
        return Err(AuthError::EmailExists);
    }

    let password_hash = hash_password(password)
        .map_err(|_|AuthError::PasswordHashingFailed)?;

    let user = User {
        id: Uuid::new_v4(),
        username: username.to_string(),
        email: email.to_string(),
        password_hash,
        created_at: Utc::now(),
    };

    repo.add_user(user);    

    Ok(())
}

fn login_user(repo: &UserRepository, session_repo: &mut SessionRepository, username: &str, password: &str) -> AuthResult<Uuid> {
    let user = repo 
        .find_by_username(username)
        .ok_or(AuthError::UserNotFound)?;

    let ok = verify_password(password, &user.password_hash)
        .map_err(|_| AuthError::InvalidPasswordLogin)?;

    if !ok {
        return Err(AuthError::InvalidPasswordLogin);
    }

    let session = Session {
        id: Uuid::new_v4(),
        user_id: user.id,
        created_at: Utc::now(),
        expires_at: Utc::now() + Duration::hours(24),
    };

    let session_id = session.id;
    session_repo.add_session(session);

    Ok(session_id)
}

fn is_logged_in(session_repo: &SessionRepository, session_id: &Uuid) -> bool {
    session_repo.validate_session(session_id)
}

fn logout_user(session_repo: &mut SessionRepository, session_id: &Uuid) {
    session_repo.remove_session(session_id);
}

fn get_user_from_session<'a>(
    session_repo: &'a SessionRepository,
    user_repo: &'a UserRepository,
    session_id: &Uuid,
) -> Option<&'a User> {
    // 1. session finden 
    let session = session_repo.find_by_session_id(session_id)?;

    //2.  checken ob session gültig ist 
    if !session.is_valid() {
        return None;
    }

    //3. user anhand der user_id aus session holen 
    user_repo.find_by_id(&session.user_id)
}