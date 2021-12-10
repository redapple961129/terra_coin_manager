#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, CosmosMsg, BankMsg, QueryRequest, BalanceResponse, BankQuery,
};
use cw2::set_contract_version;
use cw_storage_plus::{U128Key};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ProjectResponse};
use crate::state::{Config, CONFIG, PROJECTSTATES, ProjectState, BackerState,
                    save_projectstate};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .admin
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(info.sender.clone());
    let wefund = msg
        .wefund
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(info.sender.clone());

    let config = Config {
        owner: owner.clone(),
        wefund: wefund.clone(),
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetWefund{ wefund } => try_setwefund(deps, info, wefund),
        ExecuteMsg::AddProject { 
            project_name, 
            project_wallet, 
            project_collected,
            creator_wallet,
            project_website,
            project_about,
            project_email,
            project_ecosystem,
            project_category } => 
            try_addproject(deps, info, 
                project_name, 
                project_wallet, 
                project_collected,
                creator_wallet,
                project_website,
                project_about,
                project_email,
                project_ecosystem,
                project_category),

        ExecuteMsg::Back2Project { project_id, backer_wallet } => 
            try_back2project(deps, info, project_id, backer_wallet),

        ExecuteMsg::CompleteProject{ project_id } =>
            try_completeproject(deps, info, project_id ),

        ExecuteMsg::FailProject{ project_id } =>
            try_failproject(deps, info, project_id),
    }
}
pub fn try_setwefund(deps:DepsMut, info:MessageInfo, wefund:String) 
            -> Result<Response, ContractError>
{
    let mut config = CONFIG.load(deps.storage).unwrap();
    config.wefund = deps.api.addr_validate(&wefund)
                                .unwrap_or(info.sender);
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}
pub fn try_completeproject(
    deps: DepsMut,
    _info: MessageInfo,
    _project_id: Uint128
) -> Result<Response, ContractError>
{
    remove_project(deps, _project_id);
    Ok(Response::new())
}
pub fn try_failproject(
    deps: DepsMut,
    _info: MessageInfo,
    _project_id: Uint128
) -> Result<Response, ContractError>
{
    remove_project(deps, _project_id);
    Ok(Response::new())
}
fn remove_project(deps:DepsMut, _project_id:Uint128)
    ->Result<Response, ContractError>
{
    let res = PROJECTSTATES.may_load(deps.storage, _project_id.u128().into());
    if res == Ok(None) {
        return Err(ContractError::NotRegisteredProject {});
    }
    PROJECTSTATES.remove(deps.storage, U128Key::new(_project_id.u128()));
    Ok(Response::new())
}
pub fn try_addproject(
    deps:DepsMut,
    _info: MessageInfo,
    _project_name: String, 
    _project_wallet: String, 
    _project_collected: Uint128,
    _creator_wallet: String,
    _project_website: String,
    _project_about: String,
    _project_email: String,
    _project_ecosystem: String,
    _project_category: String
) -> Result<Response, ContractError> 
{
    // let res = PROJECTSTATES.may_load(deps.storage, _project_id.u128().into());
    // if res != Ok(None) {//exist
    //     return Err(ContractError::AlreadyRegisteredProject {});
    // }
{
    // let all: StdResult<Vec<_>> = PROJECTCONTRACTS.range(deps.storage, None, None, 
    //     cosmwasm_std::Order::Ascending).collect();
    // let all = all.unwrap();
    // let mut prj_wallet:String = "".to_string();
    // for x in all{
    //     if x.1 == false{
    //         prj_wallet = String::from_utf8(x.0).unwrap();

    //         //convert to true on Map<Address, bool>
    //         let act = |a: Option<bool>| -> StdResult<_> { Ok(true) };
    //         PROJECTCONTRACTS.update(deps.storage, prj_wallet.clone(), act)?;
    //         break;
    //     }
    // }
    // if prj_wallet == "" {
    //     return Err(ContractError::NOTFOUNDAVAILABLEPROJECTCONTRACT{});
    // }
}
    let backer_states = Vec::new();
    let new_project:ProjectState = ProjectState{
        project_id: Uint128::zero(),
        project_name: _project_name, 
        project_wallet: _project_wallet, 
        project_collected: _project_collected,
        creator_wallet: _creator_wallet,
        project_website: _project_website ,
        project_about: _project_about,
        project_email: _project_email,
        project_ecosystem: _project_ecosystem,
        project_category: _project_category,
        backer_states: backer_states,
    };
        
    save_projectstate(deps, &new_project);

    Ok(Response::new()
        .add_attribute("action", "add project"))
}

