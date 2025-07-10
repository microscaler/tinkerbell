use clap::Parser;
use cli::{Cli, Commands};

#[test]
fn parse_status() {
    let cli = Cli::parse_from(["tctl", "status"]);
    assert!(matches!(cli.command, Commands::Status));
}

#[test]
fn parse_json_flag() {
    let cli = Cli::parse_from(["tctl", "--json", "status"]);
    assert!(cli.json);
}
