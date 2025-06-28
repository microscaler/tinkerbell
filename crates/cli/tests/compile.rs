use clap::CommandFactory;
use cli::Cli;

#[test]
fn compile() {
    let _cmd = Cli::command();
}
