use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = lotus_genesis_tools::cli::GenesisCli::parse();
    cli.execute().await
}
