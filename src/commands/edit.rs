use std::process;
use chrono::NaiveTime;
use tick_cli::{Entry, EntryList, Project, Task};
use crate::{api, config::Config, repository, ui};

pub fn edit_entry(config: &Config) -> std::io::Result<()> {
    let path = ui::select_file();
    let mut entries: EntryList = repository::load_entry_list(&path).expect("Cannot load entries");

    let entry = select_entry(&mut entries).unwrap();

    let project = select_project(config, entry.get_project_name());
    let mut task = None;
    if project.is_some() {
        task = select_task(
            config,
            &project.as_ref().unwrap().get_id(),
            entry.get_task_name(),
        );
    }
    let start_time = input_start_time(entry.get_start_time());
    let end_time = input_end_time(entry.get_end_time());
    let notes = input_notes(entry.get_notes());

    let project_name = match project {
        Some(project) => Some(project.get_name().clone()),
        None => None,
    };
    let (task_id, task_name) = match task {
        Some(task) => (Some(*task.get_id()), Some(task.get_name().clone())),
        None => (None, None),
    };

    // Selecting no means gracefully termination.
    if confirm_entry(&project_name, &task_name, &start_time, end_time.as_ref(), &notes).is_none() {
        return Ok(());
    }

    entry.update(
        project_name,
        task_id,
        task_name,
        start_time,
        end_time,
        notes,
    );

    entries.sort();
    entries.set_all_submitted(false);

    repository::store_entry_list(&entries, &path).expect("Cannot store entry list");

    Ok(())
}

fn select_entry(entry_list: &mut EntryList) -> Option<&mut Entry> {
    let entries = entry_list.get_all();

    match ui::fuzzy_select("Select an entry", entries, Some(0), false) {
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

    match ui::fuzzy_select("Select a project", &project_names, selected_index, true) {
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

    match ui::fuzzy_select("Select a task", &task_names, selected_index, true) {
        Some(index) => Some(tasks[index].clone()),
        None => None,
    }
}

fn input_start_time(start_time: &NaiveTime) -> NaiveTime {
    let initial = start_time.format("%H:%M").to_string();

    ui::time("Input start time", Some(&initial), false).unwrap()
}

fn input_end_time(end_time: Option<&NaiveTime>) -> Option<NaiveTime> {
    let mut formatted = String::new();
    if end_time.is_some() {
        formatted = end_time.unwrap().format("%H:%M").to_string();
    }

    let mut initial = None;
    if formatted.is_empty() {
        initial = Some(&formatted);
    }

    ui::time("Input end time", initial, true)
}

fn input_notes(notes: &String) -> String {
    ui::default("Input notes", Some(notes))
}

fn confirm_entry(
    project_name: &Option<String>,
    task_name: &Option<String>,
    start_time: &NaiveTime,
    end_time: Option<&NaiveTime>,
    notes: &String,
) -> Option<bool> {
    println!("This will update the entry with the following data:");

    let empty_string = String::new();
    let project = match project_name.as_ref() { Some(p) => p, None => &empty_string };
    let task = match task_name.as_ref() { Some(t) => t, None => &empty_string };
    let formatted_end_time = match end_time.as_ref() { 
        Some(e) => e.format("%H:%M").to_string(),
        None => empty_string.clone()
    };

    println!("  Project: {}", project);
    println!("  Task: {}", task);
    println!("  Start Time: {}", start_time.format("%H:%M"));
    println!("  End Time: {}", &formatted_end_time);
    println!("  Notes: {}", notes);

    ui::confirm("Continue?")
}
