use chrono::{NaiveDate, NaiveTime, Utc};
use tick_cli::{Entry, EntryList, Project, Task};

use crate::{api, config::Config, files, input};

pub fn edit_entry(config: &Config) -> std::io::Result<()> {
    // Choose an existing file, instead of filling out date.
    let filename = select_date().format("%Y-%m-%d").to_string();
    let mut entries: EntryList = files::load_entry_list(&filename).expect("Cannot load entries");

    let entry = select_entry(&mut entries).unwrap();

    let project = select_project(config, entry.get_project_name()).unwrap();
    let task = select_task(config, &project.get_id(), entry.get_task_name()).unwrap();
    let start_time = input_start_time(entry.get_start_time());
    let notes = input_notes(entry.get_notes());

    // Selecting no means gracefully termination.
    if confirm_entry(&project.get_name(), &task.get_name(), &start_time, &notes) == false {
        return Ok(());
    }

    entry.update(
        project.get_name().clone(),
        task.get_id().clone(),
        task.get_name().clone(),
        start_time,
        notes,
    );

    Ok(())
}

fn select_entry(entry_list: &mut EntryList) -> Option<&mut Entry> {
    let entries = entry_list.get_all();

    match input::fuzzy_select("Select an entry", entries, None) {
        Some(index) => Some(entry_list.get_mut(index)),
        None => panic!("Nothing selected"),
    }
}

fn select_project(config: &Config, selected: &String) -> Option<Project> {
    let projects: Vec<Project> = api::get_projects(config);

    let project_names: Vec<String> = projects.iter().map(|p| p.get_name().clone()).collect();
    let selected_index: Option<usize> = project_names.iter().position(|n| n == selected);

    match input::fuzzy_select("Select a project", &project_names, selected_index) {
        Some(index) => Some(projects[index].clone()),
        None => panic!("Nothing selected"),
    }
}

fn select_task(config: &Config, project_id: &u32, selected: &String) -> Option<Task> {
    let tasks: Vec<Task> = api::get_tasks(config, project_id);

    let task_names: Vec<String> = tasks.iter().map(|t| t.get_name().clone()).collect();
    let selected_index: Option<usize> = task_names.iter().position(|n| n == selected);

    match input::fuzzy_select("Select a task", &task_names, selected_index) {
        Some(index) => Some(tasks[index].clone()),
        None => panic!("Nothing selected"),
    }
}

fn select_date() -> NaiveDate {
    let initial_text = Utc::now().format("%Y-%m-%d").to_string();
    let date = input::date("Select a date", Some(&initial_text)).unwrap();

    NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap()
}

fn input_start_time(start_time: &NaiveTime) -> NaiveTime {
    let initial = start_time.format("%H:%M").to_string();
    let time = input::time("Input start time", Some(&initial)).unwrap();

    NaiveTime::parse_from_str(&time, "%H:%M").unwrap()
}

fn input_notes(notes: &String) -> String {
    input::default("Input notes", Some(notes)).unwrap()
}

fn confirm_entry(
    project_name: &str,
    task_name: &str,
    start_time: &NaiveTime,
    notes: &String,
) -> bool {
    println!("This will update the entry with the following data:");

    println!("Project: {}", project_name);
    println!("Task: {}", task_name);
    println!("Start Time: {}", start_time);
    println!("Notes: {}", notes);

    input::confirm("Continue?").unwrap()
}
