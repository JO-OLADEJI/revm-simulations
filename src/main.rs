use revm::{
    context::{BlockEnv, CfgEnv, Evm as EVM, Journal, LocalContext, TxEnv},
    database::{CacheDB, EmptyDB, InMemoryDB},
    handler::{instructions::EthInstructions, EthFrame, EthPrecompiles},
    interpreter::interpreter::EthInterpreter,
    primitives::hardfork::SpecId,
    Context,
};

pub type CustomEvmContext =
    Context<BlockEnv, TxEnv, CfgEnv, InMemoryDB, Journal<InMemoryDB>, (), LocalContext>;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let spec_id: SpecId = SpecId::ARROW_GLACIER;
    let db: InMemoryDB = CacheDB::new(EmptyDB::default());
    let evm_ctx: CustomEvmContext = Context::new(db, spec_id);

    let mut evm: EVM<_, _, _, _, EthFrame<EthInterpreter>> = EVM::new(
        evm_ctx,
        EthInstructions::<EthInterpreter, CustomEvmContext>::default(),
        EthPrecompiles::default(),
    );

    // modifications for "development" purposes
    evm.cfg.disable_base_fee = true;

    Ok(())
}
