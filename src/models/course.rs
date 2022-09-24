use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, PartialEq)]
pub enum EnrollmentRole {
    Student,
    Teacher,
    TA,
    Designer,
    Observer,
}

#[derive(Deserialize, Debug)]
pub struct Enrollment {
    pub enrollment_state: String,
    pub limit_privileges_to_course_section: bool,
    pub role: EnrollmentRole,
    pub role_id: i32,
    pub r#type: String,
    pub user_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct Section {
    pub end_at: Option<String>,
    pub enrollment_role: EnrollmentRole,
    pub id: i32,
    pub name: String,
    pub start_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Calendar {
    pub ics: String,
}

#[derive(Deserialize, Debug)]
pub struct Course {
    pub account_id: i32,
    pub apply_assignment_group_weights: bool,
    pub blueprint: bool,
    pub calendar: Calendar,
    pub course_code: String,
    pub course_color: Option<Value>,
    pub created_at: String,
    pub default_view: String,
    pub end_at: Option<String>,
    pub enrollment_term_id: i32,
    pub enrollments: Vec<Enrollment>,
    pub friendly_name: Option<String>,
    pub grade_passback_setting: Option<String>,
    pub grading_standard_id: Option<i32>,
    pub hide_final_grades: bool,
    pub homeroom_course: bool,
    pub id: i32,
    pub is_public: Option<bool>,
    pub is_public_to_auth_users: bool,
    pub license: Option<String>,
    pub name: String,
    pub public_syllabus: bool,
    pub public_syllabus_to_auth: bool,
    pub restrict_enrollments_to_course_dates: bool,
    pub root_account_id: i32,
    pub sections: Vec<Section>,
    pub start_at: Option<String>,
    pub storage_quota_mb: i32,
    pub template: bool,
    pub time_zone: String,
    pub uuid: String,
    pub workflow_state: String,
}
