use std::sync::Arc;
use rust_hexagonal_service::rust_hexagonal_service_server::{RustHexagonalServiceServer};
use crate::user::port::UserUsecase;



pub mod rust_hexagonal_service {
    tonic::include_proto!("proto.rust_hexagonal.v1");
}

pub struct RustHexagonalGrpcServer {
    pub user_uc: Arc<dyn UserUsecase>,
}

impl RustHexagonalGrpcServer {
    pub fn new(user_uc: Arc<dyn UserUsecase>) -> Self {
        Self { user_uc }
    }
}



pub fn rust_hexagonal_service_server(user_uc: Arc<dyn UserUsecase>) -> RustHexagonalServiceServer<RustHexagonalGrpcServer> {
    RustHexagonalServiceServer::new(
        RustHexagonalGrpcServer::new(user_uc)
    )
}