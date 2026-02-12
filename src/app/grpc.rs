use crate::app::app::App;
use crate::server::server::rust_hexagonal_service_server;
use tonic::transport::Server;
use tonic_reflection::server::Builder;



const DESCRIPTOR: &[u8] =
    include_bytes!("../../target/descriptor.bin");



impl App {
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}",self.config.grpc.address ,self.config.grpc.port).parse()?;
        println!("Listening on {}", addr);

        let reflection_service = Builder::configure()
            .register_encoded_file_descriptor_set(DESCRIPTOR)
            .build()?;

        Server::builder()
        // .add_service(rust_hexagonal_service_server()).add_service(reflection_service)
        .add_service(rust_hexagonal_service_server()).add_service(reflection_service)
        .serve(addr).await?;
        Ok(())
    }

    pub fn stop(&self) {
        println!("Stopping {}", self.config.name);
    }
}