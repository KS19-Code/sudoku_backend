use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone)]
pub struct ResetToken {
    pub user_id: Uuid,
    pub token: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl ResetToken {
    pub fn new(user_id: Uuid) -> Self {
        let created = Utc::now();
        let expires = created + Duration::minutes(30);

        Self {
            user_id,
            token: Uuid::new_v4(),
            created_at: created,
            expires_at: expires,
        }
    }

    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
}

