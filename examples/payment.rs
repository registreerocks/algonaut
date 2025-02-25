use algonaut::algod::AlgodBuilder;
use algonaut_core::MicroAlgos;
use algonaut_transaction::Pay;
use algonaut_transaction::{account::Account, TxnBuilder};
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let algod = AlgodBuilder::new()
        .bind(env::var("ALGOD_URL")?.as_ref())
        .auth(env::var("ALGOD_TOKEN")?.as_ref())
        .build_v2()?;

    let from = Account::from_mnemonic("fire enlist diesel stamp nuclear chunk student stumble call snow flock brush example slab guide choice option recall south kangaroo hundred matrix school above zero")?;
    let to = Account::from_mnemonic("since during average anxiety protect cherry club long lawsuit loan expand embark forum theory winter park twenty ball kangaroo cram burst board host ability left")?;

    let params = algod.suggested_transaction_params().await?;

    let t = TxnBuilder::with(
        params,
        Pay::new(from.address(), to.address(), MicroAlgos(123_456)).build(),
    )
    .build();

    let sign_response = from.sign_transaction(&t);
    let sign_response = sign_response.unwrap();

    let send_response = algod.broadcast_signed_transaction(&sign_response).await;
    println!("response: {:?}", send_response);

    Ok(())
}
