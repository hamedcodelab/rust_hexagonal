use crate::server::server::RustHexagonalGrpcServer;
use crate::server::server::rust_hexagonal_service;
use crate::server::server::rust_hexagonal_service::rust_hexagonal_service_server::{RustHexagonalService};
use crate::user::domain::user::User;
use crate::server::server::rust_hexagonal_service::{CreateUserRequest, CreateUserResponse, GetUserRequest, GetUserResponse, ListUserRequest, ListUserResponse, DeleteUserRequest, DeleteUserResponse};
use tonic::{ Request,Response, Status};


#[tonic::async_trait]
impl RustHexagonalService for RustHexagonalGrpcServer {
    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<CreateUserResponse>, Status> {
        let req: CreateUserRequest = request.into_inner();
        // handle use_case to create user
        let new_user =self.user_uc.create(&mut User::new(
            req.email,req.password
        )).await
            .map_err(|e| Status::internal(format!("Failed to create user: {}", e)))?;

        let res: CreateUserResponse = CreateUserResponse {
            user: Some(rust_hexagonal_service::User{
                id: new_user.base.id,
                created_at:new_user.base.created_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
                email:new_user.email,
                password:new_user.password,
                updated_at:new_user.base.updated_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
            })
        };
        Ok(Response::new(res))
    }

    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<GetUserResponse>, Status> {
        let req: GetUserRequest = request.into_inner();
        let fetch_user = self.user_uc.get_by_id(req.id)
            .await.map_err(|e| Status::internal(format!("Failed to get user: {}", e)))?
            .ok_or_else(|| Status::not_found("User not found"))?;

        let res: GetUserResponse = GetUserResponse {
            user: Some(rust_hexagonal_service::User{
                id: fetch_user.base.id,
                created_at: fetch_user.base.created_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
                email: fetch_user.email,
                password: fetch_user.password,
                updated_at: fetch_user.base.updated_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
            })
        };
        Ok(Response::new(res))
    }

    async fn list_user(&self, _request: Request<ListUserRequest>) -> Result<Response<ListUserResponse>, Status> {
        let users = self.user_uc
            .list()
            .await
            .map_err(|e| Status::internal(format!("Failed to list users: {}", e)))?;

        let res = ListUserResponse {
            user: users.into_iter().map(|u| {
                rust_hexagonal_service::User {
                    id: u.base.id,
                    created_at: u.base.created_at
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default(),
                    email: u.email,
                    password: String::new(), // 🔐 never expose password
                    updated_at: u.base.updated_at
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default(),
                }
            }).collect(),
        };

        Ok(Response::new(res))
    }

    async fn delete_user(&self, request: Request<DeleteUserRequest>) -> Result<Response<DeleteUserResponse>, Status> {
        let req: DeleteUserRequest = request.into_inner();
        self.user_uc.delete_by_id(req.id)
            .await.map_err(|e| Status::internal(format!("Failed to delete user: {}", e)))?;

        let res: DeleteUserResponse = DeleteUserResponse {
            result: true,
        };
        Ok(Response::new(res))
    }

}