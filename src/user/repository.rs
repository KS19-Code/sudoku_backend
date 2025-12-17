use crate::user::model::User;

pub struct UserRepository {
    users: Vec<User>,
}
impl UserRepository {
    pub fn new () -> Self {
        Self {users: Vec::new()}
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn find_by_username(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|u| u.username == username)
    }

    pub fn find_by_email(&self, email: &str) -> Option<&User> {
    self.users.iter().find(|u| u.email == email)
    }

 }

