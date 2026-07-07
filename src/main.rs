use crate::importer::load_all_items;

mod importer;


fn main() {
    // Run this once at the start of your program
    let db_items = load_all_items("poec_data.json").expect("Failed to load and parse database");

    println!("Successfully loaded {} items into memory.", db_items.len());

    // Optional: Just print the first one to verify it worked without blowing up your terminal
    if let Some(first) = db_items.first() {
        println!("{:#?}", first);
    }
}
