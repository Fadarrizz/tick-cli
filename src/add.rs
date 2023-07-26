use std::process;
use crate::api;
use crate::config::Config;
use crate::files;
use crate::input;
use chrono::{NaiveDate, NaiveTime, Utc};
use tick_cli::{Entry, EntryList, Project, Task};

pub fn add_entry(config: &Config) -> std::io::Result<()> {
    let filename = select_date().format("%Y-%m-%d").to_string();
    let mut entries: EntryList = files::load_entry_list(&filename).expect("Cannot load entries");

    let project = select_project(config);
    let mut task = None;
    if project.is_some() {
        task = select_task(config, &project.as_ref().unwrap().get_id());
    }
    let start_time = input_start_time();
    let notes = input_notes();


    let project_name = match project {
        Some(project) => Some(project.get_name().clone()),
        None => None,
    };
    let (task_id, task_name) = match task {
        Some(task) => (Some(*task.get_id()), Some(task.get_name().clone())),
        None => (None, None)
    };

    // Selecting no means gracefully termination.
    if confirm_entry(&project_name, &task_name, &start_time, &notes) == false {
        return Ok(());
    }

    entries.add(Entry::create(
        project_name,
        task_id,
        task_name,
        start_time,
        None,
        notes,
    ));

    files::store_entry_list(entries, &filename).expect("Cannot store entry list");

    Ok(())
}

fn select_project(config: &Config) -> Option<Project> {
    let projects: Vec<Project> = match api::get_projects(config) {
        Ok(projects) => projects,
        Err(e) => {
            println!("{}", e.message());
            process::exit(1)
        }
    };

    let project_names: Vec<String> = projects.iter().map(|p| p.get_name().clone()).collect();

    match input::fuzzy_select("Select a project", &project_names, None) {
        Some(index) => Some(projects[index].clone()),
        None => None,
    }
}

fn select_task(config: &Config, project_id: &u32) -> Option<Task> {
    let tasks: Vec<Task> = match api::get_tasks(config, project_id) {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("{}", e.message());
            process::exit(1)
        }
    };

    let task_names: Vec<String> = tasks.iter().map(|t| t.get_name().clone()).collect();

    match input::fuzzy_select("Select a task", &task_names, None) {
        Some(index) => Some(tasks[index].clone()),
        None => None,
    }
}

fn select_date() -> NaiveDate {
    let initial_text = Utc::now().format("%Y-%m-%d").to_string();
    
    input::date("Select a date", Some(&initial_text)).unwrap()
}

fn input_start_time() -> NaiveTime {
    input::time("Input start time", None).unwrap()
}

fn _input_end_time() -> NaiveTime {
    input::time("Input start time", None).unwrap()
}

fn input_notes() -> String {
    input::default("Input notes", None)
}

fn confirm_entry(
    project_name: &Option<String>,
    task_name: &Option<String>,
    start_time: &NaiveTime,
    notes: &String,
) -> bool {
    println!("This will create a entry with the following data:");

    let empty_string = String::new();
    let project = match project_name.as_ref() { Some(p) => p, None => &empty_string };
    let task = match task_name.as_ref() { Some(t) => t, None => &empty_string };
    println!("  Project: {}",  project);
    println!("  Task: {}", task);
    println!("  Start Time: {}", start_time);
    println!("  Notes: {}", notes);

    input::confirm("Continue?").unwrap()
}
