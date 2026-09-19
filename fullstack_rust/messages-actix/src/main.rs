use env_logger::Env;
use messages_actix::app::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("actix_web=info")).init();

    run(8080).await
}
