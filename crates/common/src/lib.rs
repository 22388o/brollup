mod bitcoin_client;

use anyhow::Result;
use testcontainers::{clients::Cli, core::WaitFor, images::generic::GenericImage, Container, Image};


async fn start_bitcoind<'a>(docker: &'a Cli) -> Result<Container<'a, GenericImage>> {
    let image = GenericImage::new("lncm/bitcoind", "v26.1")
        .with_exposed_port(8332)
        .with_exposed_port(18444)
        .with_wait_for(WaitFor::message_on_stdout("Done loading"))
        .with_env_var("BITCOIN_NETWORK", "regtest")
        .with_env_var("BITCOIN_EXTRA_ARGS", "-regtest -rpcbind=0.0.0.0 -rpcallowip=0.0.0.0/0 -rpcuser=user -rpcpassword=pass -debug");

    let container = docker.run(image);
    Ok(container)
}


#[tokio::test]
async fn start_test_bitcoind() -> Result<()> {
    let docker = Cli::default();
    let bitcoind = start_bitcoind(&docker).await?;

    println!("Bitcoind containerID:{}", bitcoind.id());

    Ok(())
}
