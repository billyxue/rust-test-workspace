use crate::settings::Settings;
use crate::state::ApplicationState;

use clap::{value_parser, Arg, ArgMatches, Command};
use axum::Router;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tower_http::trace::TraceLayer;
use std::sync::Arc;


pub const COMMAND_NAME: &str = "serve";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Start HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("serve") {
        let port: u16 = *matches.get_one("port").unwrap_or(&8080);

        println!("TBD: start the webserver on port {} {:?}", port, settings);
        start_tokio(port, settings)?;
    }

    Ok(())
}

fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let state = Arc::new(ApplicationState::new(settings)?);
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
            // let routes = Router::new();
            // 要在 axum 中启用日志记录，
            // 在函数中的配置中添加一个新层
            let routes = crate::api::configure(state)
                .layer(TraceLayer::new_for_http());

            tracing::info!("starting ... on {}", port);

            axum::Server::bind(&addr)
                .serve(routes.into_make_service())
                .await?;

            Ok::<(), anyhow::Error>(())
        })?;

    std::process::exit(0);
}
