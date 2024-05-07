use sc_cli::{ChainSpec, SubstrateCli};
use crate::cli::{Cli, Subcommand};

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Qiankun Node".into()
    }

    fn impl_version() -> String {
        env!("QIANKUN_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "qiankun.network".into()
    }

    fn copyright_start_year() -> i32 {
        2024
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn ChainSpec>, String> {
        todo!()
    }
}

pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();
    
    match &cli.subcommand {
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {

            })
        }
    }
}