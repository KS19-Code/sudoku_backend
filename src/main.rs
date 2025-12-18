mod user;

use user::repository::UserRepository;
use user::session_repository::SessionRepository;
use user::reset_token_repository::ResetTokenRepository;

use user::services::{
    register_user,
    login_user,
    is_logged_in,
    logout_user,
    get_user_from_session,
    change_password,
    request_password_reset,
    reset_password,
};

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









