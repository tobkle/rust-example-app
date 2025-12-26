use clorinde::deadpool_postgres::Pool;
use grpc_api::api::{self, GetUsersRequest, GetUsersResponse, User};
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct UsersService {
    pub pool: Pool,
}

#[tonic::async_trait]
impl api::users_server::Users for UsersService {
    async fn get_users(
        &self,
        _request: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let db_users = clorinde::queries::users::get_users()
            .bind(&client)
            .all()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let users = db_users
            .into_iter()
            .map(|u| User {
                id: u.id as u32,
                email: u.email,
            })
            .collect();

        Ok(Response::new(GetUsersResponse { users }))
    }
}
