use rust_hexagonal::app::app::App;

fn main() {
    let mut app = App::new("my_container".to_string(), "MyApp".to_string());
    app.config.initialize();
    println!("Timeout: {:?}", app.config.grpc.grace_fully_shutdown_timeout);
}
