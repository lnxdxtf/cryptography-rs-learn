extern crate dotenv;
pub mod zkp_course {
    include!("../proto_gen/zkp_auth.rs");
}
use std::{collections::HashMap, sync::Mutex};

use num_bigint::BigUint;
use tonic::{transport::Server, Code, Request, Response, Status};
use zkp_course::{
    auth_server::{self, Auth, AuthServer},
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};
#[macro_use]
extern crate dotenv_codegen;

#[derive(Debug, Default)]
pub struct AuthService {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
}
#[tonic::async_trait]
impl Auth for AuthService {
    async fn register(
        &self,
        _request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = _request.into_inner();
        let user_info = UserInfo {
            user_name: req.user_name.clone(),
            y1: BigUint::from_bytes_be(&req.y1),
            y2: BigUint::from_bytes_be(&req.y2),
            ..Default::default()
        };
        match &mut self.user_info.lock() {
            Ok(user_info_hashmap_store) => {
                if user_info_hashmap_store.contains_key(&req.user_name) {
                    return Err(Status::new(
                        Code::AlreadyExists,
                        "User already exists".to_string(),
                    ));
                }
                user_info_hashmap_store.insert(req.user_name, user_info);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(Status::new(
                    Code::Internal,
                    "Internal Server Error".to_string(),
                ));
            }
        }

        Ok(Response::new(RegisterResponse {}))
    }

    async fn authentication_challenge(
        &self,
        _request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        todo!()
    }

    async fn verify_authentication(
        &self,
        _request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct UserInfo {
    pub user_name: String,
    // Registration
    pub y1: BigUint,
    pub y2: BigUint,
    // Authorization
    pub r1: BigUint,
    pub r2: BigUint,
    // Verification
    pub c: BigUint,
    pub s: BigUint,
}

#[tokio::main]
async fn main() {
    let addr = format!(
        "{}:{}",
        dotenv!("GRPC_SERVER_ADDRS"),
        dotenv!("GRPC_SERVER_PORT")
    );
    println!("Listening on {}", addr);
    let auth_service = AuthService::default();
    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(addr.parse().expect("could not parse addr"))
        .await
        .unwrap();
}
