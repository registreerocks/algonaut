use algonaut::algod::AlgodBuilder;
use algonaut::core::{MicroAlgos, ToMsgPack};
use algonaut::crypto::mnemonic;
use algonaut::transaction::account::Account;
use algonaut::transaction::{Pay, TxnBuilder};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let algod = AlgodBuilder::new()
        .bind(env::var("ALGOD_URL")?.as_ref())
        .auth(env::var("ALGOD_TOKEN")?.as_ref())
        .build_v2()?;

    // print algod status
    let node_status = algod.status().await?;
    println!("node_status: {:?}", node_status);

    let account = Account::generate();
    println!("Public Key: {:?}", account.address().to_string());

    let m = mnemonic::from_key(&account.seed())?;
    println!("Backup phrase: {}", m);

    let params = algod.suggested_transaction_params().await?;

    let t = TxnBuilder::with(
        params,
        Pay::new(
            account.address(),
            "4MYUHDWHWXAKA5KA7U5PEN646VYUANBFXVJNONBK3TIMHEMWMD4UBOJBI4".parse()?,
            MicroAlgos(123_456),
        )
        .build(),
    )
    .build();

    println!("Made unsigned transaction: {:?}", t);

    // sign the transaction
    let signed_transaction = account.sign_transaction(&t)?;
    let bytes = signed_transaction.to_msg_pack()?;

    let filename = "./signed.tx";
    let mut f = File::create(filename)?;
    f.write_all(&bytes)?;

    println!("Saved signed transaction to file: {}", filename);

    Ok(())
}
