mod user;

use user::repository::UserRepository;
use user::auth::{hash_password, verify_password};
use user::model::User;
use user::validation::{validate_username, validate_email, validate_password};
use user::error::{AuthError, AuthResult};

use crate::user::session::Session;
use crate::user::session_repository::SessionRepository;
use crate::user::reset_token::ResetToken;
use crate::user::reset_token_repository::ResetTokenRepository;

use uuid::Uuid;
use chrono::{Utc, Duration};

fn main() {
    let mut repo = UserRepository::new();
    let mut session_repo = SessionRepository::new();
    let mut token_repo = ResetTokenRepository::new();


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

           // Test: Passwort ändern
            println!("=== Testing password change ===");
            match change_password(&session_repo, &mut repo, &session_id, "securepassword", "newsecurepassword") {
            Ok(_) => println!("Password changed successfully."),
            Err(err) => println!("Password change failed: {}", err),
            }
      
          
            //session refresh direkt nach login
            session_repo.refresh_session(&session_id, 24);
            println!("Session refreshed for another 24 hours.");

            // abgelaufene sessions bereinigen
            session_repo.clean_expired_sessions();

            //check: ist user eingeloggt?
            if is_logged_in(&session_repo, &session_id) {
                println!("User is logged in.");
            } else {
                println!("User is not logged in.");
            }

            // (=== User holenn==)
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

            println!("=== PASSWORD RESET TEST ===");

            // Token generieren
            let reset_token = match request_password_reset(&repo, &mut token_repo, email) {
                Ok(t) => {
                    println!("Generated reset token: {}", t);
                    t
                }
                Err(err) => {
                    println!("Failed to generate reset token: {}", err);
                    return;
                }
            };

            // neues Passwort setzen
            match reset_password(&mut repo, &mut token_repo, &reset_token, "verynewpassword123") {
                Ok(_) => println!("Password reset completed."),
                Err(err) => println!("Password reset failed: {}", err),
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

fn change_password(
    session_repo: &SessionRepository,
    user_repo: &mut UserRepository,
    session_id: &Uuid,
    old_password: &str,
    new_password: &str,
) -> AuthResult<()> {
    // session check 
    let session = session_repo
        .find_by_session_id(session_id)
        .ok_or(AuthError::SessionExpired)?;

    if !session.is_valid() {
        return Err(AuthError::SessionExpired);
    }

    let user_id = session.user_id;

    //user holen
    let user = user_repo
        .find_by_id(&user_id)
        .ok_or(AuthError::UserNotFound)?;

    // altes passwort checken
    let ok = verify_password(old_password, &user.password_hash)
        .map_err(|_| AuthError::InvalidPasswordLogin)?;

    if !ok {
        return Err(AuthError::InvalidPasswordLogin);
    }

    //neues passwort validieren
    validate_password(new_password).map_err(|_| AuthError::InvalidPassword)?;

    //neues passwort hashen
    let new_hash = hash_password(new_password)
        .map_err(|_| AuthError::PasswordHashingFailed)?;

    //passwort im user repository updaten
    user_repo.update_password(&user_id, new_hash);

    Ok(())
}

fn request_password_reset(repo: &UserRepository, token_repo: &mut ResetTokenRepository, email: &str) -> AuthResult<Uuid> {
    let user = repo 
        .find_by_email(email)
        .ok_or(AuthError::UserNotFound)?;

    let token = ResetToken::new(user.id);
    let token_id = token.token;

    token_repo.add_token(token);

    Ok(token_id)
}

fn reset_password(
    repo: &mut UserRepository,
    token_repo: &mut ResetTokenRepository,
    token: &Uuid,
    new_password: &str,
) -> AuthResult<()> {
    // 1. Token prüfen
    let token_data = token_repo
        .find_token(token)
        .ok_or(AuthError::TokenInvalid)?;

    if !token_data.is_valid() {
        return Err(AuthError::TokenExpired);
    }

    // 2. neues Passwort validieren
    validate_password(new_password).map_err(|_| AuthError::InvalidPassword)?;

    // 3. neues Passwort hashen
    let new_hash = hash_password(new_password)
        .map_err(|_| AuthError::PasswordHashingFailed)?;

    // 4. Passwort in UserRepository setzen
    repo.update_password(&token_data.user_id, new_hash);

    // 5. Token löschen (kann nur einmal genutzt werden)
    token_repo.remove_token(token);

    Ok(())
}
