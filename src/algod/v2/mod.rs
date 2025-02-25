use algonaut_client::algod::v2::Client;
use algonaut_core::{Address, Round, SuggestedTransactionParams, ToMsgPack};
use algonaut_model::algod::v2::{
    Account, Application, Block, Catchup, CompiledTealWithHash, DryrunRequest, DryrunResponse,
    GenesisBlock, KeyRegistration, NodeStatus, PendingTransaction, PendingTransactions, Supply,
    TransactionParams, TransactionResponse, Version,
};
use algonaut_transaction::SignedTransaction;

use crate::error::AlgonautError;

pub struct Algod {
    pub(crate) client: Client,
}

impl Algod {
    pub fn new(client: Client) -> Algod {
        Algod { client }
    }

    /// Returns the entire genesis file in json.
    pub async fn genesis(&self) -> Result<GenesisBlock, AlgonautError> {
        Ok(self.client.genesis().await?)
    }

    /// Returns Ok if healthy
    pub async fn health(&self) -> Result<(), AlgonautError> {
        Ok(self.client.health().await?)
    }

    /// Return metrics about algod functioning.
    pub async fn metrics(&self) -> Result<String, AlgonautError> {
        Ok(self.client.metrics().await?)
    }

    /// Get account information.
    /// Description Given a specific account public key, this call returns the accounts status,
    /// balance and spendable amounts
    pub async fn account_information(&self, address: &Address) -> Result<Account, AlgonautError> {
        Ok(self
            .client
            .account_information(&address.to_string())
            .await?)
    }

    /// Get a list of unconfirmed transactions currently in the transaction pool by address.
    /// Description: Get the list of pending transactions by address, sorted by priority,
    /// in decreasing order, truncated at the end at MAX. If MAX = 0, returns all pending transactions.
    pub async fn pending_transactions_for(
        &self,
        address: &Address,
        max: u64,
    ) -> Result<PendingTransactions, AlgonautError> {
        Ok(self
            .client
            .pending_transactions_for(&address.to_string(), max)
            .await?)
    }

    /// Get application information.
    ///
    /// Given a application id, it returns application information including creator,
    /// approval and clear programs, global and local schemas, and global state.
    pub async fn application_information(&self, id: usize) -> Result<Application, AlgonautError> {
        Ok(self.client.application_information(id).await?)
    }

    /// Get asset information.
    ///
    /// Given a asset id, it returns asset information including creator, name,
    /// total supply and special addresses.
    pub async fn asset_information(&self, id: usize) -> Result<Application, AlgonautError> {
        Ok(self.client.asset_information(id).await?)
    }

    /// Get the block for the given round.
    pub async fn block(&self, round: Round) -> Result<Block, AlgonautError> {
        Ok(self.client.block(round).await?)
    }

    /// Starts a catchpoint catchup.
    pub async fn start_catchup(&self, catchpoint: &str) -> Result<Catchup, AlgonautError> {
        Ok(self.client.start_catchup(catchpoint).await?)
    }

    /// Aborts a catchpoint catchup.
    pub async fn abort_catchup(&self, catchpoint: &str) -> Result<Catchup, AlgonautError> {
        Ok(self.client.abort_catchup(catchpoint).await?)
    }

    /// Get the current supply reported by the ledger.
    pub async fn ledger_supply(&self) -> Result<Supply, AlgonautError> {
        Ok(self.client.ledger_supply().await?)
    }

    /// Generate (or renew) and register participation keys on the node for a given account address.
    ///
    /// address: The account-id to update, or all to update all accounts.
    /// fee: The fee to use when submitting key registration transactions. Defaults to the suggested
    /// fee. (default = 1000)
    /// key-dilution: value to use for two-level participation key.
    /// no-wait: Don't wait for transaction to commit before returning response.
    /// round-last-valid: The last round for which the generated participation keys will be valid.
    pub async fn register_participation_keys(
        &self,
        address: &Address,
        params: &KeyRegistration,
    ) -> Result<String, AlgonautError> {
        Ok(self
            .client
            .register_participation_keys(address, params)
            .await?)
    }

    /// Special management endpoint to shutdown the node. Optionally provide a timeout parameter
    /// to indicate that the node should begin shutting down after a number of seconds.
    pub async fn shutdown(&self, timeout: usize) -> Result<(), AlgonautError> {
        Ok(self.client.shutdown(timeout).await?)
    }

    /// Gets the current node status.
    pub async fn status(&self) -> Result<NodeStatus, AlgonautError> {
        Ok(self.client.status().await?)
    }

