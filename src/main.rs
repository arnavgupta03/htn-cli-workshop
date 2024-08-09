use clap::{Parser,ValueEnum};

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

    match args.operation {
        Operation::Generate => {
            // TODO: generate and store password
        },
        Operation::Insert => {
            // TODO: insert given password
        },
        Operation::Get => {
            // TODO: get password
        },
    }
}
