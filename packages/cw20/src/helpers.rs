#![allow(clippy::field_reassign_with_default)] // see https://github.com/CosmWasm/cosmwasm/issues/685

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, CosmosMsg, HumanAddr, Querier, QuerierWrapper, StdResult, Uint128, WasmMsg,
    WasmQuery,
};

use crate::{
    AllowanceResponse, BalanceResponse, Cw20HandleMsg, Cw20QueryMsg, MinterResponse,
    TokenInfoResponse,
};

/// Cw20Contract is a wrapper around HumanAddr that provides a lot of helpers
/// for working with this.
///
/// If you wish to persist this, convert to Cw20CanonicalContract via .canonical()
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw20Contract(pub HumanAddr);

impl Cw20Contract {
    pub fn addr(&self) -> HumanAddr {
        self.0.clone()
    }

    pub fn call<T: Into<Cw20HandleMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr(),
            msg,
            send: vec![],
        }
        .into())
    }

    /// Get token balance for the given address
    pub fn balance<Q: Querier>(&self, querier: &Q, address: HumanAddr) -> StdResult<Uint128> {
        let msg = Cw20QueryMsg::Balance { address };
        let query = WasmQuery::Smart {
            contract_addr: self.addr(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: BalanceResponse = QuerierWrapper::new(querier).query(&query)?;
        Ok(res.balance)
    }

    /// Get metadata from the contract. This is a good check that the address
    /// is a valid Cw20 contract.
    pub fn meta<Q: Querier>(&self, querier: &Q) -> StdResult<TokenInfoResponse> {
        let msg = Cw20QueryMsg::TokenInfo {};
        let query = WasmQuery::Smart {
            contract_addr: self.addr(),
            msg: to_binary(&msg)?,
        }
        .into();
        QuerierWrapper::new(querier).query(&query)
    }

    /// Get allowance of spender to use owner's account
    pub fn allowance<Q: Querier>(
        &self,
        querier: &Q,
        owner: HumanAddr,
        spender: HumanAddr,
    ) -> StdResult<AllowanceResponse> {
        let msg = Cw20QueryMsg::Allowance { owner, spender };
        let query = WasmQuery::Smart {
            contract_addr: self.addr(),
            msg: to_binary(&msg)?,
        }
        .into();
        QuerierWrapper::new(querier).query(&query)
    }

    /// Find info on who can mint, and how much
    pub fn minter<Q: Querier>(&self, querier: &Q) -> StdResult<Option<MinterResponse>> {
        let msg = Cw20QueryMsg::Minter {};
        let query = WasmQuery::Smart {
            contract_addr: self.addr(),
            msg: to_binary(&msg)?,
        }
        .into();
        QuerierWrapper::new(querier).query(&query)
    }

    /// returns true if the contract supports the allowance extension
    pub fn has_allowance<Q: Querier>(&self, querier: &Q) -> bool {
        self.allowance(querier, self.addr(), self.addr()).is_ok()
    }

    /// returns true if the contract supports the mintable extension
    pub fn is_mintable<Q: Querier>(&self, querier: &Q) -> bool {
        self.minter(querier).is_ok()
    }
}
