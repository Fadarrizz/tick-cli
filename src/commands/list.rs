use crate::{repository, ui::components};

pub fn list_entries() -> std::io::Result<()> {
    let filename = components::select_file().unwrap();
    let entries = repository::load_entry_list(&filename).expect("Cannot load entries");

    print!("{}", entries);

    Ok(())
}
