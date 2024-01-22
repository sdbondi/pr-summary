use clap::Parser;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[clap(long, short = 'o')]
    pub owner: String,
    #[clap(long, short = 'r')]
    pub repo: String,
    #[clap(
        long,
        short = 't',
        alias = "token",
        env = "GITHUB_TOKEN",
        hide_env_values = true,
        hide_default_value = true
    )]
    pub personal_token: String,

    /// Load a specific PR. Mostly used for development purposes
    #[clap(long)]
    pub pr: Option<u64>,
}

impl Cli {
    pub fn init() -> anyhow::Result<Self> {
        let cli = Self::try_parse()?;
        Ok(cli)
    }
}
