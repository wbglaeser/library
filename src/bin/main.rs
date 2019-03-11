mod db_setup;
mod entry_management;
use std::io;


extern crate postgres;
use crate::db_setup::dbpsql;
use crate::entry_management::entry_add;

fn main() {

    // check whether databse exists
    let conn = dbpsql::connect_to_db();
    println!("database is connected");
    dbpsql::build_publication_table(&conn);

    // add new entry
    entry_add::add_entry(&conn);
}
