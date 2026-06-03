use std::ptr::null;

use bytes::Bytes;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: EmailAddress,
    pub auth_provider: String, //value object
    pub onboarded: bool,
    pub created_at: DateTime<Utc>,
}

// i may need to work on this
pub struct RiffyVoiceStyle {}

pub struct UserOnboarding {
    pub user_id: Uuid,
    pub quiet_time_day: String,
    pub quiet_time_night: String,
    pub first_recording: Recording,
}

pub struct SpeakingGoal {
    pub id: Uuid,
    pub title: String,
    pub circle_id: Uuid,
    pub goal: SpeakingGoalType,
    pub status: Status,
    pub resources: SpeakingGoalResourcesPack,
}

pub struct SpeakingGoalResourcesPack {
    pub id: Uuid,
    pub goal_id: Uuid,
    pub title: String,
    pub tldr: String,
}

pub struct SpeakingGoalResourceItem {
    pub id: Uuid,
    pub kind: ResourceKind,
    pub pack_id: Uuid,
    pub title: String,
    pub tldr: String,
    pub body: Option<String>,
    pub url: Option<String>,
    pub option_idex: u8,
}

pub struct WordOwnershipSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: Status,
    pub riffy_words: WordOwnershipSentence,
    pub valid_recording: Option<Recording>, //value object
    pub attempts: u8,
    pub duration_secs: u32,
    pub rejection_count: u8,
    pub created_at: DateTime<Utc>,
}

pub struct WordOwnershipAttempt {
    pub id: Uuid,
    pub word_session_id: Uuid,
    pub recording: Recording, //value object
    pub transcript: Transcript,
    pub created_at: DateTime<Utc>,
}

pub struct SpeakingGoalSession {
    pub id: Uuid,
    pub speaking_goal_id: Uuid,
    pub status: Status,
    pub riffy_words: VocalEgoStatement,
    pub valid_recording: Option<Recording>,
    pub attempts: u8,
    pub duration_secs: u32,
    pub rejection_count: u8,
    pub created_at: DateTime<Utc>,
}

pub struct SpeakingGoalSessionAttempt {
    pub id: Uuid,
    pub goal_session_id: Uuid,
    pub recording: Recording,
    pub transcript: Transcript,
    pub passed: bool,
    pub created_at: DateTime<Utc>,
}

pub struct Recording {
    pub id: Uuid,
    pub file: Bytes,
    pub size: usize,
    pub duration: u16,
}

pub struct Circle {
    pub id: Uuid,
    pub user_id: Uuid,
    pub planned_duration_days: u8,
    pub status: Status,
    pub goal_count: u8,
    pub session_count: u16,
    pub grace_remaining: u8,
    pub created_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}

pub struct Transcript {
    pub id: Uuid,
    pub recording_id: Uuid,
    pub text: String,
    pub transcript_type: TranscriptType,
}

pub struct WordOwnership {
    pub id: Uuid,
    pub user_id: Uuid,
    pub word: String,
    pub pronounciation: Pronounctiation,
    pub properties: Vec<String>,
    pub sentences: [String; 10],
    pub created_at: DateTime<Utc>,
}

pub struct NotificationPreferences {
    pub notify_quiet_time_day: bool,
    pub notify_quiet_time_night: bool,
    pub practice_speaking_goal: bool,
}

// ------ Enums -------
pub enum RiffyStates {
    Idle,
    Happy,
    Sad,
    Thinking,
    Listening,
    Speaking,
}

pub enum Status {
    Active,
    Archived,
    Expired,
    Completed,
}

pub enum SpeakingGoalType {
    SituationalGoal,
    TopicGoal,
}
pub enum TranscriptType {
    Raw,
    Processed,
}

pub enum ResourceKind {
    Video,
    Article,
}

// --- value object implementations ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAddress(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordOwnershipSentence(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pronounctiation(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VocalEgoStatement(String);

impl EmailAddress {
    pub fn new(input: impl Into<String>) -> Result<Self, &'static str> {
        let email = input.into().trim().to_string();

        if email.is_empty() {
            return Err("Email can not be empty");
        }

        if email.contains(' ') {
            return Err("Email can not contain space");
        }

        let email_parts: Vec<&str> = email.split('@').collect();
        let local = email_parts[0];
        let domain = email_parts[1];

        if local.is_empty() || domain.is_empty() {
            return Err("Email must contain an address and domain");
        }

        if !matches!(domain, "gmail.com" | "protonmail.com" | "outlook.com") {
            return Err("This email is not supported");
        }
        Ok(Self(email))
    }
}

impl WordOwnershipSentence {
    pub fn new(input: impl Into<String>) -> Result<Self, &'static str> {
        let sentence = input.into().trim().to_string();

        if sentence.is_empty() {
            return Err("Sentence can not be empty");
        }

        if sentence.len() > 100 {
            return Err("Sentence is too long");
        }

        if sentence.contains('\n') || sentence.contains('\r') {
            return Err("Sentence can only be a one liner");
        }

        Ok(Self(sentence))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Pronounctiation {
    pub fn new(input: impl Into<String>) -> Result<Self, &'static str> {
        let word = input.into().trim().to_string();

        if word.is_empty() {
            return Err("Pronouciation cannot be empty");
        }

        if word.len() > 12 {
            return Err("Pronounciation is too long");
        }

        if word.contains('.') {
            return Err("prounctiation cannot have full stop");
        }

        Ok(Self(word))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl VocalEgoStatement {
    pub fn new(input: impl Into<String>) -> Result<Self, &'static str> {
        let statement = input.into().trim().to_string();

        if statement.is_empty() {
            return Err("Statement cannot be empty");
        }

        if statement.len() > 250 {
            return Err("Statement is too long");
        }

        Ok(Self(statement))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
