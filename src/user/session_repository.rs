use crate::user::session::Session;
use uuid::Uuid;
use chrono::{Utc, Duration};

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

    pub fn refresh_session(&mut self, session_id: &Uuid, hours: i64) -> bool {
        if let Some(session) = self.sessions.iter_mut().find(|s| s.id == *session_id) {
            session.expires_at = Utc::now() + Duration::hours(hours);
            return true;
        }   
        false
    } 
    
}  