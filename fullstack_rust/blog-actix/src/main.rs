use dotenv::dotenv;
use env_logger::Env;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("actix_web=info")).init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app = blog_actix::app::Blog::new(8998);

    app.run(database_url).await
}
