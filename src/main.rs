use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum Operation {
    Generate,
    Insert,
    Get,
}

#[derive(ValueEnum, Clone, Debug, Default)]
enum TInterface {
    #[default]
    CLI,
    TUI,
}

#[derive(Parser, Debug)]
#[command(about="A Simple Password Manager", long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    operation: Option<Operation>,

    #[arg(short, long, default_value = "default")]
    name: String,

    #[arg(short, long)]
    password_to_insert: Option<String>,

    #[arg(short, long)]
    interface: Option<TInterface>,
}

fn main() {
    let args = Args::parse();

    let path = Path::new("src/store.json");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open store file: {}", why),
    };
    let reader = BufReader::new(file);

    match args.operation {
        Some(Operation::Generate) => {
            todo!("generate and store password")
        }
        Some(Operation::Insert) => {
            todo!("insert given password")
        }
        Some(Operation::Get) => {
            todo!("get password")
        }
        None => {
            todo!("error case")
        }
    }
}

fn gather_passwords() -> String {
    todo!("return all passwords")
}

fn handle_events() -> io::Result<bool> {
    todo!("handle input for tui")
}
