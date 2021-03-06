mod register;
mod list;
mod unregister;

pub use cli::*;

mod cli {

    use structopt::StructOpt;

    use register::RegisterCustomSpuOpt;
    use register::process_register_custom_spu;

    use unregister::UnregisterCustomSpuOpt;
    use unregister::process_unregister_custom_spu;

    use list::ListCustomSpusOpt;
    use list::process_list_custom_spus;

    use crate::COMMAND_TEMPLATE;
    use crate::error::CliError;
    use crate::Terminal;

    use super::*;

    #[derive(Debug, StructOpt)]
    pub enum CustomSpuOpt {
        /// Registers a new custom SPU with the cluster
        #[structopt(
            name = "register",
            template = COMMAND_TEMPLATE,
        )]
        Create(RegisterCustomSpuOpt),

        /// Unregisters a custom SPU from the cluster
        #[structopt(
            name = "unregister",
            template = COMMAND_TEMPLATE,
        )]
        Delete(UnregisterCustomSpuOpt),

        /// List all custom SPUs known by this cluster
        #[structopt(
            name = "list",
            template = COMMAND_TEMPLATE,
        )]
        List(ListCustomSpusOpt),
    }

    pub(crate) async fn process_custom_spu<O: Terminal>(
        out: std::sync::Arc<O>,
        custom_spu_opt: CustomSpuOpt,
    ) -> Result<String, CliError> {
        match custom_spu_opt {
            CustomSpuOpt::Create(custom_spu_opt) => {
                process_register_custom_spu(custom_spu_opt).await?;
            }
            CustomSpuOpt::Delete(custom_spu_opt) => {
                process_unregister_custom_spu(custom_spu_opt).await?;
            }
            CustomSpuOpt::List(custom_spu_opt) => {
                process_list_custom_spus(out, custom_spu_opt).await?;
            }
        }
        Ok("".to_string())
    }
}
