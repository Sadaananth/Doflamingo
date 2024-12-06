#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use duckdb::{params, Connection, Result};
use duckdb::arrow::record_batch::RecordBatch;
use duckdb::arrow::util::pretty::print_batches;
use std::error::Error;
use std::io::{self, Write};

slint::include_modules!();

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("starting");
    io::stdout().flush().unwrap();
    let conn = Connection::open_in_memory().expect("Failed to open connection");

    conn.execute_batch(
        r"CREATE SEQUENCE seq;
          CREATE TABLE person (
                  id              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  name            TEXT NOT NULL,
                  data            BLOB
                  );
         ").expect("Failed to create table");
    let me = Person {
        id: 0,
        name: "Test".to_string(),
        data: None,
    };
    println!("id {}", me.id);
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?, ?)",
        params![me.name, me.data],
    ).expect("Failed to execute");

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    }).expect("Failed to query_map");

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    // query table by arrow
    let rbs: Vec<RecordBatch> = stmt.query_arrow([])?.collect();
    let _ = print_batches(&rbs);
    println!("All well");

    let ui = AppWindow::new()?;
    ui.run()?;

    Ok(())
}
