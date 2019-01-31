#![windows_subsystem = "windows"]

pub mod db;

fn main() {
    let database = db::SqlHandler::new();
}
