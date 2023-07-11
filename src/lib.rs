use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

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
            notes,
        }
    }

    pub fn set_end_time(&mut self, end_time: NaiveTime) {
        self.end_time = Some(end_time)
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_entries_sorted_on_add() {
        let mut entries = EntryList::empty();

        entries.add(Entry::create(
            1,
            NaiveTime::from_str("10:00:00").unwrap(),
            None,
            "notes".to_string(),
        ));

        entries.add(Entry::create(
            2,
            NaiveTime::from_str("09:00:00").unwrap(),
            None,
            "notes".to_string(),
        ));

        assert_eq!(NaiveTime::from_str("09:00:00").unwrap(), entries.get_all()[0].start_time);

        entries.add(Entry::create(
            3,
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
            1,
            NaiveTime::from_str("10:00:00").unwrap(),
            None,
            "notes".to_string(),
        ));

        entries.add(Entry::create(
            2,
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
