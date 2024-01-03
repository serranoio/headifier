// Headifier 0.4.0
// David Serrano
// January 3rd, 2023


use ratui::init_ratui;
pub mod core;
pub mod ratui;
pub mod ui;
use anyhow::Result;

fn main() -> Result<()> {
    init_ratui()
}





