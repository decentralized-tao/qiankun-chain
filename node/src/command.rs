use crate::{chain_spec, cli::Cli, service};
use sc_cli::{ChainSpec, SubstrateCli};

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Qiankun Network".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
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
        Ok(match id {
            "dev" => Box::new(chain_spec::developement_config()?),
            path => Box::new(chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }
}

pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                match config.network.network_backend {
                    sc_network::config::NetworkBackendType::Libp2p => {
                        service::new_full::<sc_network::NetworkWorker<_, _>>(config, cli.consensus)
                            .map_err(sc_cli::Error::Service)
                    }
                    sc_network::config::NetworkBackendType::Litep2p => service::new_full::<
                        sc_network::Litep2pNetworkBackend,
                    >(
                        config, cli.consensus
                    )
                    .map_err(sc_cli::Error::Service),
                }
            })
        }
        _ => Ok(()),
    }
}
