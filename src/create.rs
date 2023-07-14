use dialoguer::{console, theme::ColorfulTheme, FuzzySelect, Input, Confirm};
use chrono::{NaiveTime, NaiveDate, Utc};
use tick_cli::{Project, Task, Entry, EntryList};
use crate::config::Config;
use crate::files;
use crate::api;

pub fn create_entry(config: &Config) -> std::io::Result<()> {
    let filename = select_date().format("%Y-%m-%d").to_string();
    let mut entries: EntryList = files::load_entry_list(&filename).expect("Cannot load entries");

    let project = select_project(config).unwrap();
    let task = select_task(config, &project.get_id()).unwrap();
    let start_time = input_start_time();
    let notes = input_notes().unwrap();

    // Selecting no means gracefully termination.
    if confirm_entry(&project.get_name(), &task.get_name(), &start_time, &notes) == false {
        return Ok(())
    }

    entries.add(Entry::create(
        project.get_name().clone(),
        *task.get_id(),
        task.get_name().clone(),
        start_time,
        None,
        notes.to_owned(),
    ));
    
    files::store_entry_list(entries, &filename).expect("Cannot store entry list");

    Ok(())
}

fn select_project(config: &Config) -> Option<Project> {
    let projects: Vec<Project> = api::get_projects(config);

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

fn select_task(config: &Config, project_id: &u32) -> Option<Task> {
    let tasks: Vec<Task> = api::get_tasks(config, project_id);

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
