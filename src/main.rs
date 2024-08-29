use clap::{Parser,ValueEnum};
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

#[derive(ValueEnum,Clone,Debug)]
enum Operation {
    Generate,
    Insert,
    Get,
}

#[derive(ValueEnum,Clone,Debug,Default)]
enum TInterface {
    #[default]
    CLI,
    TUI,
}

#[derive(Parser,Debug)]
#[command(about="A Simple Password Manager", long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    operation: Operation,

    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    password_to_insert: Option<String>,

    #[arg(short, long)]
    interface: Option<TInterface>,
}

fn main() {
    let args = Args::parse();

    let path = Path::new("store.json")
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open store file: {}", why);
    };

    match args.operation {
        Operation::Generate => {
            // TODO: generate and store password
        },
        Operation::Insert => {
            // TODO: insert given password
        },
        Operation::Get => {
            let reader = BufReader::new(file);

            let store = match serde_json::from_reader(reader) {
                Ok(json) => json,
                Err(why) => panic!("couldn't parse store file JSON: {}", why);
            };

            match store[args.name] {
                serde_json::Null => println!("no password for {}", args.name),
                serde_json::String(s) => println!("password for {}", s),
            }
        },
    }
}
