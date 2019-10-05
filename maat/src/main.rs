extern crate kafka;
extern crate env_logger;

use std::time;
use std::env;
use std::thread;

use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::Error as KafkaError;

fn main() {
    env_logger::init();

    let broker = env::var("BROKER").unwrap();
    let topic = "Events";

    println!("Connecting to {}, sending events to topic {}", broker, topic);

    let one_second = time::Duration::new(1, 0);

    let mut tick = 0;
    let mut next = time::Instant::now() + one_second;

    loop {
        let data = format!("{{ event_type=\"CombatTick\", \"id\":\"{}\" }}", tick).to_owned();
        if let Err(e) = produce_message(topic, vec![broker.to_owned()], data.as_bytes()) {
            println!("Failed consuming messages: {}", e);
        }
        println!("Sent Event: {}", data);

        if tick % 6 == 0 {
            let data = format!("{{ event_type=\"WorldTick\", \"id\":\"{}\" }}", tick / 6).to_owned();
            if let Err(e) = produce_message(topic, vec![broker.to_owned()], data.as_bytes()) {
                println!("Failed consuming messages: {}", e);
            }
            println!("Sent Event: {}", data);
        }

        let sleep_time = next - time::Instant::now();
        if sleep_time.as_millis() > 0 {
            println!("Sleeping {} millis", sleep_time.as_millis());
            thread::sleep(sleep_time);
        }

        tick = tick + 1;
        next = next + one_second;
    }
}

fn produce_message(
    topic: &str,
    brokers: Vec<String>,
    data: &[u8],
) -> Result<(), KafkaError> {
    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(time::Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    producer.send(&Record::from_value(topic, data))?;

    Ok(())
}