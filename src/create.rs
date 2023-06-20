use std::io::ErrorKind;
use dialoguer::{console, theme::ColorfulTheme, FuzzySelect, Input, Confirm};
use chrono::{NaiveTime, NaiveDate, Utc};
use reqwest::{blocking::Client, header::USER_AGENT};
use tick_cli::{Project, Task, Entry, EntryList};

const BASE_URL: &str = "https://secure.tickspot.com";
const ORG_ID: &str = "45669";
const TOKEN: &str = "d115b93153b4f9968214865e96ff7789";
const BASE_DIR: &str = "/Users/auke/Documents/Tick";

pub fn create_entry() -> std::io::Result<()> {
    let http_client = Client::new();
    let filename = select_date().format("%Y-%m-%d").to_string();
    let file_path = get_file_path(&filename);
    let mut entries = get_entries(&file_path);

    let project = select_project(&http_client).unwrap();
    let task = select_task(&http_client, &project.get_id()).unwrap();
    let start_time = input_start_time();
    let notes = input_notes().unwrap();

    // Selecting no means gracefully termination.
    if confirm_entry(&project.get_name(), &task.get_name(), &start_time, &notes) == false {
        return Ok(())
    }

    entries.add(Entry::create(
        *task.get_id(),
        start_time,
        None,
        notes.to_owned(),
    ));

    std::fs::write(
        file_path,
        serde_json::to_string_pretty(&entries).expect("Cannot serialize entries"),
    ).expect("Cannot write to file");

    Ok(())
}

fn select_project(http_client: &Client) -> Option<Project> {
    let projects: Vec<Project> = http_client
        .get(format!("{}/{}/api/v2/projects.json", BASE_URL, ORG_ID))
        .bearer_auth(TOKEN)
        .header(USER_AGENT, "tick-cli (auke@ijsfontein.nl)")
        .send()
        .unwrap()
        .json()
        .unwrap();

    let project_names: Vec<String> = projects.iter().map(|p| p.get_name().clone()).collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a project")
        .items(&project_names)
        .interact_opt()
        .unwrap();

    match selection {
        Some(index) => Some(projects[index].clone()),
        None => panic!("Nothing selected"),
    }
}

fn select_task(http_client: &Client, project_id: &u32) -> Option<Task> {
    let tasks: Vec<Task> = http_client
        .get(format!(
            "{}/{}/api/v2/projects/{}/tasks.json",
            BASE_URL, ORG_ID, project_id
        ))
        .bearer_auth(TOKEN)
        .header(USER_AGENT, "tick-cli (auke@ijsfontein.nl)")
        .send().expect("Unable to retrieve tasks")
        .json().expect("Unable to convert task response to json");

    let task_names: Vec<String> = tasks.iter().map(|t| t.get_name().clone()).collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a task")
        .default(0)
        .items(&task_names)
        .interact_on_opt(&console::Term::stderr())
        .unwrap();

    match selection {
        Some(index) => Some(tasks[index].clone()),
        None => panic!("Nothing selected"),
    }
}

fn select_date() -> NaiveDate {
    let date: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a date")
        .with_initial_text(format!("{}", Utc::now().format("%Y-%m-%d")))
        .validate_with(|input: &String| -> Result<(), &str> {
            let date = NaiveDate::parse_from_str(&input, "%Y-%m-%d");
            if date.is_ok() {
                Ok(())
            } else {
                Err("Wrong date format. Please provide a date like 2023-01-30")
            }
        })
        .interact()
        .unwrap();

    NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap()
}

fn input_start_time() -> NaiveTime {
    let time = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Input start time")
        .validate_with(|input: &String| -> Result<(), &str> {
            let time = NaiveTime::parse_from_str(&input, "%H:%M");
            if time.is_ok() {
                Ok(())
            } else {
                Err("Wrong time format. Please provide a time like 9:15")
            }
        })
        .interact()
        .unwrap();

    NaiveTime::parse_from_str(&time, "%H:%M").unwrap()
}

fn input_notes() -> Result<String, std::io::Error> {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Input notes")
        .interact()
}

fn get_file_path(filename: &String) -> String {
    format!("{}/{}.json", BASE_DIR, filename)
}

fn get_entries(file_path: &String) -> EntryList {
    let data = std::fs::read_to_string(&file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            String::new()
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    if data.is_empty() {
        return EntryList::empty();
    } 
    
    return serde_json::from_str(data.as_str()).expect("Unable to parse file to json");
}

fn confirm_entry(project_name: &str, task_name: &str, start_time: &NaiveTime, notes: &String) -> bool {
    println!("This will create a entry with the following data:");

    println!("Project: {}", project_name);
    println!("Task: {}", task_name);
    println!("Start Time: {}", start_time);
    println!("Notes: {}", notes);

    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Continue?")
        .default(true)
        .wait_for_newline(true)
        .interact()
        .unwrap()
}
