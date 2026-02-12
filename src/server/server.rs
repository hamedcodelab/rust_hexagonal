use rust_hexagonal_service ::rust_hexagonal_service_server::{RustHexagonalService,RustHexagonalServiceServer};
use rust_hexagonal_service::{CreateUserRequest, CreateUserResponse,GetUserRequest,GetUserResponse};
use tonic::{ Request,Response, Status};



mod rust_hexagonal_service {
    tonic::include_proto!("proto.rust_hexagonal.v1");
}


#[derive(Debug,Default)]
pub struct RustHexagonalV1Service {}

#[tonic::async_trait]
impl RustHexagonalService for RustHexagonalV1Service {
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


pub fn rust_hexagonal_service_server() -> rust_hexagonal_service::rust_hexagonal_service_server::RustHexagonalServiceServer<RustHexagonalV1Service> {
    let user: RustHexagonalV1Service = RustHexagonalV1Service::default();
    rust_hexagonal_service::rust_hexagonal_service_server::RustHexagonalServiceServer::new(user)
}