pub fn try_back2project(
    deps:DepsMut, 
    info: MessageInfo,
    _project_id:Uint128, 
    _backer_wallet:String
) -> Result<Response, ContractError> 
{
    let res = PROJECTSTATES.may_load(deps.storage, _project_id.u128().into());
    if res == Ok(None) { //not exist
        return Err(ContractError::NotRegisteredProject {});
    }

    if info.funds.is_empty() || info.funds[0].amount.u128() < 4*(10^6) {
        return Err(ContractError::NeedCoin{});
    }

    let fee = 4*(10^6);
    let mut fund = info.funds[0].clone();
    fund.amount = Uint128::new(fund.amount.u128() - fee);
 
    //add new backer to PROJECTSTATE
    let new_baker:BackerState = BackerState{
        backer_wallet:_backer_wallet,
        amount: fund.clone(),
    };

    let mut x = PROJECTSTATES.load(deps.storage, _project_id.u128().into())?;
    x.backer_states.push(new_baker);
    let act = |a: Option<ProjectState>| -> StdResult<ProjectState> { 
        Ok(ProjectState {
            project_id: a.clone().unwrap().project_id,
            project_name: a.clone().unwrap().project_name,
            project_wallet: a.clone().unwrap().project_wallet,
            project_collected: a.clone().unwrap().project_collected,
            project_website: a.clone().unwrap().project_website,
            project_about: a.clone().unwrap().project_about,
            project_email: a.clone().unwrap().project_email,
            project_ecosystem: a.clone().unwrap().project_ecosystem,
            project_category: a.clone().unwrap().project_category,
            creator_wallet: a.clone().unwrap().creator_wallet,
            backer_states: x.backer_states.clone(),
        })
    };
    PROJECTSTATES.update(deps.storage, _project_id.u128().into(), act)?;


    let mut fund_project = fund.clone();  
    let amount_projectwallet = (fund_project.amount.u128()-fee) * 100 / 105;
    
    fund_project.amount = Uint128::new(amount_projectwallet);
    let bank_project = BankMsg::Send { 
        to_address: x.project_wallet.clone(),
        amount: vec![fund_project]
    };

    let config = CONFIG.load(deps.storage).unwrap();
    let mut fund_wefund = info.funds[0].clone();
    let amount_wefund = (fund_wefund.amount.u128()-fee) * 5 / 105;
    fund_wefund.amount = Uint128::new(amount_wefund);
    let bank_wefund = BankMsg::Send { 
        to_address: config.wefund.to_string(),
        amount: vec![fund] 
    };

    let mut collected = 0;
    for backer in x.backer_states{
        collected += backer.amount.amount.u128();
    }

    if collected >= x.project_collected.u128(){

    }

    Ok(Response::new()
    .add_messages(vec![
        CosmosMsg::Bank(bank_project),
        CosmosMsg::Bank(bank_wefund)])
    .add_attribute("action", "back to project")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance{ wallet } => to_binary(&query_balance(deps, wallet)?),
        QueryMsg::GetConfig{ } => to_binary(&query_getconfig(deps)?),
        QueryMsg::GetAllProject{ } => to_binary(&query_allproject(deps)?),
        QueryMsg::GetProject{ project_id } => to_binary(&query_project(deps, project_id)?),
        QueryMsg::GetBacker{ project_id } => to_binary(&query_backer(deps, project_id)?),
    }
}
fn query_balance(deps:Deps, wallet:String) -> StdResult<BalanceResponse>{
    let denom = String::from("uusd");
    let balance: BalanceResponse = deps.querier.query(
        &QueryRequest::Bank(BankQuery::Balance {
            address: wallet,
            denom,
        }
    ))?;
    Ok(balance)
}
fn query_getconfig(deps:Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage).unwrap();
    Ok(config)
}
fn query_allproject(deps:Deps) -> StdResult<Vec<ProjectState>> {
    let all: StdResult<Vec<_>> = PROJECTSTATES.range(deps.storage, None, None, 
        cosmwasm_std::Order::Ascending).collect();
    let all = all.unwrap();

    let mut all_project:Vec<ProjectState> = Vec::new();
    for x in all{
        all_project.push(x.1);
    }
    Ok(all_project)
}
fn query_backer(deps:Deps, id:Uint128) -> StdResult<Vec<BackerState>>{
    let x = PROJECTSTATES.load(deps.storage, id.u128().into())?;
    Ok(x.backer_states)
}
fn query_project(deps:Deps, id:Uint128) -> StdResult<ProjectState>{
    let x = PROJECTSTATES.load(deps.storage, id.u128().into())?;
    
    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, 
        MOCK_CONTRACT_ADDR, MockQuerier};
    use cosmwasm_std::{from_binary, Addr, CosmosMsg, WasmMsg,
        BankQuery, BalanceResponse, Coin };
    #[test]
    fn add_project(){
        let mut deps = mock_dependencies(&[]);
        
        let msg = InstantiateMsg{
            admin: Some(String::from("admin")),
            wefund: Some(String::from("Wefund")),
        };

        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

//add project        

       let msg = ExecuteMsg::AddProject{
            project_id: Uint128::new(1),
            project_wallet: String::from("project1"),
            project_collected: Uint128::new(5000),
            creator_wallet: String::from("creator1"),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(res.messages.len(), 0);

        let msg = ExecuteMsg::AddProject{
            project_id: Uint128::new(2),
            project_wallet: String::from("project2"),
            project_collected: Uint128::new(1000),
            creator_wallet: String::from("creator2"),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(res.messages.len(), 0);
//balance

//back 2 projct
        let info = mock_info("backer1", &[Coin::new(3150, "uusd")]);
        let msg = ExecuteMsg::Back2Project{
            project_id: Uint128::new(1),
            backer_wallet: String::from("backer1"),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
println!("back2project:{:?}", res);
        
        let info = mock_info("backer2", &[Coin::new(1234, "uusd")]);
        let msg = ExecuteMsg::Back2Project{
            project_id: Uint128::new(2),
            backer_wallet: String::from("backer2"),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
println!("back2project:{:?}", res);

//-Get Project-----------------
        let msg = QueryMsg::GetAllProject{};
        let allproject = query(deps.as_ref(), mock_env(), msg).unwrap();

        let res:Vec<ProjectState> = from_binary(&allproject).unwrap();
        println!("allproject {:?}", res );
//-Get Config-------------            
        let msg = QueryMsg::GetConfig{};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let config:Config= from_binary(&res).unwrap();
        println!("Config = {:?}", config);
//-Complte project--------------------------
        // let msg = ExecuteMsg::CompleteProject{project_id:Uint128::new(1)};
        // let res = execute(deps.as_mut(), mock_env(), info, msg);

//-Get project1 Balance-------------------
        let msg = QueryMsg::GetBalance{ wallet: String::from("project1")};
        let balance = query(deps.as_ref(), mock_env(), msg).unwrap();

        let res:BalanceResponse = from_binary(&balance).unwrap();
        println!("project1 Balance {:?}", res );
//-Get wefund Balance-------------------
        let msg = QueryMsg::GetBalance{ wallet: String::from("wefund")};
        let balance = query(deps.as_ref(), mock_env(), msg).unwrap();

        let res:BalanceResponse = from_binary(&balance).unwrap();
        println!("Wefund Balance {:?}", res );

//-Get project1 Balance-------------------
        let msg = QueryMsg::GetBalance{ wallet: String::from("project2")};
        let balance = query(deps.as_ref(), mock_env(), msg).unwrap();

        let res:BalanceResponse = from_binary(&balance).unwrap();
        println!("project2 Balance {:?}", res );
//-Get wefund Balance-------------------
        let msg = QueryMsg::GetBalance{ wallet: String::from("wefund")};
        let balance = query(deps.as_ref(), mock_env(), msg).unwrap();

        let res:BalanceResponse = from_binary(&balance).unwrap();
        println!("Wefund Balance {:?}", res );
    }
}
