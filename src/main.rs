use rdkafka::{
    ClientContext, 
    Message,
    config::ClientConfig,
    consumer::{
        BaseConsumer, 
        Consumer
    },
    producer::{
        BaseRecord, 
        ProducerContext, 
        ThreadedProducer,
    }
};

use std::thread;
use std::time::Duration;

const KAFKA_BROKER_PORT: &str = "127.0.0.1:9092";
const TOPIC_NAME: &str = "cute_pandas";
const GROUP_ID: &str = "robs_group";
const TIMEOUT_MS: i32 = 5000;

fn main() {

    println!("Creating Consumer");

    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", KAFKA_BROKER_PORT)
        .set("group.id", GROUP_ID)
        .create()
        .expect("invalid consumer config");

    consumer
        .subscribe(&[TOPIC_NAME])
        .expect("topic subscribe failed");

    // Rob - move with the closure
    thread::spawn(move || loop {
        for result in consumer.iter() {
            let evt = result.unwrap();
            let value = evt.payload().unwrap();
            println!(
                "received event {:?} {:?}",
                value,
                String::from_utf8(value.to_vec()).unwrap()
            )
        }
    });

    let custom_context = ProduceCallbackLogger;

    let producer: ThreadedProducer<ProduceCallbackLogger> = ClientConfig::new()
        .set("bootstrap.servers", KAFKA_BROKER_PORT)
        .set("message.timeout.ms",TIMEOUT_MS.to_string())
        .create_with_context(custom_context)
        .expect("Error creating producer");

    for i in 1..100 {

        let _delivery_status = producer
            .send(
                BaseRecord::to(TOPIC_NAME)
                    .payload("test")
                    .key(&i.to_string()),
            );

        thread::sleep(Duration::from_secs(3));
    }
}

struct ProduceCallbackLogger;

impl ClientContext for ProduceCallbackLogger {}

impl ProducerContext for ProduceCallbackLogger {

    type DeliveryOpaque = ();

    fn delivery(&self, 
                delivery_result: &rdkafka::producer::DeliveryResult<'_>, 
                _delivery_opaque: Self::DeliveryOpaque) {

        let result = delivery_result.as_ref();

        match result {
            Ok(_m) => {
                println!("delivered successfuly")
            }
            Err (err) => {
                println!("borked - error below"); 
                println!(
                    "failed to produce message with error - {}",
                    err.0,
                )
            }
        }
    }
}

