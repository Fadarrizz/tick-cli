use core::fmt;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    subscription_id: u32,
    company: String,
    api_token: String,
}

impl Role {
    pub fn get_subscription_id(&self) -> &u32 {
        &self.subscription_id
    }

    pub fn get_company(&self) -> &String {
        &self.company
    }

    pub fn get_api_token(&self) -> &String {
        &self.api_token
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    first_name: String,
}

impl User {
    pub fn get_first_name(&self) -> &String {
        &self.first_name
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    id: u32,
    name: String,
}

impl Project {
    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    id: u32,
    name: String,
}

impl Task {
    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Entry {
    project_name: String,
    task_id: u32,
    task_name: String,
    start_time: NaiveTime,
    end_time: Option<NaiveTime>,
    notes: String,
}

impl Entry {
    pub fn create(
        project_name: String,
        task_id: u32,
        task_name: String,
        start_time: NaiveTime,
        end_time: Option<NaiveTime>,
        notes: String,
    ) -> Self {
        Self {
            project_name,
            task_id,
            task_name,
            start_time,
            end_time,
            notes,
        }
    }

    pub fn get_project_name(&self) -> &String {
        &self.project_name
    }

    pub fn get_task_name(&self) -> &String {
        &self.task_name
    }

    pub fn get_start_time(&self) -> &NaiveTime {
        &self.start_time
    }

    pub fn get_notes(&self) -> &String {
        &self.notes
    }

    pub fn update(
        &mut self,
        project_name: String,
        task_id: u32,
        task_name: String,
        start_time: NaiveTime,
        notes: String,
    ) {
        self.project_name = project_name;
        self.task_id = task_id;
        self.task_name = task_name;
        self.start_time = start_time;
        self.notes = notes;
    }

    pub fn set_end_time(&mut self, end_time: NaiveTime) {
        self.end_time = Some(end_time)
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} | {}", self.start_time.format("%H:%M"), self.project_name, self.notes)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryList {
    entries: Vec<Entry>,
}

impl EntryList {
    pub fn empty() -> Self {
        Self { entries: vec![] }
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }

    pub fn get_all(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn add(&mut self, entry: Entry) {
        self.entries.push(entry);

        self.entries.sort_by(|a, b| a.start_time.cmp(&b.start_time));
    }

    pub fn set_end_times(&mut self) {
        let mut iter = self.entries.iter_mut();

        if let Some(mut current) = iter.next() {
            for next in iter {
                let end_time = next.start_time.clone();
                current.set_end_time(end_time);

                current = next;
            }
        }
    }
}

impl fmt::Display for EntryList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in &self.entries {
            write!(f, "{}\n", entry)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_entries_sorted_on_add() {
        let mut entries = EntryList::empty();

        entries.add(Entry::create(
            "project A".to_string(),
            1,
            "task 1".to_string(),
            NaiveTime::from_str("10:00:00").unwrap(),
            None,
            "notes".to_string(),
        ));

        entries.add(Entry::create(
            "project B".to_string(),
            2,
            "task 2".to_string(),
            NaiveTime::from_str("09:00:00").unwrap(),
            None,
            "notes".to_string(),
        ));

        assert_eq!(NaiveTime::from_str("09:00:00").unwrap(), entries.get_all()[0].start_time);

        entries.add(Entry::create(
            "project C".to_string(),
            3,
            "task 3".to_string(),
            NaiveTime::from_str("08:59:59").unwrap(),
            None,
            "notes".to_string(),
        ));

        assert_eq!(NaiveTime::from_str("08:59:59").unwrap(), entries.get_all()[0].start_time);
    }

    #[test]
    fn test_set_entry_end_times() {
        let mut entries = EntryList::empty();

        entries.add(Entry::create(
            "project A".to_string(),
            1,
            "task 1".to_string(),
            NaiveTime::from_str("10:00:00").unwrap(),
            None,
            "notes".to_string(),
        ));

        entries.add(Entry::create(
            "project B".to_string(),
            2,
            "task 2".to_string(),
            NaiveTime::from_str("09:00:00").unwrap(),
            None,
            "notes".to_string(),
        ));

        assert_eq!(None, entries.get_all()[0].end_time);
        assert_eq!(None, entries.get_all()[1].end_time);

        entries.set_end_times();

        let expected = Some(NaiveTime::from_str("10:00:00").unwrap());
        assert_eq!(expected, entries.get_all()[0].end_time);
        assert_eq!(None, entries.get_all()[1].end_time);
    }
}
