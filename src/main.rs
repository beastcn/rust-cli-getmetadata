
use ethers::{
    prelude::*,
    providers::{Http, Provider},
};
use serde::Serialize;
use std::sync::Arc;

// Define the structure for JSON output
#[derive(Serialize)]
struct TokenMetadata {
    name: String,
    symbol: String,
    decimals: u8,
}

// Basic ERC20 ABI with just the functions we need for get_metadata
abigen!(
    ERC20,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
    ]"#,
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Check if an address is provided as argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <erc20_address>", args[0]);
        std::process::exit(1);
    }

    // Parse the address
    let address = args[1].parse::<Address>()?;

    // Connect to Ethereum mainnet using a public RPC endpoint. Created an alchemy project for this.
    let provider = Provider::<Http>::try_from(
        "https://eth-mainnet.g.alchemy.com/v2/wG-LsTvyJpd_ofx1QOVZGtRI01EBb3t8"
    )?;
    let provider = Arc::new(provider);

    // Create contract instance
    let contract = ERC20::new(address, provider);

    // Fetch token metadata
    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let decimals = contract.decimals().call().await?;

    // Create and serialize the token metadata
    let token_metadata = TokenMetadata {
        name,
        symbol,
        decimals,
    };

    // Print the JSON output
    println!("{}", serde_json::to_string_pretty(&token_metadata)?);

    Ok(())
} 