use alloy::{
    primitives::{address, Bytes, U256},
    sol,
    sol_types::SolCall,
};
use revm::{
    context::{tx::TxEnvBuilder, BlockEnv, CfgEnv, Evm as EVM, Journal, LocalContext, TxEnv},
    database::{CacheDB, EmptyDB, InMemoryDB},
    handler::{instructions::EthInstructions, EthFrame, EthPrecompiles},
    interpreter::interpreter::EthInterpreter,
    primitives::hardfork::SpecId,
    Context, ExecuteEvm,
};

type EvmContext =
    Context<BlockEnv, TxEnv, CfgEnv, InMemoryDB, Journal<InMemoryDB>, (), LocalContext>;

type EvmType =
    EVM<EvmContext, (), EthInstructions<EthInterpreter, EvmContext>, EthPrecompiles, EthFrame>;

sol! {
    #[sol(rpc)]
    contract EtherStore {
        mapping(address => uint256) public lastWithdrawTime;
        mapping(address => uint256) public balances;

        function depositFunds() public payable;
        function withdrawFunds(uint256 _weiToWithdraw) public;
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let spec_id: SpecId = SpecId::ARROW_GLACIER;
    let db: InMemoryDB = CacheDB::new(EmptyDB::default());
    let evm_ctx: EvmContext = Context::new(db, spec_id);

    let mut evm: EvmType = EVM::new(
        evm_ctx,
        EthInstructions::<EthInterpreter, EvmContext>::default(),
        EthPrecompiles::default(),
    );

    // modifications for "development" purposes
    evm.cfg.disable_base_fee = true;
    deposit_funds(&mut evm);

    Ok(())
}

pub fn deposit_funds(evm: &mut EvmType) {
    let calldata: Vec<u8> = EtherStore::withdrawFundsCall {
        _weiToWithdraw: U256::from(1000),
    }
    .abi_encode();

    let tx = TxEnvBuilder::new()
        .caller(address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"))
        .to(address!("0x5FbDB2315678afecb367f032d93F642f64180aa3"))
        .data(Bytes::from(calldata));

    match evm.transact(tx.build_fill()) {
        Ok(result_and_state) => {
            println!(
                "ExecResultAndState<ExecutionResult>: {:#?}",
                result_and_state
            );
        }
        Err(evm_error) => {
            println!("EVMError<Infallible>: {:#?}", evm_error);
        }
    }
}
