use codec::Encode;
use frame_support::log::{error, trace};
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use sp_runtime::DispatchError;

/// contract extension for `FetchRandom`
pub struct FetchBalancesExtension;

impl<C: pallet_contracts::Config> ChainExtension<C> for FetchBalancesExtension {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
        where
            <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {

        match func_id {
            1001 => {
                let mut env = env.buf_in_buf_out();
                let mut account_slice: [u8; 32] = [0_u8; 32];
                account_slice[0..32].clone_from_slice(&env.read(32)?[..]);
                let account_id: super::AccountId = account_slice.into();
                error!(
                    target: "runtime",
                    "[ChainExtension]|call|account_id:{:}",
                    account_id
                );
                // let account_id = super::AccountId::default();
                // let balances: (Option<super::Balance>, Option<super::Balance>) = super::OffchainWorkerModule::account_balance(account_id);
                // hardcode to verify the function
                let balances: (Option<super::Balance>, Option<super::Balance>)  = (Some(1_u128), Some(1_u128));
                let balances_vec = balances.encode();
                error!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}",
                    func_id
                );
                env.write(&balances_vec, false, None)
                    .map_err(|_| DispatchError::Other("ChainExtension failed to call account_balance"))?;
            }

            _ => {
                error!("call an unregistered `func_id`, func_id:{:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"));
            }
        }
        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}