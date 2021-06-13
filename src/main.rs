pub mod utils;
use std::process::Command;
use utils::*;
use std::ffi::OsStr;
use std::env;
use std::fs;

// fn main() -> Result<(), anyhow::Error> {
fn main () {
    // download_pi();
    let pipi = cmd_args();
    let matches = pipi.clone().get_matches();

    // Check for given operation variants from user input
    let op = if matches.is_present("add") {
        Operation::Add
    } else if matches.is_present("remove") {
        Operation::Remove
    } else if matches.is_present("build") {
        Operation::Build
    } else {
        Operation::Help
    };

    // Bind each operation variants to operation functions
    match op {
        Operation::Add => {
            let args_list = matches.values_of("add").unwrap().collect::<Vec<_>>();
            let mut add_list: Vec<String> = Vec::new();
            args_list
                .iter()
                .for_each(|arg| add_list.push(arg.to_string()));

            let _the_process = Command::new("bin-repo")
                .arg("add")
                .arg("/var/www/core/core.db")
                .args(add_list)
                .spawn()
                .ok()
                .expect("Failed to execute.");

        }

        Operation::Remove => {
            let args_list = matches.values_of("remove").unwrap().collect::<Vec<_>>();
            let mut remove_list: Vec<String> = args_list
                .iter()
                .map(|arg| arg.clone().to_string())
                .collect();
            let _the_process = Command::new("bin-repo")
                .arg("remove")
                .arg("/var/www/core/core.db")
                .args(remove_list)
                .spawn()
                .ok()
                .expect("Failed to execute.");
        }

        Operation::Build => {
            let file = fs::read_to_string("packages").expect("Something wrong");
            let mut package = vec![file];
            let mut build_list = String::new();

            for fi in package.iter_mut() {
                *fi = fi.trim().replace("\n", " ").to_string();
                build_list = fi.to_string();
            }

            // example: env rootfs=pi install (packages)
            let _the_process = Command::new("env")
                .arg("rootfs=pi")
                .arg("install")
                .arg(build_list)
                .spawn()
                .ok()
                .expect("Failed to execute.");
            }

        _ => {
            let helper = pipi.clone().print_help();
            helper.unwrap();
            println!();
        }
    }
}



        // Operation::Remove => {
        //     let args_list = matches.values_of("remove").unwrap().collect::<Vec<_>>();
        //     let remove_list: Vec<String> = args_list
        //         .iter()
        //         .map(|arg| arg.clone().to_string())
        //         .collect();
        //     let the_process = Command::new("bin-repo")
        //         .arg("remove")
        //         .arg("/var/www/core/core.db")
        //         .args(remove_list)
        //         .spawn()
        //         .ok()
        //         .expect("Failed to execute.");

        // _ => {
        //     let helper = pipi.clone().print_help();
        //     helper.unwrap();
        //     println!();
        // }

    // Ok(())
