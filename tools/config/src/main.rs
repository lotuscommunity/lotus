use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    lotus_config::config_cli::ConfigCli::parse().run().await
}
