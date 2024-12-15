#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::prelude::*;
use duckdb::Result;
use std::error::Error;

slint::include_modules!();

use Doflamingo::database::database::DatabaseHandler;

fn main() -> Result<(), Box<dyn Error>> {
    let database_handler = DatabaseHandler::new();
    database_handler.init_handler();

    let today = Local::now();
    let ui = AppWindow::new()?;
    ui.global::<Data>().set_this_year(today.year());
    ui.global::<Data>()
        .set_this_month(today.month().try_into().unwrap());
    ui.global::<Data>()
        .set_today(today.day().try_into().unwrap());
    ui.on_add_salary(move |date, salary| {
        database_handler.add_salary(
            "Test".to_string(),
            date.to_string(),
            salary.try_into().unwrap(),
            0u32,
        );
    });
    ui.run()?;

    Ok(())
}
