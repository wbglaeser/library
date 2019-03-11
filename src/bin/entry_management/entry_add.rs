use std::io;
use postgres::{Connection};

pub struct Publication {
    id: i32,
    title: String,
    author: String,
    genre: String,
}

pub fn read_input(info: String) -> String {
    println!("Type in the {} of the work!", info);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => {
            println!("error: {}", error);
            read_input(info);
        },
    }
    input
}

pub fn insert_entry(conn: &Connection, entry: Publication) {
    conn.execute("INSERT INTO publication (title, author, genre) VALUES ($1, $2, $3)",
                 &[&entry.title, &entry.author, &entry.genre]).unwrap();
}

pub fn add_entry(conn: &Connection) {

    let title = read_input(String::from("Title"));
    let author = read_input(String::from("Author"));
    let genre = read_input(String::from("Genre"));

    let new_pub = Publication {
        id: 0,
        title: title,
        author: author,
        genre: genre,
    };
    insert_entry(conn, new_pub);
}




