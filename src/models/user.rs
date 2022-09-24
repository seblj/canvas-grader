use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct User {
    pub avatar_url: Option<String>,
    pub created_at: String,
    pub email: Option<String>,
    pub id: i32,
    pub login_id: String,
    pub name: String,
    pub short_name: String,
    pub sortable_name: String,
}

#[derive(Deserialize, Debug)]
pub struct SectionUserUser {
    pub created_at: String,
    pub id: i32,
    pub login_id: String,
    pub name: String,
    pub short_name: String,
    pub sortable_name: String,
}

#[derive(Deserialize, Debug)]
pub struct SectionUser {
    pub associated_user_id: Option<i32>,
    pub course_id: i32,
    pub course_section_id: i32,
    pub created_at: String,
    pub end_at: Option<String>,
    pub enrollment_state: String,
    pub html_url: String,
    pub id: i32,
    pub last_activity_at: String,
    pub last_attended_at: Option<String>,
    pub limit_privileges_to_course_section: bool,
    pub role: String,
    pub role_id: i32,
    pub root_account_id: i32,
    pub start_at: Option<Value>,
    pub total_activity_time: i32,
    pub r#type: String,
    pub updated_at: String,
    pub user: SectionUserUser,
    pub user_id: i32,
}
