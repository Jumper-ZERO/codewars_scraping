use serde::{Deserialize, Serialize};
use crate::codewars::rank::Rank;

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct User {
    pub username: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Unresolved {
    pub issues: u32,
    pub suggestions: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Kata {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub category: String,
    pub published_at: String,
    pub approved_at: String,
    pub languages: Vec<String>,
    pub url: String,
    pub rank: Rank,
    pub created_at: String,
    pub created_by: User,
    pub approved_by: User,
    pub description: String,
    pub total_attempts: u32,
    pub total_completed: u32,
    pub total_starts: Option<u32>,
    pub vote_score: i32,
    pub tags: Vec<String>,
    pub contributors_wanted: bool,
    pub unresolved: Unresolved,
}

#[derive(Debug)]
pub enum KataError {
    NoLanguageAvailable,
}

impl Kata {
    pub fn train_url(&self) -> Result<String, KataError> {
        if self.languages.contains(&"rust".to_string()) {
            Ok(format!("{}/train/rust", &self.url))
        } else {
            Err(KataError::NoLanguageAvailable)
        }
    }

    pub fn rank(&self) -> String {
        self.rank.snake_case()
    }
}
