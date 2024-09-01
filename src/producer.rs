use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use serde::{Deserialize, Serialize};
use std::error::Error;

use pharmsol::{Subject, SubjectBuilderExt};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    subject: Subject,
    support_point: Vec<f64>,
}

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
    for i in 1..10 {
        // Subject
        let subject = Subject::builder(i.to_string())
            .bolus(0.0, i as f64 * 100.0, 0)
            .observation(12.0, 20.0, 0)
            .build();

        // Support point
        let support_point = vec![1.0, 2.0, 0.0, 10.0];

        // Create a task with the given data
        let task = Task {
            subject: subject,
            support_point: support_point,
        };

        // Serialize the task to a JSON string
        let subject = serde_json::to_string(&task)?;

        // Publish the task to the queue
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
