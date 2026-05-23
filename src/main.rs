mod store;
mod cli;

use clap::Parser;
use store::DirList;
use cli::{Commands, Cli};

fn main() {
    let args = Cli::parse();

    let mut directories = DirList::load();

    match args.command {
        Commands::Add { path, name } => {
            match directories.add(&path, &name) {
                Ok(_) => println!("\nAdded directory {name} as {path}"),
                Err(e) => println!("\n{e}")
            };
        },
        Commands::Delete { name } => {
            match directories.delete(&name) {
                Ok(_) => println!("\nDeleted directory {name}"),
                Err(e) => println!("\n{e}")
            };
        },
        Commands::List => {
            println!("Your directories:\n");
            directories.list();
        },
        Commands::Return { name } => {
            match directories.get_path(&name) {
                Ok(pth) => println!("{pth}"),
                Err(e) => println!("{e}")
            }
        }
    }
}

