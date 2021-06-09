pub mod utils;
use std::process::Command;
use utils::*;

fn main() -> Result<(), anyhow::Error> {
    // download_pi();
    let pipi = cmd_args();
    let matches = pipi.clone().get_matches();

    // Check for given operation variants from user input
    let op = if matches.is_present("add") {
        Operation::Add
    } else if matches.is_present("remove") {
        Operation::Remove
    } else {
        Operation::Help
    };

    // Bind each operation variants to operation functions
    match op {
        Operation::Add => {
            let args_list = matches.values_of("add").unwrap().collect::<Vec<_>>();
            let mut search_list: Vec<String> = Vec::new();
            args_list
                .iter()
                .for_each(|arg| search_list.push(arg.to_string()));
            println!("{:?}", search_list);
        }
        Operation::Remove => {
            let args_list = matches.values_of("remove").unwrap().collect::<Vec<_>>();
            let remove_list: Vec<String> = args_list
                .iter()
                .map(|arg| arg.clone().to_string())
                .collect();
        }
        _ => {
            let helper = pipi.clone().print_help();
            helper.unwrap();
            println!();
        }
    }

    Ok(())
}
