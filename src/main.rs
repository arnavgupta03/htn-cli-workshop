use clap::{Parser, ValueEnum};
use core::panic;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::{distributions::Alphanumeric, Rng};
use ratatui::{
    prelude::CrosstermBackend,
    widgets::{Block, Paragraph},
    Terminal,
};
use serde_json::{Map, Value};
use std::path::Path;
use std::{
    collections::HashMap,
    io::{self, BufReader, Write},
};
use std::{fs::File, io::stdout};

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

fn main() -> io::Result<()> {
    let args = Args::parse();

    let path = Path::new("src/store.json");
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

    match args.interface {
        Some(TInterface::CLI) | None => {
            match args.operation {
                Some(Operation::Generate) => {
                    let to_insert = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(16)
                        .map(char::from)
                        .collect();

                    insert_password(to_insert);
                }
                Some(Operation::Insert) => {
                    let to_insert = match args.password_to_insert {
                        Some(p) => p,
                        None => {
                            println!("please pass a password to insert!");
                            return Ok(());
                        }
                    };

                    insert_password(to_insert);
                }
                Some(Operation::Get) => match store.contains_key(&args.name) {
                    false => {
                        println!("no password for {}", args.name);
                    }
                    true => match &store[&args.name] {
                        Value::Null => {
                            println!("no password for {}", args.name)
                        }
                        Value::String(s) => println!("password for {}", s),
                        _ => panic!("bad name passed!"),
                    },
                },
                None => panic!("no operation passed for CLI"),
            }
            Ok(())
        }
        Some(TInterface::TUI) => {
            enable_raw_mode()?;
            stdout().execute(EnterAlternateScreen)?;
            let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

            let mut time_to_quit = false;
            let passwords: String = gather_passwords();

            while !time_to_quit {
                terminal.draw(|frame| {
                    frame.render_widget(
                        Paragraph::new(passwords.clone())
                            .block(Block::bordered().title("Password Manager")),
                        frame.area(),
                    );
                })?;
                time_to_quit = handle_events()?;
            }

            disable_raw_mode()?;
            stdout().execute(LeaveAlternateScreen)?;

            Ok(())
        }
    }
}

fn gather_passwords() -> String {
    let path = Path::new("src/store.json");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open store file: {}", why),
    };
    let reader = BufReader::new(file);

    let store: HashMap<String, Value> = match serde_json::from_reader(reader) {
        Ok(json) => json,
        Err(why) => panic!("couldn't parse store file JSON: {}", why),
    };

    match store.len() {
        0 => "No passwords stored currently!".to_string(),
        _ => store
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("\n"),
    }
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
