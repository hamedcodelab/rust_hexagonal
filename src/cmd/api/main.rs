use rust_hexagonal::pkg::config::loader;
use rust_hexagonal::app::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = loader::load()?;
    let app = App::new(config);
    app.run();
    Ok(())
}
