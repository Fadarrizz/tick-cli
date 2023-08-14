use crate::{repository, ui};

pub fn list_entries() -> std::io::Result<()> {
    let filename = ui::select_file();
    let entries = repository::load_entry_list(&filename).expect("Cannot load entries");

    print!("{}", entries);

    Ok(())
}
