use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use anyhow::{anyhow, bail, Context, Result};
use flutter_rust_bridge::frb;
use log::{debug, info};
use parking_lot::RwLock;
use xelis_common::api::{DataElement, DataValue};
use xelis_common::config::{COIN_DECIMALS, XELIS_ASSET};
use xelis_common::crypto::{Address, Hash, Hashable};
use xelis_common::serializer::Serializer;
use xelis_common::transaction::builder::{FeeBuilder, TransactionTypeBuilder, TransferBuilder};
use xelis_common::transaction::BurnPayload;
pub use xelis_common::transaction::Transaction;
use xelis_common::network::Network;

use xelis_common::utils::{format_coin, format_xelis};
pub use xelis_wallet::transaction_builder::TransactionBuilderState;
use xelis_wallet::wallet::Wallet;

use xelis_common::crypto::ecdlp::NoOpProgressTableGenerationReportFunction;
use crate::frb_generated::StreamSink;

pub struct XelisWallet {
    wallet: Arc<Wallet>,
}

pub fn create_xelis_wallet(
    _: String,
    _: String,
    _: Option<String>,
    precomputed_tables_path: Option<String>,
) -> Result<XelisWallet> {
    let precomputed_tables = Wallet::read_or_generate_precomputed_tables(
        precomputed_tables_path,
        NoOpProgressTableGenerationReportFunction,
    )?;
    todo!("")
}

impl XelisWallet {
    pub async fn events_stream(&self, _: StreamSink<String>) {
        // let mut _rx = self.wallet.subscribe_events().await;
        todo!("")
    }
}
