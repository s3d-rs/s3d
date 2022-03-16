use crate::build_gen::S3OpsCommands;
use crate::utils::{new_s3d_client, staticify};

/// Call S3 API operation
/// S3OpsCommands is generated into build_gen and contain a command per operation in the model.
#[derive(clap::Parser, Debug, Clone)]
pub struct ApiCmd {
    #[clap(subcommand)]
    op: S3OpsCommands,
}

impl ApiCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let s3 = staticify(new_s3d_client());
        self.op.run(s3).await;
        Ok(())
    }
}
