/*
use clap::{ArgMatches, Command};

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

mod hello;
mod serve;

use clap::{ArgMatches, Command};
use crate::settings::Settings;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
}

/*
pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    hello::handle(matches)?;
    serve::handle(matches)?;

    Ok(())
}
*/

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, settings)?;
    serve::handle(matches, settings)?;

    Ok(())
}
