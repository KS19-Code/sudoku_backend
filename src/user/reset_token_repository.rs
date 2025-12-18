use uuid::Uuid;
use crate::user::reset_token::ResetToken;

pub struct ResetTokenRepository {
    tokens: Vec<ResetToken>,
}

impl ResetTokenRepository {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn add_token(&mut self, token: ResetToken) {
        self.tokens.push(token);
    }

    pub fn find_token(&self, token: &Uuid) -> Option<&ResetToken> {
        self.tokens.iter().find(|t| &t.token == token)
    }

    pub fn remove_token(&mut self, token: &Uuid) {
        self.tokens.retain(|t| &t.token != token);
    }

    pub fn clean_expired(&mut self) {
        self.tokens.retain(|t| t.is_valid());
    }
}
