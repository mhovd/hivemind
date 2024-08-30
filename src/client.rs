use solver::solver_client::SolverClient;
use solver::{SolveRequest, Subject, SupportPoint};

pub mod solver {
    tonic::include_proto!("solver"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the gRPC server
    let mut client = SolverClient::connect("http://[::1]:50051").await?;

    // Create a sample Subject and SupportPoint
    let subject = Subject {
        id: "subject1".to_string(),
        occasions: vec![], // Add any required occasions here
    };

    let support = SupportPoint {
        values: vec![1.0, 2.0, 3.0], // Example support point values
    };

    // Create the request
    let request = tonic::Request::new(SolveRequest {
        subject: Some(subject),
        support: Some(support),
    });

    // Send the request
    let response = client.solve(request).await?;

    println!("Response: {:?}", response.into_inner());

    Ok(())
}
