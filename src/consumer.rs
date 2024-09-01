use futures::stream::StreamExt;
use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use serde::{Deserialize, Serialize};
use std::error::Error;

use pharmsol::{fa, fetch_cov, fetch_params, lag};
use pharmsol::{simulator, Subject};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    subject: Subject,
    support_point: Vec<f64>,
}

const SERVER_ADDR: &str = "amqp://192.168.1.119:5672/%2f";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Establish a connection to RabbitMQ
    let conn = Connection::connect(SERVER_ADDR, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    // Declare the same queue to ensure it exists
    let queue = channel
        .queue_declare(
            "subjects",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Declared queue: {:?}", queue);

    // Start consuming messages from the queue
    let mut consumer = channel
        .basic_consume(
            "subjects",
            "worker1",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Waiting for messages...");

    // Define model
    let ode = simulator::Equation::new_ode(
        //Difussion Equations
        |x, p, _t, dx, _rateiv, _cov| {
            fetch_cov!(cov, t,);
            fetch_params!(p, ka, ke, _tlag, _v);
            dx[0] = -ka * x[0];
            dx[1] = ka * x[0] - ke * x[1];
        },
        // Lag definition (In this case boluses on dx[0] will be delayed by `tlag`)
        |p| {
            fetch_params!(p, _ka, _ke, tlag, _v);
            lag! {0=>tlag}
        },
        // No bio-availability
        |_p| fa! {},
        // Default initial conditions (0.0,0.0)
        |_p, _t, _cov, _x| {},
        // Output Equations
        |x, p, _t, _cov, y| {
            fetch_params!(p, _ka, _ke, _tlag, v);
            y[0] = x[1] / v;
        },
        (2, 1),
    );

    // Listen for tasks
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("Error in consumer stream");

        // Deserialize the task from the message
        let task: Task = serde_json::from_slice(&delivery.data)?;
        println!("Received task: {:?}", task);

        // Perform the computation (example: multiply by 2)
        //sleep(std::time::Duration::from_secs(5));
        //let result = task.data * 2;
        //println!("Computed result: {}", result);

        // Acknowledge the message
        delivery.ack(BasicAckOptions::default()).await?;

        // Simulate the subject
        let pred = ode.simulate_subject(&task.subject, &task.support_point);

        dbg!(pred);

        // Send the result to the results queue
        //let result = Task { data: result };
        //let result = serde_json::to_string(&result)?;
        channel
            .basic_publish(
                "",
                "results_queue",
                BasicPublishOptions::default(),
                "s".as_bytes(),
                BasicProperties::default(),
            )
            .await?;
    }

    Ok(())
}
