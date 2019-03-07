extern crate postgres;

use postgres::{Connection, TlsMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {

    // check whether databse exists
    let conn = Connection::connect("postgres://ben.glaeser@localhost:5432/newdatabase", TlsMode::None);

    let conn = match conn {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    println!("Found person");

}