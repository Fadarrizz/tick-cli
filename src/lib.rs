use core::fmt;

use chrono::NaiveTime;
use itertools::Itertools;
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

    pub fn get_task_id(&self) -> &u32 {
        &self.task_id
    }

    pub fn get_task_name(&self) -> &String {
        &self.task_name
    }

    pub fn get_start_time(&self) -> &NaiveTime {
        &self.start_time
    }

    pub fn is_missing_end_time(&self) -> bool {
        self.end_time.is_none()
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

    pub fn calculate_hours(&self) -> f64 {
        if self.end_time.is_none() {
            return 0.0
        }

        let diff = self.end_time.unwrap() - self.start_time;

        (diff.num_minutes() as f64) / 60.0
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "  {}{}: {} | {}",
            self.start_time.format("%H:%M"),
            if self.end_time.is_some() { 
                format!(" - {}", self.end_time.unwrap().format("%H:%M")) 
            } else { 
                String::new() 
            },
            self.project_name,
            self.notes,
        )
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

    pub fn get(&self, index: usize) -> &Entry {
        &self.entries[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }

    pub fn get_all(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn get_last(&self) -> Option<&Entry> {
        self.entries.last()
    }

    pub fn get_last_mut(&mut self) -> Option<&mut Entry> {
        self.entries.last_mut()
    }

    pub fn add(&mut self, entry: Entry) {
        self.entries.push(entry);

        self.sort();
    }

    pub fn sort(&mut self) {
        self.entries.sort_by(|a, b| a.start_time.cmp(&b.start_time));
    }

    pub fn set_end_times(&mut self) {
        let mut iter = self.entries.iter_mut();

        if let Some(mut current) = iter.next() {
            for next in iter {
                if current.end_time.is_none() {
                    let end_time = next.start_time.clone();
                    current.set_end_time(end_time);
                }
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

#[derive(Serialize, Debug, Clone)]
pub struct TickEntry {
    date: String,
    task_id: u32,
    hours: f64,
    notes: String,
}

impl TickEntry {
    pub fn from_entry(date: String, entry: &Entry) -> Self {
        Self {
            date,
            task_id: *entry.get_task_id(),
            hours: entry.calculate_hours(),
            notes: entry.get_notes().clone(),
        }
    }

    pub fn get_date(&self) -> &String {
        &self.date
    }

    pub fn get_task_id(&self) -> &u32 {
        &self.task_id
    }

    pub fn get_hours(&self) -> &f64 {
        &self.hours
    }

    pub fn get_notes(&self) -> &String {
        &self.notes
    }
}

#[derive(Serialize, Debug)]
pub struct TickEntryList {
    tick_entries: Vec<TickEntry>,
}

impl TickEntryList {
    pub fn from_entry_list(filename: &String, entry_list: &EntryList) -> Self {
        let tick_entries = entry_list
        .get_all()
        .iter()
        .map(|entry| TickEntry::from_entry(filename.clone(), entry))
        .collect::<Vec<TickEntry>>();

        Self { tick_entries }
    }

    pub fn get_all(&self) -> &Vec<TickEntry>
    {
        &self.tick_entries
    }

    pub fn get(&self, index: usize) -> &TickEntry {
        &self.tick_entries[index]
    }

    pub fn len(&self) -> usize {
        self.tick_entries.len()
    }

    pub fn merge(&mut self) {
        self.tick_entries = self.tick_entries
            .iter()
            .sorted_by_key(|e| e.task_id)
            .group_by(|e| (e.date.clone(), e.task_id))
            .into_iter()
            .map(|((date, task_id), group)| {
                let (mut hours, mut notes) = (0.0, String::new());
                for entry in group {
                    hours += entry.hours;

                    if !notes.contains(&entry.notes) {
                        if !notes.is_empty() {
                            notes.push_str(", ");
                        }
                        notes.push_str(&entry.notes);
                    }
                }

                TickEntry { date, task_id, hours, notes }
            })
            .collect::<Vec<TickEntry>>()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_calculate_entry_hours() {
        let entry = Entry::create(
            "project A".to_string(),
            1,
            "task 1".to_string(),
            NaiveTime::from_str("09:00:00").unwrap(),
            Some(NaiveTime::from_str("10:00:00").unwrap()),
            "notes".to_string(),
        );

        assert_eq!(1.0, entry.calculate_hours());

        let entry = Entry::create(
            "project A".to_string(),
            1,
            "task 1".to_string(),
            NaiveTime::from_str("09:00:00").unwrap(),
            Some(NaiveTime::from_str("10:15:00").unwrap()),
            "notes".to_string(),
        );

        assert_eq!(1.25, entry.calculate_hours());
    }

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

    #[test]
    fn test_set_entry_end_times_only_when_not_set() {
        let mut entries = EntryList::empty();

        entries.add(Entry::create(
            "project A".to_string(),
            1,
            "task 1".to_string(),
            NaiveTime::from_str("09:00:00").unwrap(),
            Some(NaiveTime::from_str("09:30:00").unwrap()),
            "notes".to_string(),
        ));

        entries.add(Entry::create(
            "project B".to_string(),
            2,
            "task 2".to_string(),
            NaiveTime::from_str("10:00:00").unwrap(),
            None,
            "notes".to_string(),
        ));

        entries.add(Entry::create(
            "project C".to_string(),
            3,
            "task 3".to_string(),
            NaiveTime::from_str("10:30:00").unwrap(),
            None,
            "notes".to_string(),
        ));

        entries.set_end_times();

        assert_eq!(
            Some(NaiveTime::from_str("09:30:00").unwrap()),
            entries.get(0).end_time,
        );
        assert_eq!(
            Some(NaiveTime::from_str("10:30:00").unwrap()),
            entries.get(1).end_time,
        );
        assert_eq!(None, entries.get(2).end_time);
    }

    #[test]
    fn test_merge_tick_entries() {
        let tick_entry1 = TickEntry {
            date: "2023-07-01".to_string(),
            task_id: 0,
            hours: 1.5,
            notes: "first entry".to_string(),
        };

        let tick_entry2 = TickEntry {
            date: "2023-07-01".to_string(),
            task_id: 0,
            hours: 1.25,
            notes: "second entry".to_string(),
        };

        let tick_entry3 = TickEntry {
            date: "2023-07-01".to_string(),
            task_id: 99,
            hours: 2.0,
            notes: "third entry".to_string(),
        };
        
        let mut tick_entry_list = TickEntryList {
            tick_entries: vec![tick_entry1, tick_entry2, tick_entry3],
        };

        assert_eq!(3, tick_entry_list.len());

        tick_entry_list.merge();

        assert_eq!(2, tick_entry_list.len());

        let merged_entry = tick_entry_list.get(0);
        assert_eq!(&"2023-07-01".to_string(), merged_entry.get_date());
        assert_eq!(&0, merged_entry.get_task_id());
        assert_eq!(&2.75, merged_entry.get_hours());
        assert_eq!(&"first entry, second entry".to_string(), merged_entry.get_notes());

        let not_merged_entry = tick_entry_list.get(1);
        assert_eq!(&"2023-07-01".to_string(), not_merged_entry.get_date());
        assert_eq!(&99, not_merged_entry.get_task_id());
        assert_eq!(&2.0, not_merged_entry.get_hours());
        assert_eq!(&"third entry".to_string(), not_merged_entry.get_notes());
    }

    #[test]
    fn test_merge_tick_entries_spread_out() {
        let tick_entry1 = TickEntry {
            date: "2023-07-01".to_string(),
            task_id: 0,
            hours: 1.5,
            notes: "first entry".to_string(),
        };

        let tick_entry2 = TickEntry {
            date: "2023-07-01".to_string(),
            task_id: 1,
            hours: 2.0,
            notes: "second entry".to_string(),
        };

        let tick_entry3 = TickEntry {
            date: "2023-07-01".to_string(),
            task_id: 0,
            hours: 1.25,
            notes: "third entry".to_string(),
        };

        let tick_entry4 = TickEntry {
            date: "2023-07-01".to_string(),
            task_id: 1,
            hours: 2.75,
            notes: "fourth entry".to_string(),
        };
        
        let mut tick_entry_list = TickEntryList {
            tick_entries: vec![tick_entry1, tick_entry2, tick_entry3, tick_entry4],
        };

        assert_eq!(4, tick_entry_list.len());

        tick_entry_list.merge();

        assert_eq!(2, tick_entry_list.len());

        let merged_entry1 = tick_entry_list.get(0);
        assert_eq!(&"2023-07-01".to_string(), merged_entry1.get_date());
        assert_eq!(&0, merged_entry1.get_task_id());
        assert_eq!(&2.75, merged_entry1.get_hours());
        assert_eq!(&"first entry, third entry".to_string(), merged_entry1.get_notes());

        let merged_entry2 = tick_entry_list.get(1);
        assert_eq!(&"2023-07-01".to_string(), merged_entry2.get_date());
        assert_eq!(&1, merged_entry2.get_task_id());
        assert_eq!(&4.75, merged_entry2.get_hours());
        assert_eq!(&"second entry, fourth entry".to_string(), merged_entry2.get_notes());
    }

    #[test]
    fn test_merge_tick_entries_equal_notes() {
        let tick_entry1 = TickEntry {
            date: "2023-07-01".to_string(),
            task_id: 0,
            hours: 1.5,
            notes: "same notes".to_string(),
        };

        let tick_entry2 = TickEntry {
            date: "2023-07-01".to_string(),
            task_id: 0,
            hours: 1.25,
            notes: "same notes".to_string(),
        };
        
        let mut tick_entry_list = TickEntryList {
            tick_entries: vec![tick_entry1, tick_entry2],
        };

        tick_entry_list.merge();

        let merged_entry = tick_entry_list.get(0);
        assert_eq!(&"same notes".to_string(), merged_entry.get_notes());
    }
}
