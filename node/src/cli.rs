use sc_cli::RunCmd;

#[derive(Debug, Clone)]
pub enum Consensus {
    ManualSeal(u64),
    InstantSeal,
}

impl std::str::FromStr for Consensus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "instant-seal" {
            Consensus::InstantSeal
        } else if let Some(block_time) = s.strip_prefix("manual-seal-") {
            Consensus::ManualSeal(block_time.parse().map_err(|_| "invalid block time")?)
        } else {
            return Err("incorrect consensus identifier".into());
        })
    }
}

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[clap(long, default_value = "manual-seal-3000")]
    pub consensus: Consensus,

    #[clap(flatten)]
    pub run: RunCmd,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {}
