use tick_cli::{Project, Role, Task, TickEntry, User};
use crate::config::Config;
use crate::http::{self, HttpError};

const BASE_URL: &str = "https://secure.tickspot.com";
const API_VERSION: &str = "v2";

pub fn get_roles(email: &String, password: &String) -> Result<Vec<Role>, HttpError> {
    let url = format!("{}/api/{}/roles.json", BASE_URL, API_VERSION);

    http::get_basic_auth((email, password), &url)
}

pub fn get_users(config: &Config) -> Result<Vec<User>, HttpError> {
    let url = format!(
        "{}/{}/api/{}/users.json",
        BASE_URL,
        config.get_subscription_id(),
        API_VERSION,
    );

    http::get(config, &url)
}

pub fn get_projects(config: &Config) -> Result<Vec<Project>, HttpError> {
    let url = format!(
        "{}/{}/api/{}/projects.json",
        BASE_URL,
        config.get_subscription_id(),
        API_VERSION,
    );

    http::get(config, &url)
}

pub fn get_tasks(config: &Config, project_id: &u32) -> Result<Vec<Task>, HttpError> {
    let url = format!(
        "{}/{}/api/{}/projects/{}/tasks.json",
        BASE_URL,
        config.get_subscription_id(),
        API_VERSION,
        project_id
    );

    http::get(config, &url)
}

pub fn create_entry(config: &Config, entry: &TickEntry) -> Result<TickEntry, HttpError> {
    let url = format!(
        "{}/{}/api/{}/entries.json",
        BASE_URL,
        config.get_subscription_id(),
        API_VERSION,
    );

    http::post(config, &url, entry)
}

pub fn update_entry(config: &Config, entry: &TickEntry) -> Result<TickEntry, HttpError> {
    let url = format!(
        "{}/{}/api/{}/entries/{}.json",
        BASE_URL,
        config.get_subscription_id(),
        API_VERSION,
        entry.get_id().unwrap(),
    );

    http::put(config, &url, entry)
}

pub fn delete_entry(config: &Config, id: u32) -> Result<TickEntry, HttpError> {
    let url = format!(
        "{}/{}/api/{}/entries/{}.json",
        BASE_URL,
        config.get_subscription_id(),
        API_VERSION,
        id,
    );

    http::delete(config, &url)
}
