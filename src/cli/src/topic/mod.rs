mod create;
mod delete;
mod describe;
mod list;

pub use cli::*;

mod cli {

    use structopt::StructOpt;
    use super::*;

    use create::CreateTopicOpt;
    use delete::DeleteTopicOpt;
    use describe::DescribeTopicsOpt;
    use list::ListTopicsOpt;

    use create::process_create_topic;
    use delete::process_delete_topic;
    use describe::process_describe_topics;
    use list::process_list_topics;

    use crate::COMMAND_TEMPLATE;
    use crate::Terminal;
    use crate::CliError;

    #[derive(Debug, StructOpt)]
    #[structopt(name = "topic", about = "Topic operations")]
    pub enum TopicOpt {
        /// Creates a Topic with the given name
        #[structopt(
            name = "create",
            template = COMMAND_TEMPLATE,
        )]
        Create(CreateTopicOpt),

        /// Deletes a Topic with the given name
        #[structopt(
            name = "delete",
            template = COMMAND_TEMPLATE,
        )]
        Delete(DeleteTopicOpt),

        /// Prints detailed information about a Topic
        #[structopt(
            name = "describe",
            template = COMMAND_TEMPLATE,
        )]
        Describe(DescribeTopicsOpt),

        /// Lists all of the Topics in the cluster
        #[structopt(
            name = "list",
            template = COMMAND_TEMPLATE,
        )]
        List(ListTopicsOpt),
    }

    pub(crate) async fn process_topic<O>(
        out: std::sync::Arc<O>,
        topic_opt: TopicOpt,
    ) -> Result<String, CliError>
    where
        O: Terminal,
    {
        let output = match topic_opt {
            TopicOpt::Create(create_topic_opt) => process_create_topic(create_topic_opt).await?,
            TopicOpt::Delete(delete_topic_opt) => process_delete_topic(delete_topic_opt).await?,
            TopicOpt::Describe(describe_topics_opt) => {
                process_describe_topics(out, describe_topics_opt).await?
            }
            TopicOpt::List(list_topics_opt) => process_list_topics(out, list_topics_opt).await?,
        };
        Ok(output)
    }
}
