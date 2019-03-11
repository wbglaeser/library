extern crate postgres;
use postgres::{Connection, TlsMode};

struct Publication {
    id: i32,
    title: String,
    author: String,
}

pub fn connect_to_db () -> Connection {

    let conn = Connection::connect("postgres://ben.glaeser@localhost:5432/library", TlsMode::None);
    let conn = match conn {
        Ok(file) => file,
        Err(error) => {
            build_db()
        },
    };
    conn
}

fn build_db () -> Connection {
    let conn = Connection::connect("postgres://ben.glaeser@localhost:5432/postgres", TlsMode::None);
    let conn = match conn {
        Ok(file) => file,
        Err(error) => {
            panic!("Could not connect to postgres database");
        }
    };
    conn.execute("CREATE DATABASE library", &[]).unwrap();
    connect_to_db()
}

pub fn build_publication_table(conn: &Connection) {
    match conn.execute("CREATE TABLE publication (
                    id              SERIAL PRIMARY KEY,
                    title           VARCHAR NOT NULL,
                    author          VARCHAR,
                    genre           VARCHAR
                  )", &[]) {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
        }
    }
}
