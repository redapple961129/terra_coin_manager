use cosmwasm_std::{Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub wefund: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetWefund { wefund: String },
    AddProject { project_name: String, project_wallet: String, 
        project_collected: Uint128, creator_wallet: String , 
        project_website: String, project_about: String,
        project_email: String, project_ecosystem:String,
        project_category:String},
    Back2Project { project_id: Uint128, backer_wallet: String},
    CompleteProject{ project_id:Uint128 },
    FailProject{project_id:Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig{},
    GetAllProject{},
    GetProject { project_id:Uint128 },
    GetBacker{ project_id:Uint128},
    GetBalance{ wallet:String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProjectResponse{
    pub project_id: Uint128,
    pub project_wallet: String,
    pub project_collected: Uint128,
    pub creator_wallet: String,
    pub balance: Uint128,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct BackerResponse{
//     pub project_id:Uint128,
//     pub baker_wallet: String,
//     pub amount: Coin,
// }
