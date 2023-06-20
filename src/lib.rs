use chrono::NaiveTime;
use serde::{Serialize, Deserialize};

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

    pub fn get_name(&self) -> &String{
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    task_id: u32,
    start_time: NaiveTime,
    end_time: Option<NaiveTime>,
    notes: String,
}

impl Entry {
    pub fn create(
        task_id: u32,
        start_time: NaiveTime,
        end_time: Option<NaiveTime>,
        notes: String,
    ) -> Self {
        Self {
            task_id,
            start_time,
            end_time,
            notes
        }
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

    pub fn add(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}
