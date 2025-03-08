use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::{Duration, SystemTime};
use tokio::{task, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mqttoptions = MqttOptions::new("rumqtt-async", "rumqttd", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("hello/rumqtt", QoS::AtMostOnce)
        .await
        .unwrap();

    task::spawn(async move {
        while let Ok(notification) = eventloop.poll().await {
            println!("Received = {:?}", notification);
        }
    });

        loop {
            let now = SystemTime::now();
            let outgoing_message = format!("{}", now.elapsed().unwrap().as_secs());
            if let Err(e) = client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, outgoing_message.as_bytes().to_vec())
                .await {
                println!("Error publishing stats: {:?}", e);
            }
            time::sleep(Duration::from_secs(5)).await;
        }
}
