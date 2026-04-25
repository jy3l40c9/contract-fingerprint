mod detect_compiler_new_with_proxies;

use crate::detect_compiler_new_with_proxies::detect_compiler_new;
use alloy_primitives::{hex, Address};
use alloy_provider::{Network, Provider, ProviderBuilder};
use alloy_rpc_client::ClientBuilder;
use clap::Parser;
use eyre::Result;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // Contract address
    address: Address,

    // Enable evmole output
    #[clap(long)]
    evmole: bool,
}

// Trigger comment
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Contract: {:?}", cli.address);

    let node_url = std::env::var("MAINNET_HTTP").unwrap_or("https://eth.llamarpc.com".to_string()).to_string();

    let node_url = url::Url::parse(node_url.as_str())?;
    let client = ClientBuilder::default().http(node_url);
    let provider = ProviderBuilder::new().on_client(client);

    contract_details(provider, cli.address, cli.evmole).await?;

    Ok(())
}

async fn contract_details<N, P>(provider: P, address: Address, evmole: bool) -> Result<()>
where
    N: Network,
    P: Provider<N> + Send + Sync + Clone + 'static,
{
    let code = provider.get_code_at(address).await?;
    if code.is_empty() {
        println!("Address has no code");
        return Ok(());
    }

    println!("Compiler:");
    let _ = detect_compiler_new(&code);

    if !evmole {
        return Ok(());
    }

    let contract = evmole::contract_info(
        evmole::ContractInfoArgs::new(&code).with_selectors().with_arguments().with_storage().with_state_mutability(),
    );
    println!("Functions:");
    for functions in contract.functions.unwrap_or_default() {
        let args = functions.arguments.unwrap_or_default().iter().map(|a| a.to_string()).collect::<Vec<String>>().join(", ");
        println!("-> {}({})", hex::encode_prefixed(functions.selector), args);
    }
    println!("Storage:");
    for storage in contract.storage.unwrap_or_default() {
        println!("-> {}: {}", hex::encode_prefixed(storage.slot), storage.r#type);
    }

    Ok(())
}
