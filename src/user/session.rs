use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};


#[derive(Debug,Clone)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: Uuid) -> Self {
        let created = Utc::now();
        let expires = created + Duration::hours(1); // Session expires in 1 hour

        Self {
            id: Uuid::new_v4(),
            user_id,
            created_at: created,
            expires_at: expires,
        }
    }

    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
}