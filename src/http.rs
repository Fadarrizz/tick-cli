use crate::{
    cache::{Cache, CachedResponse},
    config::Config,
};
use reqwest::{
    blocking::{Client, RequestBuilder, Response},
    Method,
};
use serde::{de::DeserializeOwned, Serialize};
use std::process;
use reqwest::header::{self};

const USER_AGENT: &str = "tick-cli (auke@ijsfontein.nl)";

#[derive(Debug)]
pub struct HttpError {
    code: u16,
    message: String,
}

impl HttpError {
    pub fn is_unauthenticated_error(&self) -> bool {
        self.code == 401
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

pub fn get_basic_auth<T: DeserializeOwned + Serialize + Clone>(
    credentials: (&String, &String),
    url: &String,
) -> Result<T, HttpError> {
    call(None, Method::GET, url, None, false, Some(credentials))
}

pub fn get<T: DeserializeOwned + Serialize + Clone>(
    config: &Config,
    url: &String,
) -> Result<T, HttpError> {
    call(Some(config), Method::GET, url, None, true, None)
}

pub fn post<T: DeserializeOwned + Serialize + Clone>(
    config: &Config,
    url: &String,
    body: &T,
) -> Result<T, HttpError> {
    call(Some(config), Method::POST, url, Some(body), false, None)
}

pub fn put<T: DeserializeOwned + Serialize + Clone>(
    config: &Config,
    url: &String,
    body: &T,
) -> Result<T, HttpError> {
    call(Some(config), Method::PUT, url, Some(body), false, None)
}

pub fn delete<T: DeserializeOwned + Serialize + Clone>(
    config: &Config,
    url: &String,
) -> Result<T, HttpError> {
    call(Some(config), Method::DELETE, url, None, false, None)
}

fn call<T: DeserializeOwned + Serialize + Clone>(
    config: Option<&Config>,
    method: Method,
    url: &String,
    body: Option<&T>,
    should_cache: bool,
    credentials: Option<(&String, &String)>,
) -> Result<T, HttpError> {
    let mut cache = Cache::<T>::new();

    let mut request = construct_request(method, url);
    request = enable_auth(config, credentials, request);
    request = set_headers(request, &cache, url);

    if let Some(body) = body {
        request = request.json(body);
    }

    let response = request.send();

    if response.is_err() {
        println!("Error connecting to Tickspot.\nPlease check your internet connection.");
        process::exit(1);
    }

    let cache_option = if should_cache {
        Some(&mut cache)
    } else {
        None
    };

    handle_response(response.unwrap(), cache_option)
}

fn construct_request(method: Method, url: &String) -> RequestBuilder {
    Client::new().request(method, url)
}

fn enable_auth(
    config: Option<&Config>,
    credentials: Option<(&String, &String)>,
    client: RequestBuilder,
) -> RequestBuilder {
    match config {
        Some(config) => client.bearer_auth(config.get_api_key()),
        None => match credentials {
            Some((username, password)) => client.basic_auth(username, Some(password)),
            None => client,
        },
    }
}

fn set_headers<T: DeserializeOwned + Serialize + Clone>(
    request: RequestBuilder,
    cache: &Cache<T>,
    url: &String,
) -> RequestBuilder {
    let mut _request = request.header(header::USER_AGENT, USER_AGENT);

    if let Some(cached_response) = cache.get(url.clone()) {
        if let Some(etag) = cached_response.get_etag() {
            _request = _request.header(header::IF_NONE_MATCH, etag);
        }
        if let Some(last_modified) = cached_response.get_last_modified() {
            _request = _request.header(header::IF_MODIFIED_SINCE, last_modified);
        }
    }

    _request
}

fn handle_response<T: DeserializeOwned + Serialize + Clone>(
    response: Response,
    cache_option: Option<&mut Cache<T>>,
) -> Result<T, HttpError> {
    match response.status().as_u16() {
        304 => {
            let cached_response = cache_option.unwrap().get(response.url().to_string().clone()).unwrap();

            return Ok(cached_response.get_json().clone());
        }
        200..=299 => {
            let json: T;
            if let Some(cache) = cache_option {
                json = cache_response(cache, response);
            } else {
                json = response.json().expect("Unable to convert response to json");
            }

            Ok(json)
        }
        400..=499 => Err(HttpError {
            code: response.status().as_u16(),
            message: response.text().unwrap(),
        }),
        _ => {
            panic!("Unexpected status code: {}", response.status());
        }
    }
}

fn cache_response<T: DeserializeOwned + Serialize + Clone>(
    cache: &mut Cache<T>,
    response: Response,
) -> T {
    let etag = response
        .headers()
        .get(header::ETAG)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    let last_modified = response
        .headers()
        .get(header::LAST_MODIFIED)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    let url = response.url().clone();

    let json: T = response.json().expect("Unable to convert response to json");

    cache.set(
        url.to_string(),
        CachedResponse::new(etag, last_modified, json.clone()),
    );

    json
}
