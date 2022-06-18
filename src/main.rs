mod api ; // Declare module api existence

use api::task::{
    get_task
}

use actix_web::{web, App,HttpServer ,HttpResponse, web::Data, middleware::Logger};
#[actix_web::main]//So cargo know the apps starts running Here
async fn main() -> std::io::Result<()> {
    std::env::set_var("Rust_LOG", "debug"); // When running runs with these env variables enabled
    std::env::set_var("Rust_BACKTRACE", "1");
    env_logger::init();
    H
    Ok(())
}

