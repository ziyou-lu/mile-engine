
use tonic::{transport::Server, Request, Response, Status};

use login::greeter_server::{Greeter, GreeterServer};
use login::{LoginRequest, LoginResponse};

pub mod login {
    tonic::include_proto!("login"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct LoginGeeter {

}

#[tonic::async_trait]
impl Greeter for LoginGeeter {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = LoginResponse {
            success: true,
            message: format!("Hello {}!", request.into_inner().user_name).into(), 
            token: String::from("11111111111111111111111"),
        };

        Ok(Response::new(reply))
    }
}