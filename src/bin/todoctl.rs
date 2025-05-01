use clap::{Parser, Subcommand};
use rusqlite::Connection;
use todoctl::{actions, db};

/// Simple CLI Todo app, backed by rusqlite + prettytable.
#[derive(Debug, Parser)]
#[command(name = "todoctl", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new todo (you can also omit `--title` to be prompted)
    Add {
        #[arg(short, long)]
        title: Option<String>,
    },
    /// Show all todos
    Show,
    /// Update a todo’s title and/or completed status
    Update {
        /// ID of the todo to update (omit to be prompted)
        #[arg(short, long)]
        id: Option<String>,

        /// New title (omit to be prompted)
        #[arg(short, long)]
        title: Option<String>,

        /// New completed status
        #[arg(long)]
        completed: Option<bool>,
    },
    /// Mark a todo complete (omit `--id` to be prompted)
    Complete {
        #[arg(short, long)]
        id: Option<String>,
    },
    /// Delete a todo (omit `--id` to be prompted)
    Delete {
        #[arg(short, long)]
        id: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("todoctl [Built w/ ❤️ by mt0.dev]");

    // initialize DB
    let conn: Connection = match db::init() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error initializing database: {}", e);
            std::process::exit(1);
        }
    };

    // dispatch
    match cli.command {
        Commands::Add { title } => actions::add_action(&conn, title).await,
        Commands::Show => actions::show_action(&conn).await,
        Commands::Update {
            id,
            title,
            completed,
        } => actions::update_action(&conn, id, title, completed).await,
        Commands::Complete { id } => actions::complete_action(&conn, id).await,
        Commands::Delete { id } => actions::delete_action(&conn, id).await,
    }
}
