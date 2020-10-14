use fluvio::{Fluvio, FluvioError};
use fluvio::metadata::topic::{TopicSpec, TopicReplicaParam};

pub async fn fresh_topic(fluvio: &Fluvio) -> Result<String, FluvioError> {
    let topic = format!("test-{}", uuid::Uuid::new_v4());
    let mut admin = fluvio.admin().await;
    let topic_spec = TopicSpec::Computed(TopicReplicaParam::new(1, 1, false));
    admin.create(topic.clone(), false, topic_spec).await?;
    Ok(topic)
}
