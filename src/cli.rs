use clap::{Parser, Subcommand};

/*
- start
- docs
- docs --users
- version
- generate --users --len=30
 */

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Start the server
    Start {
        /// Port to listen on
        #[clap(short, long, default_value = "8080")]
        port: u16,
    },
    /// Generate random users
    Generate {
        /// Number of data to generate
        #[clap(short, long, default_value = "10")]
        len: u32,

        /// Generate users
        #[clap(short, long)]
        users: bool,

        /// Generate posts
        #[clap(short, long)]
        posts: bool,

        /// Generate todos
        #[clap(short, long)]
        todos: bool,
    },
    /// Show docs
    Docs {
        /// Show docs for users
        #[clap(short, long)]
        users: bool,

        /// Show docs for posts
        #[clap(short, long)]
        posts: bool,

        /// Show docs for todos
        #[clap(short, long)]
        todos: bool,
    },
}
