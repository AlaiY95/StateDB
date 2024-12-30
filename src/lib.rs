use alloy::primitives::{Address, U256};
use reth::api::NodeTypesWithDBAdapter;
use reth::providers::{ProviderFactory, StateProviderBox};
use reth::revm::db::AccountState;
use reth::rpc::types::AccountInfo;
use reth_db::DatabaseEnv;
use reth_node_ethereum::EthereumNode;
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::sync::RwLock;

use eyre::Result;

pub struct StateDB {
    db_provider: RwLock<StateProviderBox>,
    provider_factory: ProviderFactory<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>,
    accounts: HashMap<Address, StateDBAccount>,
    db_block: AtomicU64,
}

#[derive(Default)]
pub struct StateDBAccount {
    pub info: AccountInfo,
    pub state: AccountState,
    pub storage: HashMap<U256, U256>,
}

impl StateDBAccount {
    pub fn new_not_existing() -> Self {
        Self {
            state: AccountState::NotExisting,
            ..Default::default()
        }
    }
}
