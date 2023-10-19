// David Serrano
// Made with love
// David Serrano
// Made with Love, 2023// David Serrano, 2023
// Accential
// Made with Love
// Enjoy.

use std::env;

use ratui::initRatui;



pub mod core;
pub mod ui;

pub mod ratui;

fn main() {


    initRatui();


    // let path = env::current_dir();

    // let path_buf = match path {
    //     Ok(p) => {
    //         println!("{}", p.display());

    //         p
    //     }
    //     Err(e) => {
    //         panic!("No current directory{e}");
    //     }
    // };

    // let ignore_list = core::list_git_ignore(&path_buf);

    // core::visit_drs(&path_buf, &ignore_list);


}
