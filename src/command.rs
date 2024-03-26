use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "hypertext-stitcher")]
#[command(about = "A simple static site generator", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Build your website
    Build,
    /// Creates a local dev server
    Serve
}