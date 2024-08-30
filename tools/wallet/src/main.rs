use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    lotus_wallet::wallet_cli::WalletCli::parse().run().await
}
