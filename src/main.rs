use anyhow::{anyhow, Context, Result};
use clap::arg_enum;
use env_logger::Env;
use lib::routes::build_warp_routes;
use lib::service::{AuthHandler, RequestHandler};
use lib::storage::{MemoryBuddiesStore, PsqlBuddiesStore};
use log::info;
use std::env;
use std::fs;
use std::path::PathBuf;
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
    #[structopt(
        long,
        env = "DATABASE_URL",
        default_value = "postgresql://localhost:5432/buddies"
    )]
    database_url: String,
    /// Make the logging loud and annoying
    #[structopt(short, long)]
    debug: bool,
    /// Port to listen on
    #[structopt(short, long, default_value = "9001")]
    port: u16,
    #[structopt(long, possible_values = &Storage::variants(), case_insensitive = true, default_value="psql")]
    storage_type: Storage,
    #[structopt(long, env = "PRIVATE_KEY_LOCATION", hidden = true)]
    private_key_location: Option<PathBuf>,
    #[structopt(long, env = "PUBLIC_KEY_LOCATION", hidden = true)]
    public_key_location: Option<PathBuf>,
    #[structopt(long, env = "PRIVATE_KEY", hidden = true)]
    private_key: Option<String>,
    #[structopt(long, env = "PUBLIC_KEY", hidden = true)]
    public_key: Option<String>,
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

    let public: Vec<u8> = match (args.public_key, args.public_key_location) {
        (Some(key), _) => key.as_bytes().to_vec(),
        (None, Some(path)) => fs::read(path).expect("Reading Public Key"),
        (None, None) => {
            // TODO -> Logic in structopt
            return Err(anyhow!("One of public key or path must be provided"));
        }
    };

    let secret: Vec<u8> = match (args.private_key, args.private_key_location) {
        (Some(key), _) => key.as_bytes().to_vec(),
        (None, Some(path)) => fs::read(path).expect("Reading Private Key"),
        (None, None) => {
            // TODO -> Logic in structopt
            return Err(anyhow!("One of private key or path must be provided"));
        }
    };

    // Run the service. Because we can't return different types, and we can't make
    // things trait objects either, we run the code in a weird way.
    // TODO --> Can I make these not required to be clone?
    match args.storage_type {
        Storage::Psql => {
            info!("Connecting to database at url: {}", args.database_url);
            let buddies_store = PsqlBuddiesStore::new(&args.database_url);
            let auth_handler = AuthHandler::new(buddies_store.clone(), secret, public)
                .context("creating auth handler")?;
            let handler = RequestHandler::new(buddies_store);
            let routes = build_warp_routes(auth_handler, handler);
            info!("Running server on port {}", port);
            warp::serve(routes).run(([0, 0, 0, 0], port)).await;
        }
        Storage::Memory => {
            info!("Using Memory Storage. Note, no information will be saved!");
            let buddies_store = MemoryBuddiesStore::new();
            let auth_handler = AuthHandler::new(buddies_store.clone(), secret, public)
                .context("creating auth handler")?;
            let handler = RequestHandler::new(buddies_store);
            let routes = build_warp_routes(auth_handler, handler);
            info!("Running server on port {}", port);
            warp::serve(routes).run(([0, 0, 0, 0], port)).await;
        }
    };
    Ok(())
}
