#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use duckdb::{Connection, Result};
use std::error::Error;

slint::include_modules!();

mod salary;
use crate::salary::salary::add_salary;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open_in_memory().expect("Failed to open connection");

    conn.execute_batch(
        r"CREATE SEQUENCE seq;
          CREATE TABLE salary (
                  id              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  name            TEXT NOT NULL,
                  date            TEXT NOT NULL,
                  salary          INTEGER,
                  bonus           INTEGER
                  );
         ").expect("Failed to create table");
    let ui = AppWindow::new()?;
    ui.on_add_salary(move |date, salary| {
        add_salary(date.to_string(), salary.try_into().unwrap());
    });
    ui.run()?;

    Ok(())
}
