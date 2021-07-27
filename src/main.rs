pub mod generate_iso;
pub mod utils;

use nix::unistd::Uid;
// use std::env;
// use std::ffi::OsStr;
use generate_iso::make_livecd::livecd;
use std::fs;
use std::path::Path;
use std::process::Command;
use utils::*;
use std::io::prelude::*;

fn main() {
    // download_pi();
    let pipi = cmd_args();
    let matches = pipi.clone().get_matches();

    // Check for given operation variants from user input
    let op = if matches.is_present("add") {
        Operation::Add
    } else if matches.is_present("remove") {
        Operation::Remove
    } else if matches.is_present("create") {
        Operation::Create
    } else if matches.is_present("build") {
        Operation::Build
    } else if matches.is_present("update") {
        Operation::Update
    } else if matches.is_present("new") {
        Operation::New
    } else if matches.is_present("iso") {
        Operation::Iso
    } else {
        Operation::Help
    };

    // Bind each operation variants to operation functions
    match op {
        Operation::Add => {
            if !Uid::effective().is_root() {
                panic!("You need sudo permission to run use this feature");
            }
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
            if !Uid::effective().is_root() {
                panic!("You need sudo permission to run use this feature");
            }
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

        Operation::Create => {
            if !Uid::effective().is_root() {
                panic!("You need sudo to run this feature");
            }

            let args_list = matches.values_of("create").unwrap().collect::<Vec<_>>();
            let mut db_path: Vec<String> = args_list
                .iter()
                .map(|arg| arg.clone().to_string())
                .collect();

            let _the_process = Command::new("bin-repo")
                .arg("create")
                .args(db_path)
                .spawn()
                .ok()
                .expect("Failed to execute.");
        }

        Operation::Build => {
            // let file = fs::read_to_string("packages").expect("Something wrong");
            // let mut packages: Vec<String> = file.lines().map(|p|p.to_string()).collect();
            // let mut build_list = Vec::new();

            // for fi in packages.iter_mut() {
            //     // *fi = fi.trim().replace("\n", " ");
            //     *fi = fi.replace("\n", " ");
            //     // build_list = vec![fi];
            //     println!("{:?}", build_list);
            // }

            // println!("{:?}", packages);

            // example: env rootfs=pi install (packages)
            let mut data = String::new();
            let mut file = fs::File::open("packages").unwrap();
            file.read_to_string(&mut data).unwrap();
            let packages: Vec<String> = data.lines().map(|l| l.to_string()).collect();

            let _the_process = Command::new("env")
                // .env("ROOT_DIR", "rootfs")
                .arg("ROOT=rootfs")
                .arg("pi")
                .arg("install")
                .args(&packages)
                .spawn()
                .ok()
                .expect("Failed to execute.");
        }

        Operation::Update => {
            let _the_process = Command::new("pi")
                .arg("-u")
                .spawn()
                .ok()
                .expect("Failed to execute.");
        }

        Operation::New => {
            let args_list = matches.value_of("new").unwrap();

            if Path::new(&args_list).exists() {
                println!("Failed to create new projects, project name already exists");
            } else {
                let _copy_files = Command::new("cp")
                    .arg("-r")
                    .arg("/opt/pipi-live")
                    .arg(&args_list)
                    .spawn()
                    .ok()
                    .expect("Failed to execute.");

                println!("Successfully create new project");
            }
        }

        Operation::Iso => {
            if !Uid::effective().is_root() {
                panic!("You need sudo permission to run use this feature");
            }
            livecd();
        }

        _ => {
            // if !Uid::effective().is_root() {
            //     panic!("You need sudo permission to run use this feature");
            // }

            let pi_exist = Path::new("/usr/bin/pi").exists();
            if pi_exist == true {
                let helper = pipi.clone().print_help();
                helper.unwrap();
                println!();
            } else {
                download_pi();
                let helper = pipi.clone().print_help();
                helper.unwrap();
                println!();
            }
        }
    }
}
