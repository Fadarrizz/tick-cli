use std::process;

use chrono::NaiveTime;
use tick_cli::{Entry, EntryList, Project, Task};

use crate::{api, config::Config, files, input};

pub fn edit_entry(config: &Config) -> std::io::Result<()> {
    let filename = select_file().unwrap();
    let mut entries: EntryList = files::load_entry_list(&filename).expect("Cannot load entries");

    let entry = select_entry(&mut entries).unwrap();

    let project = select_project(config, entry.get_project_name());
    let mut task = None;
    if project.is_some() {
        task = select_task(config, &project.as_ref().unwrap().get_id(), entry.get_task_name());
    }
    let start_time = input_start_time(entry.get_start_time());
    let notes = input_notes(entry.get_notes());

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

    entry.update(
        project_name,
        task_id,
        task_name,
        start_time,
        notes,
    );

    entries.sort();

    files::store_entry_list(entries, &filename).expect("Cannot store entry list");

    Ok(())
}

fn select_file() -> Option<String> {
    let existing_files = files::get_existing_file_names();

    match input::fuzzy_select("Select a file", &existing_files, None) {
        Some(index) => Some(existing_files[index].clone()),
        None => None,
    }
}

fn select_entry(entry_list: &mut EntryList) -> Option<&mut Entry> {
    let entries = entry_list.get_all();

    match input::fuzzy_select("Select an entry", entries, Some(0)) {
        Some(index) => Some(entry_list.get_mut(index)),
        None => panic!("Nothing selected"),
    }
}

fn select_project(config: &Config, selected: Option<&String>) -> Option<Project> {
    let projects: Vec<Project> = match api::get_projects(config) {
        Ok(projects) => projects,
        Err(e) => {
            println!("{}", e.message());
            process::exit(1)
        }
    };

    let project_names: Vec<String> = projects.iter().map(|p| p.get_name().clone()).collect();
    let mut selected_index = None;
    if selected.is_some() {
        selected_index = project_names.iter().position(|n| n == selected.unwrap());
    }

    match input::fuzzy_select("Select a project", &project_names, selected_index) {
        Some(index) => Some(projects[index].clone()),
        None => None,
    }
}

fn select_task(config: &Config, project_id: &u32, selected: Option<&String>) -> Option<Task> {
    let tasks: Vec<Task> = match api::get_tasks(config, project_id) {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("{}", e.message());
            process::exit(1)
        }
    };

    let task_names: Vec<String> = tasks.iter().map(|t| t.get_name().clone()).collect();
    let mut selected_index = None;
    if selected.is_some() {
        selected_index = task_names.iter().position(|n| n == selected.unwrap());
    }

    match input::fuzzy_select("Select a task", &task_names, selected_index) {
        Some(index) => Some(tasks[index].clone()),
        None => None,
    }
}

fn input_start_time(start_time: &NaiveTime) -> NaiveTime {
    let initial = start_time.format("%H:%M").to_string();

    input::time("Input start time", Some(&initial)).unwrap()
}

fn input_notes(notes: Option<&String>) -> Option<String> {
    input::default("Input notes", notes)
}

fn confirm_entry(
    project_name: &Option<String>,
    task_name: &Option<String>,
    start_time: &NaiveTime,
    notes: &Option<String>,
) -> bool {
    println!("This will update the entry with the following data:");

    println!("  Project: {}", project_name.as_ref().unwrap());
    println!("  Task: {}", task_name.as_ref().unwrap());
    println!("  Start Time: {}", start_time);
    println!("  Notes: {}", notes.as_ref().unwrap());

    input::confirm("Continue?").unwrap()
}
