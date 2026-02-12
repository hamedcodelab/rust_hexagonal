use std::sync::Arc;
use rust_hexagonal_service ::rust_hexagonal_service_server::{RustHexagonalService, RustHexagonalServiceServer};
use rust_hexagonal_service::{CreateUserRequest, CreateUserResponse,GetUserRequest,GetUserResponse};
use tonic::{ Request,Response, Status};
use crate::user::port::UserUsecase;



mod rust_hexagonal_service {
    tonic::include_proto!("proto.rust_hexagonal.v1");
}

pub struct RustHexagonalGrpcServer {
    user_uc: Arc<dyn UserUsecase>,
}

impl RustHexagonalGrpcServer {
    pub fn new(user_uc: Arc<dyn UserUsecase>) -> Self {
        Self { user_uc }
    }
}

#[tonic::async_trait]
impl RustHexagonalService for RustHexagonalGrpcServer {
    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<CreateUserResponse>, Status> {
        let req:CreateUserRequest= request.into_inner();
        println!("Received CreateUserRequest:{:?}", req);
        let res:CreateUserResponse = CreateUserResponse{
            user: Some(rust_hexagonal_service::User{
                id:11,
                created_at:"created_at".to_string(),
                email:"email".to_string(),
                password:"password".to_string(),
                updated_at:"updated_at".to_string(),
            })
        };
        Ok(Response::new(res))
    }

    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<GetUserResponse>, Status> {
        self.user_uc.get_by_id(11).await;
        let req:GetUserRequest = request.into_inner();
        println!("Received GetUserRequest:{:?}", req);
        let res:GetUserResponse = GetUserResponse{
            user: Some(rust_hexagonal_service::User{
                id:11,
                created_at:"created_at".to_string(),
                email:"email".to_string(),
                password:"password".to_string(),
                updated_at:"updated_at".to_string(),
            })
        };
        Ok(Response::new(res))
    }
}

pub fn rust_hexagonal_service_server(user_uc: Arc<dyn UserUsecase>) -> RustHexagonalServiceServer<RustHexagonalGrpcServer> {
    RustHexagonalServiceServer::new(
        RustHexagonalGrpcServer::new(user_uc)
    )
}