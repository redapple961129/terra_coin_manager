use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Coin};
use cw_storage_plus::{Item, Map, U128Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub wefund: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BackerState{
    pub backer_wallet: String,
    pub amount: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProjectState{
    pub project_id: Uint128,
    pub project_wallet: String,
    pub project_collected: Uint128,
    pub creator_wallet: String,
    pub backer_states:Vec<BackerState>,
}

pub const PROJECTSTATES: Map<U128Key, ProjectState> = Map::new("prj");