use anyhow::{Context, Result};
use clap::arg_enum;
use env_logger::Env;
use lib::routes::build_warp_routes;
use lib::service::RequestHandler;
use lib::storage::MemoryBuddiesStore;
use log::info;
use std::env;
use structopt::StructOpt;

// This is for the psql impl that's not yet built
#[macro_use]
extern crate diesel;

mod lib;

arg_enum! {
    #[derive(StructOpt, PartialEq, Debug)]
    pub enum Storage {
    Psql,
        Memory,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "buddies", about = "A tool to help me be a better buddy")]
struct Args {
    /// Database URL to connect to
    #[structopt(long, env, default_value = "i-am-currently-unused")]
    database_url: String,
    /// Make the logging loud and annoying
    #[structopt(short, long)]
    debug: bool,
    /// Port to listen on
    #[structopt(short, long, default_value = "9001")]
    port: u16,
    #[structopt(long, possible_values = &Storage::variants(), case_insensitive = true, default_value="memory")]
    storage_type: Storage,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    // Setup Logging
    if args.debug {
        // If both RUST_LOG env variable and debug are given, choose env variable
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    }

    let port = match env::var("PORT") {
        Ok(port) => {
            info!(
                "Port set in environment variable {}. Overwriting {}",
                port, args.port
            );
            port.parse::<u16>().context("ENV Port is not a u16?!")?
        }
        Err(..) => args.port,
    };

    // Run the service. Because we can't return different types, and we can't make
    // things trait objects either, we run the code in a weird way.
    // TODO --> Can I make these not required to be clone?
    match args.storage_type {
        Storage::Psql => {
            todo!()
        }
        Storage::Memory => {
            info!("Using Memory Storage. Note, no information will be saved!");
            let handler = RequestHandler::new(MemoryBuddiesStore::new());
            let routes = build_warp_routes(handler);
            info!("Running server on port {}", port);
            warp::serve(routes).run(([0, 0, 0, 0], port)).await;
        }
    };
    Ok(())
}
