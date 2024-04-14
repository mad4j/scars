use tonic::{transport::Server, Request, Response, Status};

use file::file_server::{File, FileServer};
use file::{SizeOfRequest, SizeOfReply};

pub mod file {
    tonic::include_proto!("file");
}

#[derive(Debug, Default)]
pub struct MyFileServer {}

#[tonic::async_trait]
impl File for MyFileServer {


    async fn size_of(
        &self,
        request: Request<SizeOfRequest>
    ) -> Result<Response<SizeOfReply>, Status> {
        let reply = file::SizeOfReply {
            size: 1234u64,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyFileServer::default();

    Server::builder()
        .add_service(FileServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}