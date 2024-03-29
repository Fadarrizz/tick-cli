use std::process;
use crate::{api, ui, files};
use crate::config::Config;
use crate::repository;
use chrono::{NaiveDate, NaiveTime, Utc};
use tick_cli::{Entry, EntryList, Project, Task};

pub fn add_entry(config: &Config) -> std::io::Result<()> {
    let filename = select_date().format("%Y-%m-%d").to_string();
    let path = files::get_document_file_path_from(&filename).expect("Cannot convert filename to path");
    let mut entries: EntryList = repository::load_entry_list(&path).expect("Cannot load entries");

    let project = select_project(config);
    let mut task = None;
    if project.is_some() {
        task = select_task(config, &project.as_ref().unwrap().get_id());
    }
    let start_time = input_start_time();
    let end_time = input_end_time();
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
    if confirm_entry(&project_name, &task_name, &start_time, end_time.as_ref(), &notes).is_none() {
        return Ok(());
    }

    entries.add(Entry::create(
        project_name,
        task_id,
        task_name,
        start_time,
        end_time,
        notes,
    ));

    repository::store_entry_list(&entries, &path).expect("Cannot store entry list");

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

    match ui::fuzzy_select("Select a project", &project_names, Some(0), true) {
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

    match ui::fuzzy_select("Select a task", &task_names, Some(0), true) {
        Some(index) => Some(tasks[index].clone()),
        None => None,
    }
}

fn select_date() -> NaiveDate {
    let initial_text = Utc::now().format("%Y-%m-%d").to_string();
    
    ui::date("Select a date", Some(&initial_text)).unwrap()
}

fn input_start_time() -> NaiveTime {
    ui::time("Input start time", None, false).unwrap()
}

fn input_end_time() -> Option<NaiveTime> {
    ui::time("Input end time", None, true)
}

fn input_notes() -> String {
    ui::default("Input notes", None)
}

fn confirm_entry(
    project_name: &Option<String>,
    task_name: &Option<String>,
    start_time: &NaiveTime,
    end_time: Option<&NaiveTime>,
    notes: &String,
) -> Option<bool> {
    println!("This will add an entry with the following data:");

    let empty_string = String::new();
    let project = match project_name.as_ref() { Some(p) => p, None => &empty_string };
    let task = match task_name.as_ref() { Some(t) => t, None => &empty_string };
    let formatted_end_time = match end_time.as_ref() { 
        Some(e) => e.format("%H:%M").to_string(),
        None => empty_string.clone()
    };

    println!("  Project: {}",  project);
    println!("  Task: {}", task);
    println!("  Start Time: {}", start_time.format("%H:%M"));
    println!("  End Time: {}", &formatted_end_time);
    println!("  Notes: {}", notes);

    ui::confirm("Continue?")
}
