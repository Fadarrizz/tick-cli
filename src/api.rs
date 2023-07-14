use reqwest::{blocking::Client, header};
use tick_cli::{Role, Project, Task, User};

const BASE_URL: &str = "https://secure.tickspot.com";
const ORG_ID: &str = "45669";
const TOKEN: &str = "d115b93153b4f9968214865e96ff7789";
const USER_AGENT: &str = "tick-cli (auke@ijsfontein.nl)";

pub struct ApiError {
    code: u16,
    message: String,
}

impl ApiError {
    pub fn is_unauthenticated_error(&self) -> bool {
        self.code == 401
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

pub fn get_roles(email: &String, password: &String) -> Result<Vec<Role>, ApiError> {
    let response = Client::new()
        .get(format!("{}/api/v2/roles.json", BASE_URL))
        .header(header::USER_AGENT, USER_AGENT)
        .basic_auth(email, Some(password))
        .send().unwrap();

    match response.status().as_u16() {
        200..=299 => {
            return Ok(response.json().expect("Unable to convert roles response to json"));
        }
        400..=499 => {
            Err(ApiError {
                code: response.status().as_u16(),
                message: response.text().unwrap(),
            })
        },
        _ => {
            panic!("Unexpected status code: {}", response.status());
        }
    }
}

pub fn get_users() -> Vec<User> {
    Client::new()
        .get(format!("{}/{}/api/v2/users.json", BASE_URL, ORG_ID))
        .bearer_auth(TOKEN)
        .header(header::USER_AGENT, USER_AGENT)
        .send().unwrap()
        .json().unwrap()
}

pub fn get_projects() -> Vec<Project> {
    Client::new()
        .get(format!("{}/{}/api/v2/projects.json", BASE_URL, ORG_ID))
        .bearer_auth(TOKEN)
        .header(header::USER_AGENT, USER_AGENT)
        .send().expect("Unable to retrieve projects")
        .json().expect("Unable to convert project response to json")
}

pub fn get_tasks(project_id: &u32) -> Vec<Task> {
    Client::new()
        .get(format!("{}/{}/api/v2/projects/{}/tasks.json", BASE_URL, ORG_ID, project_id))
        .bearer_auth(TOKEN)
        .header(USER_AGENT, USER_AGENT)
        .send().expect("Unable to retrieve tasks")
        .json().expect("Unable to convert task response to json")
}

pub fn _send_entry() -> reqwest::blocking::Response {
    Client::new()
         .post(format!("{}/{}/api/v2/entries.json", BASE_URL, ORG_ID))
         // .json(serde_json::to_string(&entry).unwrap())
         .bearer_auth(TOKEN)
         .header(USER_AGENT, USER_AGENT)
         .send().expect("Unable to send entry")
 }
