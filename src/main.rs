use ethers::{
    abi::AbiEncode,
    providers::{Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, TransactionRequest, U256},
    utils::Ganache,
};
use eyre::{ContextCompat, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let ganache = Ganache::new()
        .mnemonic("gas monster ski craft below illegal discover limit dog bundle bus artefact")
        .spawn();
    println!("HTTP Endpoint: {}", ganache.endpoint());

    //First wallet managed by ganache - eth environment for testing
    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let wallet_address = wallet.address();
    println!("Default wallet address: {}", wallet_address);

    // A provider is an Eth JsonRPC client which connects to the ganache endpoint (eth node)
    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));

    // Query the balance of our account
    let first_balance = provider.get_balance(wallet_address, None).await?;
    println!("Wallet first address balance: {}", first_balance);

    //Query balance of random account
    let other_addr_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
    let other_addr = other_addr_hex.parse::<Address>()?;
    let other_balance = provider.get_balance(other_addr, None).await?;
    println!("Balance for address {}: {}", other_addr_hex, other_balance);

    // Create a transaction to transfer 1000 wei to other_addr
    let tx = TransactionRequest::pay(other_addr, U256::from(1000u64)).from(wallet_address);
    //Send the transaction and wait for receipt
    let receipt = provider
        .send_transaction(tx, None)
        .await?
        .log_msg("Pending transfer")
        .await?
        .context("Missing receipt")?;

    println!(
        "TX mined in block {}",
        receipt.block_number.context("Con not get blocknumber")?
    );
    println!(
        "Balance of {} {}",
        other_addr_hex,
        provider.get_balance(other_addr, None).await?
    );

    Ok(())
}
