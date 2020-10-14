use std::time::Duration;
use fluvio::{FluvioError, Offset, Fluvio};
use fluvio_future::timer::sleep;

mod common;

async fn produce(topic: String, messages: Vec<String>) -> Result<(), FluvioError> {
    println!("Running producer");
    let producer = fluvio::producer(topic).await?;
    for message in messages {
        println!("Producing message: {}", &message);
        producer.send_record(message, 0).await?;
        sleep(Duration::from_millis(10)).await;
    }
    Ok(())
}

async fn consume(topic: String, expected_messages: &[String]) -> Result<(), FluvioError> {
    println!("Running consumer");
    use futures_util::StreamExt;
    let consumer = fluvio::consumer(topic, 0).await?;
    let mut stream = consumer.stream(Offset::beginning()).await?;

    let mut index: usize = 0;
    while let Some(Ok(record)) = stream.next().await {
        println!("Received record: index={}", index);
        assert_eq!(record.offset(), index as i64, "Record offset={}, index={}", record.offset(), index);
        match record.try_into_bytes() {
            None => panic!("All records should have bytes"),
            Some(bytes) => {
                let expected_bytes = expected_messages.get(index).unwrap();
                let bytes_string = String::from_utf8(bytes).expect("messages should be UTF8");
                println!("Record {} has data: {}", index, bytes_string);
                assert_eq!(expected_bytes, &bytes_string);
                index += 1;
                if index >= expected_messages.len() { break; }
            }
        }
    }
    assert_eq!(index, expected_messages.len());
    Ok(())
}

#[test]
fn test_echo_fast_consumer() {

    let result: Result<(), FluvioError> = async_std::task::block_on(async {
        let fluvio = Fluvio::connect().await?;
        let topic = common::fresh_topic(&fluvio).await?;

        let mut payload = vec![];
        for i in 0..1000 {
            let message = format!("test-{}-{}", uuid::Uuid::new_v4(), i);
            payload.push(message);
        }
        println!("Using payload: {:?}", &payload);
        sleep(Duration::from_millis(1000)).await;

        produce(topic.clone(), payload.clone()).await?;
        consume(topic, &payload).await?;
        Ok(())
    });

    if let Err(e) = result {
        panic!("Failed test: {}", e);
    }
}
