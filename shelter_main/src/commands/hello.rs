use crate::settings::Settings; 
use clap::{ArgMatches, Command};

pub const COMMAND_NAME: &str = "hello";

/*
pub fn configure(command: Command) -> Command {
    command.subcommand(Command::new("hello").about("Hello World!"))
}

// handle 方法接受返回的参数匹配clap 并检查我们的hello子命令是否被调用。
// 如果是这种情况，它会打印Hello World!到控制台。
pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("hello") {
        println!("Hello World!");
    }

    Ok(())
}
*/

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Hello World!") 
}

pub fn handle(_matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    println!("Hello World!");

    Ok(())
}
