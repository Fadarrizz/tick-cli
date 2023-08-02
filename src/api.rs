use std::process;

use reqwest::{blocking::{Client, Response}, header};
use serde::{de::DeserializeOwned, Serialize};
use tick_cli::{Project, Role, Task, TickEntry, User};
use crate::{config::Config, cache::{self, CachedResponse, Cache}};

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
    let url = format!("{}/api/v2/roles.json", BASE_URL);

    get_basic_auth((email, password), &url)
}

pub fn get_users(config: &Config) -> Result<Vec<User>, ApiError> {
    let url = format!(
        "{}/{}/api/v2/users.json",
        BASE_URL,
        config.get_subscription_id()
    );

    get(config, &url)
}

pub fn get_projects(config: &Config) -> Result<Vec<Project>, ApiError> {
    let url = format!(
        "{}/{}/api/v2/projects.json",
        BASE_URL,
        config.get_subscription_id()
    );

    get(config, &url)
}

pub fn get_tasks(config: &Config, project_id: &u32) -> Result<Vec<Task>, ApiError> {
    let url = format!(
        "{}/{}/api/v2/projects/{}/tasks.json",
        BASE_URL,
        config.get_subscription_id(),
        project_id
    );

    get(config, &url)
}

pub fn create_entry(config: &Config, entry: &TickEntry) -> Result<TickEntry, ApiError> {
    let url = format!(
        "{}/{}/api/v2/entries.json",
        BASE_URL,
        config.get_subscription_id(),
    );

    post(config, &url, entry)
}

pub fn update_entry(config: &Config, entry: &TickEntry) -> Result<TickEntry, ApiError> {
    let url = format!(
        "{}/{}/api/v2/entries/{}.json",
        BASE_URL,
        config.get_subscription_id(),
        entry.get_id().unwrap(),
    );

    put(config, &url, entry)
}

pub fn delete_entry(config: &Config, id: u32) -> Result<TickEntry, ApiError> {
    let url = format!(
        "{}/{}/api/v2/entries/{}.json",
        BASE_URL,
        config.get_subscription_id(),
        id,
    );

    delete(config, &url)
}

fn get_basic_auth<T: DeserializeOwned + Serialize + Clone>(credentials: (&String, &String), url: &String) -> Result<T, ApiError> {
    call(None, "GET", url, None, false, Some(credentials))
}

fn get<T: DeserializeOwned + Serialize + Clone>(config: &Config, url: &String) -> Result<T, ApiError> {
    call(Some(config), "GET", url, None, true, None)
}

fn post<T: DeserializeOwned + Serialize + Clone>(config: &Config, url: &String, body: &T) -> Result<T, ApiError> {
    call(Some(config), "POST", url, Some(body), false, None)
}

fn put<T: DeserializeOwned + Serialize + Clone>(config: &Config, url: &String, body: &T) -> Result<T, ApiError> {
    call(Some(config), "PUT", url, Some(body), false, None)
}

fn delete<T: DeserializeOwned + Serialize + Clone>(config: &Config, url: &String) -> Result<T, ApiError> {
    call(Some(config), "DELETE", url, None, false, None)
}

fn call<T: DeserializeOwned + Serialize + Clone>(
    config: Option<&Config>,
    method: &str,
    url: &String,
    body: Option<&T>,
    should_cache: bool,
    basic_auth: Option<(&String, &String)>
) -> Result<T, ApiError> {
    let client = match method {
        "GET" => Client::new().get(url),
        "POST" => Client::new().post(url),
        "PUT" => Client::new().put(url),
        "DELETE" => Client::new().delete(url),
        _ => panic!("Unknown http method encountered."),
    };
        
    let mut client = match config {
        Some(config) => client.bearer_auth(config.get_api_key()),
        None => match basic_auth {
            Some((username, password)) => client.basic_auth(username, Some(password)),
            None => client,
        },
    };
        
    client = client.header(header::USER_AGENT, USER_AGENT);

    let mut cache = cache::Cache::<T>::new();

    if let Some(cached_response) = cache.get(url.clone()) {
        if let Some(etag) = cached_response.get_etag() {
            client = client.header(header::IF_NONE_MATCH, etag);
        }
        if let Some(last_modified) = cached_response.get_last_modified() {
            client = client.header(header::IF_MODIFIED_SINCE, last_modified);
        }
    }

    if let Some(body) = body {
        client = client.json(body);
    }

    let response = client.send();

    if response.is_err()  {
        println!("Error connecting to Tickspot.\nPlease check your internet connection.");
        process::exit(1);
    } 

    let response = response.unwrap();

    match response.status().as_u16() {
        304 => {
            let cached_response = cache.get(url.clone()).unwrap();

            return Ok(cached_response.get_json().clone());
        },
        200..=299 => {
            let json: T;
            if should_cache {
                json = cache_response(&mut cache, response);
            } else {
                json = response.json().expect("Unable to convert response to json");
            }

            Ok(json)
        },
        400..=499 => Err(ApiError {
            code: response.status().as_u16(),
            message: response.text().unwrap(),
        }),
        _ => {
            panic!("Unexpected status code: {}", response.status());
        }
    }
}

fn cache_response<T: DeserializeOwned + Serialize + Clone>(cache: &mut Cache<T>, response: Response) -> T {
    let etag = response.headers().get(header::ETAG)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    let last_modified = response.headers().get(header::LAST_MODIFIED)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    let url = response.url().clone();

    let json: T = response.json().expect("Unable to convert response to json");

    cache.set(
        url.to_string(),
        CachedResponse::new(etag, last_modified, json.clone())
    );

    json
}
