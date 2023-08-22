use crate::{repository, ui};

pub fn list_entries() -> std::io::Result<()> {
    let path = ui::select_file();
    let entries = repository::load_entry_list(&path).expect("Cannot load entries");

    print!("{}", entries);

    Ok(())
}
