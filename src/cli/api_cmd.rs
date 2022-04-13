// S3OpsCommands is generated into build_gen and contain a subcommand per operation in the model.
use crate::codegen_include::S3OpsCommands;
use crate::utils::{new_s3_client, staticify};

/// Call S3 API operations
#[derive(clap::Parser, Debug, Clone)]
pub struct ApiCmd {
    #[clap(subcommand)]
    op: S3OpsCommands,
}

impl ApiCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let s3 = staticify(new_s3_client().await);
        self.op.run(s3).await;
        Ok(())
    }
}
