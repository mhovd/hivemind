use futures::stream::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use pharmsol::{Subject, SubjectBuilderExt};
use serde::{Deserialize, Serialize};
use std::{error::Error, thread::sleep};
use tokio_amqp::*;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    data: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Establish a connection to RabbitMQ
    let conn = Connection::connect(
        "amqp://192.168.1.119:5672/%2f",
        ConnectionProperties::default().with_tokio(),
    )
    .await?;
    let channel = conn.create_channel().await?;

    // Declare the same queue to ensure it exists
    let queue = channel
        .queue_declare(
            "task_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Declared queue: {:?}", queue);

    // Start consuming messages from the queue
    let mut consumer = channel
        .basic_consume(
            "task_queue",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Waiting for messages...");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("Error in consumer stream");

        // Deserialize the task from the message
        let task: Task = serde_json::from_slice(&delivery.data)?;
        println!("Received task: {:?}", task);

        // Perform the computation (example: multiply by 2)
        //sleep(std::time::Duration::from_secs(5));
        let result = task.data * 2;
        println!("Computed result: {}", result);

        // Acknowledge the message
        delivery.ack(BasicAckOptions::default()).await?;
    }

    Ok(())
}
