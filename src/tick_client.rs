use reqwest::{blocking::Client, header::USER_AGENT};
use tick_cli::{Project, Task};

const BASE_URL: &str = "https://secure.tickspot.com";
const ORG_ID: &str = "45669";
const TOKEN: &str = "d115b93153b4f9968214865e96ff7789";

pub fn get_projects() -> Vec<Project> {
    Client::new()
        .get(format!("{}/{}/api/v2/projects.json", BASE_URL, ORG_ID))
        .bearer_auth(TOKEN)
        .header(USER_AGENT, "tick-cli (auke@ijsfontein.nl)")
        .send().expect("Unable to retrieve projects")
        .json().expect("Unable to convert project response to json")
}

pub fn get_tasks(project_id: &u32) -> Vec<Task> {
    Client::new()
        .get(format!("{}/{}/api/v2/projects/{}/tasks.json", BASE_URL, ORG_ID, project_id))
        .bearer_auth(TOKEN)
        .header(USER_AGENT, "tick-cli (auke@ijsfontein.nl)")
        .send().expect("Unable to retrieve tasks")
        .json().expect("Unable to convert task response to json")
}

pub fn _send_entry() -> reqwest::blocking::Response {
    Client::new()
         .post(format!("{}/{}/api/v2/entries.json", BASE_URL, ORG_ID))
         // .json(serde_json::to_string(&entry).unwrap())
         .bearer_auth(TOKEN)
         .header(USER_AGENT, "tick-cli (auke@ijsfontein.nl)")
         .send().expect("Unable to send entry")
 }
