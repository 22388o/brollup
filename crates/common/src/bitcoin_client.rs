use std::error::Error;

use anyhow::{Context, Result};
use bdk::Balance;
use bdk::bitcoin::bip32::ExtendedPubKey;
use bdk::bitcoin::Network;
use bdk::blockchain::{ConfigurableBlockchain, RpcBlockchain, RpcConfig};
use bdk::blockchain::rpc::Auth;
use bdk::database::MemoryDatabase;
use bdk::keys::{DerivableKey, ExtendedKey, GeneratableKey};
use bdk::keys::bip39::{Language, Mnemonic, WordCount};
use bdk::wallet::{AddressIndex, SyncOptions, Wallet};

pub struct BitcoinClient {
    wallet: Wallet<MemoryDatabase>,
    blockchain: RpcBlockchain,
}

pub fn generate_p2tr_descriptor(network: Network) -> Result<(String, String)> {
    // Generate a mnemonic
    let m = Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    // Convert the mnemonic to an extended key
    let xkey: ExtendedKey = m
        .clone()
        .into_extended_key()
        .context("Failed to convert mnemonic into extended key")?;
    let xprv = xkey
        .into_xprv(network)
        .context("Failed to convert extended key into xprv")?;
    let xpub = ExtendedPubKey::from_priv(&bdk::bitcoin::secp256k1::Secp256k1::new(), &xprv);

    // Get the fingerprint
    let fingerprint = xprv.fingerprint(&bdk::bitcoin::secp256k1::Secp256k1::new());

    // Create the descriptor
    let descriptor = format!("tr([{}]{}/*)", fingerprint, xpub);
    let change_descriptor = format!("tr([{}]{}/*)", fingerprint, xpub);

    Ok((descriptor, change_descriptor))
}

impl BitcoinClient {
    pub fn new(rpc_url: &str, user: &str, pass: &str, descriptor: &str, change_descriptor: Option<&str>) -> Result<Self> {
        let rpc_auth = Auth::UserPass {
            username : user.to_string(),
            password : pass.to_string()
        };

        println!("url: {}", rpc_url);

        let rpc_config = RpcConfig {
            url: rpc_url.to_string(),
            auth: rpc_auth,
            network: Network::Regtest,
            wallet_name: "bdk_wallet".to_string(),
            sync_params: None,
        };
        let blockchain = RpcBlockchain::from_config(&rpc_config)?;

        let wallet = Wallet::new(
            descriptor,
            change_descriptor,
            Network::Regtest,
            MemoryDatabase::new(),
        ).context("Failed to create wallet")?;

        Ok(BitcoinClient { wallet, blockchain })
    }

        pub fn sync(&self) -> Result<()> {
            self.wallet.sync(&self.blockchain, SyncOptions::default()).context("Failed to sync wallet")?;
            Ok(())
        }

        pub fn get_new_address(&self) -> Result<String> {
            let address = self.wallet.get_address(AddressIndex::New)
                .context("Failed to get new address")?
                .address
                .to_string();
            Ok(address)
        }

        pub fn get_balance(&self) -> Result<Balance> {
            let balance = self.wallet.get_balance().context("Failed to get wallet balance")?;
            Ok(balance)
        }

}




#[cfg(test)]
mod tests {
    use anyhow::Result;
    use bdk::bitcoin::Network;
    use testcontainers::clients::Cli;

    use crate::bitcoin_client::{BitcoinClient, generate_p2tr_descriptor};
    use crate::start_bitcoind;

    #[tokio::test]
    async fn test_bdk_client_connect() -> Result<()> {
        let docker = Cli::default();
        let bitcoind = start_bitcoind(&docker).await?;

        println!("Bitcoind container is running with ID: {}", bitcoind.id());

        let rpc_port = bitcoind.get_host_port_ipv4(8332);

        let url = format!("0.0.0.0:{}", rpc_port);

        let (descriptor, change_descriptor)= generate_p2tr_descriptor(Network::Regtest)?;

        let client = BitcoinClient::new(&url, "user","pass",descriptor.as_str(), Option::from(change_descriptor.as_str()))?;

        client.sync()?;

        let address = client.get_new_address()?;

        println!("Generated Address: {}", address);

        let balance = client.get_balance()?;

        println!("Initial Balance: {:?}", balance.get_total());

        Ok(())
    }
}
