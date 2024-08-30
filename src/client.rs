use pharmsol::{Subject, SubjectBuilderExt};
use solver::solver_client::SolverClient;
use solver::{SolveRequest, Subject as ProtoSubject, SupportPoint};
pub mod parser;
pub mod solver {
    tonic::include_proto!("solver"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the gRPC server
    let mut client = SolverClient::connect("http://[::1]:50051").await?;

    // Create a sample Subject and SupportPoint
    let subject = Subject::builder("id1")
        .bolus(0.0, 100.0, 0)
        .repeat(2, 0.5)
        .observation(0.5, 0.1, 0)
        .observation(1.0, 0.4, 0)
        .observation(2.0, 1.0, 0)
        .observation(2.5, 1.1, 0)
        .covariate("wt", 0.0, 80.0)
        .covariate("wt", 1.0, 83.0)
        .covariate("age", 0.0, 25.0)
        .build();

    let support = SupportPoint {
        values: vec![1.0, 2.0, 3.0], // Example support point values
    };

    // Create the request
    let request = tonic::Request::new(SolveRequest {
        subject: Some(ProtoSubject::from(&subject)),
        support: Some(support),
    });

    // Send the request
    let response = client.solve(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}
