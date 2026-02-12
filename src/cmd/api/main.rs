use rust_hexagonal::pkg::config::loader;
use rust_hexagonal::app::app::App;

#[cfg(debug_assertions)]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await.expect("Error running app");
    Ok(())
}

#[cfg(not(debug_assertions))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await.expect("Error running app");
    Ok(())
}


async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = loader::load()?;
    let mut app = App::new(config);
    app.init_dep().await;
    app.init_domain().await;
    app.start().await?;
    Ok(())
}