// Headifier 0.1.0
// David Serrano, October 21st, 2023
// MIT License
// Made With Love ❤️
use ratui::init_ratui;
pub mod core;
pub mod ratui;
pub mod ui;
use anyhow::Result;

fn main() -> Result<()> {
    init_ratui()
}
