use reqwest::{blocking::Client, header};
use serde::de::DeserializeOwned;
use tick_cli::{Project, Role, Task, TickEntry, User};

use crate::config::Config;

const BASE_URL: &str = "https://secure.tickspot.com";
const USER_AGENT: &str = "tick-cli (auke@ijsfontein.nl)";

#[derive(Debug)]
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
        .send()
        .unwrap();

    to_result(response)
}

pub fn get_users(config: &Config) -> Result<Vec<User>, ApiError> {
    let response = Client::new()
        .get(format!(
            "{}/{}/api/v2/users.json",
            BASE_URL,
            config.get_subscription_id()
        ))
        .bearer_auth(config.get_api_key())
        .header(header::USER_AGENT, USER_AGENT)
        .send()
        .unwrap();

    to_result(response)
}

pub fn get_projects(config: &Config) -> Result<Vec<Project>, ApiError> {
    let response = Client::new()
        .get(format!(
            "{}/{}/api/v2/projects.json",
            BASE_URL,
            config.get_subscription_id()
        ))
        .bearer_auth(config.get_api_key())
        .header(header::USER_AGENT, USER_AGENT)
        .send()
        .expect("Unable to retrieve projects");

    to_result(response)
}

pub fn get_tasks(config: &Config, project_id: &u32) -> Result<Vec<Task>, ApiError> {
    let response = Client::new()
        .get(format!(
            "{}/{}/api/v2/projects/{}/tasks.json",
            BASE_URL,
            config.get_subscription_id(),
            project_id
        ))
        .bearer_auth(config.get_api_key())
        .header(header::USER_AGENT, USER_AGENT)
        .send()
        .expect("Unable to retrieve tasks");

    to_result(response)
}

pub fn create_entry(config: &Config, entry: &TickEntry) -> Result<TickEntry, ApiError> {
    let response = Client::new()
        .post(format!("{}/{}/api/v2/entries.json", BASE_URL, config.get_subscription_id()))
        .bearer_auth(config.get_api_key())
        .header(header::USER_AGENT, USER_AGENT)
        .json(&entry)
        .send().expect("Unable to create entry");

    to_result(response)
}

pub fn update_entry(config: &Config, entry: &TickEntry) -> Result<TickEntry, ApiError> {
    let response = Client::new()
        .put(format!(
            "{}/{}/api/v2/entries/{}.json",
            BASE_URL,
            config.get_subscription_id(),
            entry.get_id().unwrap(),
        ))
        .bearer_auth(config.get_api_key())
        .header(header::USER_AGENT, USER_AGENT)
        .json(&entry)
        .send().expect("Unable to update entry");

    to_result(response)
}

pub fn delete_entry(config: &Config, id: u32) -> Result<TickEntry, ApiError> {
    let response = Client::new()
        .delete(format!("{}/{}/api/v2/entries/{}.json", BASE_URL, config.get_subscription_id(), id))
        .bearer_auth(config.get_api_key())
        .header(header::USER_AGENT, USER_AGENT)
        .send().expect("Unable to delete entry");

    to_result(response)
}

fn to_result<T: DeserializeOwned>(response: reqwest::blocking::Response) -> Result<T, ApiError> {
    match response.status().as_u16() {
        200..=299 => {
            return Ok(response.json().expect("Unable to convert response to json"));
        }
        400..=499 => Err(ApiError {
            code: response.status().as_u16(),
            message: response.text().unwrap(),
        }),
        _ => {
            panic!("Unexpected status code: {}", response.status());
        }
    }
}
