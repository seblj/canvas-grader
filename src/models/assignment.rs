use serde::Deserialize;
use serde_json::Value;

use crate::models::user::User;

#[derive(Deserialize, Debug, Clone)]
pub struct Attachment {
    #[serde(rename = "content-type")]
    pub content_type: String,
    pub created_at: String,
    pub display_name: String,
    pub filename: String,
    pub folder_id: i32,
    pub hidden: bool,
    pub hidden_for_user: bool,
    pub id: i32,
    pub lock_at: Option<Value>,
    pub locked: bool,
    pub locked_for_user: bool,
    pub media_entry_id: Option<Value>,
    pub mime_class: String,
    pub modified_at: String,
    pub preview_url: Option<String>,
    pub size: i32,
    pub thumbnail_url: Option<String>,
    pub unlock_at: Option<String>,
    pub updated_at: String,
    pub upload_status: String,
    pub url: String,
    pub uuid: String,
}

#[derive(Deserialize, Debug)]
pub struct Assignment {
    pub assignment_id: i32,
    pub assignment: Option<Value>,
    pub course: Option<Value>,
    pub attachments: Option<Vec<Attachment>>,
    pub attempt: Option<i32>,
    pub body: Option<String>,
    pub cached_due_date: Option<String>,
    pub entered_grade: Option<String>,
    pub entered_score: Option<f32>,
    pub excused: Option<bool>,
    pub extra_attempts: Option<Value>,
    pub grade: Option<String>,
    pub grade_matches_current_submission: bool,
    pub graded_at: Option<String>,
    pub grader_id: Option<i32>,
    pub grading_period_id: Option<Value>,
    pub id: i32,
    pub late: bool,
    pub late_policy_status: Option<Value>,
    pub missing: bool,
    pub points_deducted: Option<Value>,
    pub posted_at: Option<String>,
    pub preview_url: String,
    pub redo_request: bool,
    pub score: Option<f32>,
    pub seconds_late: i32,
    pub submission_type: Option<String>,
    pub submitted_at: Option<String>,
    pub url: Option<String>,
    pub user: Option<User>,
    pub user_id: i32,
    pub workflow_state: String,
}
