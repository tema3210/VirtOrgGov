use clap::Parser;
use std::{net::SocketAddr,io};
use actix_web::{HttpServer,App,web};
use deadpool_diesel::postgres::{Manager, Runtime, Pool};
use std::num::NonZeroUsize;
use actix_files::Files;

#[macro_use] extern crate diesel;

type DbPool = Pool;

pub(crate) mod schema;
pub(crate) mod orm;
pub(crate) mod pages;
pub(crate) mod api;
pub(crate) mod templates;

#[derive(Parser)]
#[clap(about,long_about = None,version)]
struct CmdArgs {
    /// Address of a database we use
    #[clap(short,long)]
    database: SocketAddr,
    /// A number of connections to DB
    #[clap(short,long)]
    connection_number: NonZeroUsize,
    /// Login for database
    #[clap(long)]
    login: String,
    /// Password for database
    #[clap(long)]
    password: String,
    /// Socket to which we bind
    #[clap(short,long)]
    socket: SocketAddr,
    /// A path to static files dir. Can be both relative and absolute
    #[clap(short,long)]
    files: std::path::PathBuf,
}


const DB_NAME: &str = "virt_org_gov";

fn build_pool(url: &str,conn_count: usize) -> Pool {
    let manager = Manager::new(url,Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .max_size(conn_count)
        .build()
        .unwrap();
    pool
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    let args = CmdArgs::parse();

    /// Init logging
    tracing_subscriber::fmt::init();

    let sock = args.socket;
    let db_url = format!("postgresql://{}:{}@{}/{}",&args.login,&args.password,&args.database,DB_NAME);
    let conn_count = args.connection_number.get();
    let files = args.files.clone();

    tracing::info!("Initializing server");

    let app = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(build_pool(&db_url,conn_count)))
            .service(Files::new("/static",&files))
            //register pages
            .service(pages::index)
            .service(pages::codex_page)
            .service(pages::laws_page)
            .service(pages::law_page)
            .service(pages::party_page)
            .service(pages::politic_page)
            .service(pages::change_law_page)
            //register endpoints
            .service(api::law_change_endpoint)

    })
    .bind(sock)?;

    tracing::info!("Server init done");
    app.run().await
}