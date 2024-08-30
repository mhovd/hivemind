use solver::solver_server::{Solver, SolverServer};
use solver::{SolveReply, SolveRequest, Subject};
use tonic::{transport::Server, Request, Response, Status};

pub mod solver {
    tonic::include_proto!("solver"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MySolver {}

#[tonic::async_trait]
impl Solver for MySolver {
    async fn solve(&self, request: Request<SolveRequest>) -> Result<Response<SolveReply>, Status> {
        println!("Received request: {:?}", request);

        // Extract the subject from the request
        let subject = request.into_inner().subject.unwrap_or_default();

        // Perform some operations with the subject and support point
        // Here you can add your logic for processing
        println!("Processing subject with ID: {}", subject.id);

        // For demonstration purposes, we return the same subject back
        let reply = SolveReply {
            subject: Some(subject),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the address where the server will run
    let addr = "[::1]:50051".parse()?;
    let solver = MySolver::default();

    println!("Server listening on {}", addr);

    // Start the gRPC server
    Server::builder()
        .add_service(SolverServer::new(solver))
        .serve(addr)
        .await?;

    Ok(())
}
