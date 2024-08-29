use clap::{Parser, ValueEnum};
use core::panic;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::{Map, Value};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

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

    let path = Path::new("store.json");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open store file: {}", why),
    };
    let reader = BufReader::new(file);

    let mut store: Map<String, Value> = match serde_json::from_reader(reader) {
        Ok(json) => json,
        Err(why) => panic!("couldn't parse store file JSON: {}", why),
    };

    let mut insert_password = |to_insert: String| {
        store.insert(args.name.clone(), serde_json::Value::String(to_insert));

        let mut new_file = match File::create(&path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't update store file: {}", why),
        };

        match new_file.write_all(Value::Object(store.clone()).to_string().as_bytes()) {
            Ok(file) => file,
            Err(why) => panic!("couldn't write to store file: {}", why),
        }
    };

    match args.operation {
        Operation::Generate => {
            let to_insert = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();

            insert_password(to_insert)
        }
        Operation::Insert => {
            let to_insert = match args.password_to_insert {
                Some(p) => p,
                None => {
                    println!("please pass a password to insert!");
                    return;
                }
            };

            insert_password(to_insert)
        }
        Operation::Get => match &store[&args.name] {
            Value::Null => println!("no password for {}", args.name),
            Value::String(s) => println!("password for {}", s),
            _ => panic!("bad name passed!"),
        },
    }
}
