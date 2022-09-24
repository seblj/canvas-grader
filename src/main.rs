use crate::download::download_file;
use crate::models::assignment::Assignment;
use crate::models::course::{Course, EnrollmentRole};
use futures::future::join_all;
use models::assignment::Attachment;
use parse_link_header::parse;
use reqwest::Response;

mod download;
mod models;

const BASE_URL: &str = "https://uit.instructure.com/api/v1/";

async fn get(url: &str) -> Result<Response, anyhow::Error> {
    let token = dotenv::var("TOKEN")
        .expect("Couldn't find canvas token")
        .trim_start_matches("Bearer ")
        .to_owned();

    Ok(reqwest::Client::new()
        .get(format!("{BASE_URL}/{url}"))
        .bearer_auth(token)
        .send()
        .await?)
}

async fn get_assignments_for_section(section: &i32) -> Result<Vec<Assignment>, anyhow::Error> {
    let assignment = dotenv::var("ASSIGNMENT").expect("Couldn't find id for assignment");
    let mut url = format!(
        "sections/{section}/assignments/{assignment}/submissions?per_page=100&include=user"
    );
    let mut assignments = Vec::new();
    loop {
        let res = get(&url).await?;
        let links = parse(res.headers().get("link").unwrap().to_str()?)?;

        let body = res.json::<Vec<Assignment>>().await?;
        assignments.extend(body);

        // helper function to make the below code easier to follow
        let link = |s: &str| links.get(&Some(s.to_owned()));

        /*
         * This can be written as:
         * if let Some(last) = link("last")
         *  && let Some(current) = link("current")
         *  && current.uri == last.uri
         *
         *  When if-let-chains are stabilized.!
         */
        if matches!((link("last"), link("current")), (Some(last), Some(current)) if current.uri == last.uri)
        {
            break;
        }

        if let Some(next) = link("next") {
            url = next.uri.to_string();
        }
    }

    Ok(assignments)
}

async fn get_assignments(section_ids: &[i32]) -> Result<Vec<Assignment>, anyhow::Error> {
    Ok(join_all(
        section_ids.iter().map(|section_id| async move {
            get_assignments_for_section(&section_id).await.unwrap()
        }),
    )
    .await
    .into_iter()
    .flatten()
    .collect())
}

async fn download_attachments(attachments: Vec<Attachment>, path: String, student_name: String) {
    let inner_tasks: Vec<_> = attachments
        .into_iter()
        .map(|attachment| {
            let path = path.clone();
            let student_name = student_name.clone();
            tokio::spawn(async move {
                if let Err(_) = download_file(&attachment.url, &attachment.filename, &path).await {
                    println!("Error when downloading submission from {}", student_name);
                }
            })
        })
        .collect();
    join_all(inner_tasks).await;
}

async fn download_assignments(assignments: Vec<Assignment>) {
    let tasks: Vec<_> = assignments
        .into_iter()
        .map(|assignment| {
            tokio::spawn(async move {
                let student_name = &assignment.user.as_ref().unwrap().name;
                let path = format!(
                    "{}/{}",
                    std::env::current_dir().unwrap().to_str().unwrap(),
                    student_name,
                );
                if let Some(attachments) = assignment.attachments {
                    std::fs::create_dir_all(&path).unwrap();
                    println!("Downloading submission from: {}", student_name);
                    download_attachments(attachments, path, student_name.to_owned()).await;
                }
            })
        })
        .collect();
    join_all(tasks).await;
}

async fn get_section_ids() -> Result<Vec<i32>, anyhow::Error> {
    let url = "users/self/courses?include=sections";
    let res = get(url).await?;

    let body = res.json::<Vec<Course>>().await?;
    let course = body
        .iter()
        .find(|x| {
            x.id == dotenv::var("COURSE")
                .expect("Couldn't find id for course")
                .parse::<i32>()
                .expect("Course id is not a number")
        })
        .expect("Couldn't find course");
    Ok(course
        .sections
        .iter()
        .filter_map(|section| {
            (section.enrollment_role == EnrollmentRole::TA
                || section.enrollment_role == EnrollmentRole::Teacher
                    && section.name != course.course_code)
                .then_some(section.id)
        })
        .collect())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let section_ids = get_section_ids().await?;
    let assignments = get_assignments(&section_ids).await?;
    download_assignments(assignments).await;

    Ok(())
}
