use jsonrpsee::RpcModule;
use runtime::interface::{AccountId, Nonce, OpaqueBlock};
use sc_transaction_pool_api::TransactionPool;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use std::sync::Arc;
use substrate_frame_rpc_system::{System, SystemApiServer};

pub use sc_rpc_api::DenyUnsafe;

pub struct FullDeps<C, P> {
    pub client: Arc<C>,
    pub pool: Arc<P>,
    pub deny_unsafe: DenyUnsafe,
}

pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: Send
        + Sync
        + sp_api::ProvideRuntimeApi<OpaqueBlock>
        + HeaderBackend<OpaqueBlock>
        + HeaderMetadata<OpaqueBlock, Error = BlockChainError>
        + 'static,
    C::Api: sp_block_builder::BlockBuilder<OpaqueBlock>,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<OpaqueBlock, AccountId, Nonce>,
    P: TransactionPool + 'static,
{
    let mut module = RpcModule::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe,
    } = deps;

    module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;

    Ok(module)
}
