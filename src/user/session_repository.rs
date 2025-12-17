use crate::user::session::Session;
use uuid::Uuid;
pub struct SessionRepository {
    sessions: Vec<Session>,
}

impl SessionRepository {
    pub fn new() -> Self {
        Self { sessions: Vec::new() }
    }

    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
    }

    pub fn find_by_session_id(&self, id: &Uuid) -> Option<&Session> {
        self.sessions.iter().find(|s| s.id == *id)
    }

    pub fn validate_session(&self, id: &Uuid) -> bool {
        self.find_by_session_id(id)
            .map(|s| s.is_valid())
            .unwrap_or(false)
    }
    pub fn remove_session(&mut self, id: &Uuid) {
        self.sessions.retain(|s| s.id != *id);
    }

    pub fn clean_expired_sessions(&mut self) {
        self.sessions.retain(|s| s.is_valid());
    }
}