use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use std::error::Error;

use pharmsol::{Subject, SubjectBuilderExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Establish a connection to RabbitMQ

    let conn = Connection::connect(
        "amqp://192.168.1.119:5672/%2f",
        ConnectionProperties::default(),
    )
    .await?;
    let channel = conn.create_channel().await?;

    // Declare a queue
    let queue = channel
        .queue_declare(
            "subjects",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Declared queue: {:?}", queue);

    println!("Sending tasks at time {}", chrono::Utc::now());
    // Send 10 tasks
    for i in 0..10 {
        let subject = Subject::builder(i.to_string())
            .bolus(0.0, 100.0, 1)
            .observation(12.0, 20.0, 1)
            .covariate("weight", 0.0, 75.0)
            .build();

        // Serialize the task to a JSON string
        let subject = serde_json::to_string(&subject)?;

        channel
            .basic_publish(
                "",
                "subjects",
                BasicPublishOptions::default(),
                subject.as_bytes(),
                BasicProperties::default(),
            )
            .await?;
    }

    println!("All tasks sent at time {}", chrono::Utc::now());

    Ok(())
}