    /// Gets the node status after waiting for the given round.
    pub async fn status_after_round(&self, round: Round) -> Result<NodeStatus, AlgonautError> {
        Ok(self.client.status_after_round(round).await?)
    }

    /// Compile TEAL source code to binary, produce its hash.
    ///
    /// Given TEAL source code in plain text, return base64 encoded program bytes and base32
    /// SHA512_256 hash of program bytes (Address style). This endpoint is only enabled when
    /// a node's configuration file sets EnableDeveloperAPI to true.
    pub async fn compile_teal(&self, teal: &[u8]) -> Result<CompiledTealWithHash, AlgonautError> {
        Ok(self.client.compile_teal(teal.to_vec()).await?)
    }

    /// Provide debugging information for a transaction (or group).
    ///
    /// Executes TEAL program(s) in context and returns debugging information about the execution.
    /// This endpoint is only enabled when a node's configureation file sets EnableDeveloperAPI
    /// to true.
    pub async fn dryrun_teal(&self, req: &DryrunRequest) -> Result<DryrunResponse, AlgonautError> {
        Ok(self.client.dryrun_teal(req).await?)
    }

    /// Broadcasts a transaction to the network.
    pub async fn broadcast_signed_transaction(
        &self,
        txn: &SignedTransaction,
    ) -> Result<TransactionResponse, AlgonautError> {
        Ok(self.broadcast_raw_transaction(&txn.to_msg_pack()?).await?)
    }

    /// Broadcasts a transaction group to the network.
    ///
    /// Atomic if the transactions share a [group](algonaut_transaction::transaction::Transaction::group)
    pub async fn broadcast_signed_transactions(
        &self,
        txns: &[SignedTransaction],
    ) -> Result<TransactionResponse, AlgonautError> {
        let mut bytes = vec![];
        for t in txns {
            bytes.push(t.to_msg_pack()?);
        }
        Ok(self.broadcast_raw_transaction(&bytes.concat()).await?)
    }

    /// Broadcasts raw transactions to the network.
    ///
    /// When passing multiple transactions, the transactions are atomic if they share a [group](algonaut_transaction::transaction::Transaction::group)
    ///
    /// Use this when using a third party (e.g. KMD) that delivers directly the serialized signed transaction.
    ///
    /// Otherwise, prefer [broadcast_signed_transaction](Self::broadcast_signed_transaction) or [broadcast_signed_transactions][Self::broadcast_signed_transactions]

    pub async fn broadcast_raw_transaction(
        &self,
        rawtxn: &[u8],
    ) -> Result<TransactionResponse, AlgonautError> {
        Ok(self.client.broadcast_raw_transaction(rawtxn).await?)
    }

    /// Get parameters for constructing a new transaction.
    pub async fn transaction_params(&self) -> Result<TransactionParams, AlgonautError> {
        Ok(self.client.transaction_params().await?)
    }

    /// Get suggested parameters for constructing a new transaction.
    pub async fn suggested_transaction_params(
        &self,
    ) -> Result<SuggestedTransactionParams, AlgonautError> {
        let params = self.client.transaction_params().await?;
        Ok(SuggestedTransactionParams {
            genesis_id: params.genesis_id,
            genesis_hash: params.genesis_hash,
            consensus_version: params.consensus_version,
            fee: params.fee,
            min_fee: params.min_fee,
            first_valid: params.last_round,
            last_valid: params.last_round + 1000,
        })
    }

    /// Get a list of unconfirmed transactions currently in the transaction pool.
    ///
    /// Get the list of pending transactions, sorted by priority, in decreasing order,
    /// truncated at the end at MAX. If MAX = 0, returns all pending transactions.
    pub async fn pending_transactions(
        &self,
        max: u64,
    ) -> Result<PendingTransactions, AlgonautError> {
        Ok(self.client.pending_transactions(max).await?)
    }

    /// Get a specific pending transaction.
    ///
    /// Given a transaction id of a recently submitted transaction, it returns information about
    /// it. There are several cases when this might succeed:
    /// - transaction committed (committed round > 0)
    /// - transaction still in the pool (committed round = 0, pool error = "")
    /// - transaction removed from pool due to error (committed round = 0, pool error != "")
    ///
    /// Or the transaction may have happened sufficiently long ago that the node no longer remembers
    /// it, and this will return an error.
    pub async fn pending_transaction_with_id(
        &self,
        txid: &str,
    ) -> Result<PendingTransaction, AlgonautError> {
        Ok(self.client.pending_transaction_with_id(txid).await?)
    }

    /// Retrieves the current version
    pub async fn versions(&self) -> Result<Version, AlgonautError> {
        Ok(self.client.versions().await?)
    }
}
