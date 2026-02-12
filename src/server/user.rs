use crate::server::server::RustHexagonalGrpcServer;
use crate::server::server::rust_hexagonal_service;
use crate::server::server::rust_hexagonal_service::rust_hexagonal_service_server::{RustHexagonalService};
use crate::user::domain::user::User;
use crate::server::server::rust_hexagonal_service::{CreateUserRequest, CreateUserResponse, GetUserRequest, GetUserResponse};
use tonic::{ Request,Response, Status};


#[tonic::async_trait]
impl RustHexagonalService for RustHexagonalGrpcServer {
    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<CreateUserResponse>, Status> {
        let req: CreateUserRequest = request.into_inner();
        // handle usecase to create user
        let new_u_result =self.user_uc.create(&User::new(
            req.email
        )).await;

        let new_u = match new_u_result {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(Status::internal("User was not created"));
            }
            Err(e) => {
                return Err(Status::internal(format!("Failed to create user: {:?}", e)));
            }
        };


        let res: CreateUserResponse = CreateUserResponse {
            user: Some(rust_hexagonal_service::User{
                id:new_u.base.id,
                created_at:new_u.base.created_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
                email:new_u.email,
                password:new_u.password,
                updated_at:new_u.base.updated_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
            })
        };
        Ok(Response::new(res))
    }

    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<GetUserResponse>, Status> {
        let req: GetUserRequest = request.into_inner();
        let new_u_result = self.user_uc.get_by_id(req.id).await;

        let new_u = match new_u_result {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(Status::internal("User was not found"));
            }
            Err(e) => {
                return Err(Status::internal(format!("Failed to find user: {:?}", e)));
            }
        };

        let res: GetUserResponse = GetUserResponse {
            user: Some(rust_hexagonal_service::User{
                id:new_u.base.id,
                created_at:new_u.base.created_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
                email:new_u.email,
                password:new_u.password,
                updated_at:new_u.base.updated_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
            })
        };
        Ok(Response::new(res))
    }
}