use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, DepsMut, StdResult, Uint128, HumanAddr, Coin};
use cw_storage_plus::{Item, Map, U128Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub cw20_addr: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pot {
    /// target_addr is the address that will receive the pot
    pub target_addr: Addr,
    /// threshold_amount is the token threshold amount
    pub threshold: Uint128,
    /// collected keeps information on how much is collected for this pot.
    pub collected: Uint128,
}
/// POT_SEQ holds the last pot ID
pub const POT_SEQ: Item<Uint128> = Item::new("pot_seq");
pub const POTS: Map<U128Key, Pot> = Map::new("pot");

pub fn save_pot(deps: DepsMut, pot: &Pot) -> StdResult<()> {
    // increment id if exists, or return 1
    let id = POT_SEQ.load(deps.storage)?;
    let id = id.checked_add(Uint128::new(1))?;
    POT_SEQ.save(deps.storage, &id)?;

    // save pot with id
    POTS.save(deps.storage, id.u128().into(), pot)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BackerState{
    pub backer_wallet: HumanAddr,
    pub amount: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProjectState{
    pub project_id: u32,
    pub project_wallet: HumanAddr,
    pub backer_states: Vec<BackerState>,
}
pub const PROJECTSTATES: Vec<ProjectState> = Vec::new();