use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Node { 
        #[command(subcommand)]
        command: NodeCommands 
    },
    #[command(flatten)]
    Legacy(LegacyCommands),
}

#[derive(Subcommand, Debug)]
enum NodeCommands {
    Health { path: String },
}

#[derive(Subcommand, Debug)]
enum LegacyCommands {
    Health { path: String },
}

fn main() {
    let cli = Cli::parse_from(vec!["test", "health", "."]);
    println!("{:?}", cli);
}
