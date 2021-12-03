#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, CosmosMsg, BankMsg, Coin, WasmMsg, wasm_execute
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, PotResponse, QueryMsg, ReceiveMsg, 
    ProjectResponse};
use crate::state::{save_pot, Config, Pot, CONFIG, POTS, POT_SEQ,
    PROJECTSTATES, ProjectState, BackerState, PROJECTCONTRACTS};
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg};

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
        .unwrap_or(info.sender);
    let config = Config {
        owner: owner.clone(),
        cw20_addr: deps.api.addr_validate(msg.cw20_addr.as_str())?,
    };
    CONFIG.save(deps.storage, &config)?;

    // init pot sequence
    POT_SEQ.save(deps.storage, &Uint128::new(0))?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner)
        .add_attribute("cw20_addr", msg.cw20_addr))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePot {
            target_addr,
            threshold,
        } => execute_create_pot(deps, info, target_addr, threshold),
        ExecuteMsg::Receive(msg) => execute_receive(deps, info, msg),
        ExecuteMsg::AddProject { project_id, project_wallet } => 
            try_addproject(deps, project_id, project_wallet),
        ExecuteMsg::Back2Project { project_id, backer_wallet } => 
            try_back2project(deps, info, project_id, backer_wallet),
        ExecuteMsg::AddContract { contract } => try_addcontract(deps, contract),
    }
}
pub fn try_addcontract(deps:DepsMut, contract:String)->Result<Response, ContractError>
{
    let res = PROJECTCONTRACTS.may_load(deps.storage, contract.clone());
    if res != Ok(None){ //exist
        return Err(ContractError::AlreadyRegisteredContract{});
    }
    let bfree:bool = false;
    PROJECTCONTRACTS.save(deps.storage, contract, &bfree)?;
    Ok(Response::new()
        .add_attribute("action", "add contract"))
}
pub fn try_addproject(
    deps:DepsMut,
    _project_id:Uint128, 
    _project_wallet:String,
) -> Result<Response, ContractError> 
{
    let res = PROJECTSTATES.may_load(deps.storage, _project_id.u128().into());
    if res != Ok(None) {//exist
        return Err(ContractError::AlreadyRegisteredProject {});
    }

    let all: StdResult<Vec<_>> = PROJECTCONTRACTS.range(deps.storage, None, None, 
        cosmwasm_std::Order::Ascending).collect();
    let all = all.unwrap();
    let mut prj_wallet:String = "".to_string();
    for x in all{
        if x.1 == false{
            prj_wallet = String::from_utf8(x.0).unwrap();

            //convert to true on Map<Address, bool>
            let act = |a: Option<bool>| -> StdResult<_> { Ok(true) };
            PROJECTCONTRACTS.update(deps.storage, prj_wallet.clone(), act)?;
            break;
        }
    }
    if prj_wallet == "" {
        return Err(ContractError::NOTFOUNDAVAILABLEPROJECTCONTRACT{});
    }

    let backer_states = Vec::new();
    let new_project:ProjectState = ProjectState{
        project_id:_project_id, 
        project_wallet:prj_wallet,
        backer_states};
        
    PROJECTSTATES.save(deps.storage, _project_id.u128().into(), &new_project)?;

    Ok(Response::new()
        .add_attribute("action", "add project"))
}
pub fn try_back2project(deps:DepsMut, info: MessageInfo,
    _project_id:Uint128, 
    _backer_wallet:String
) -> Result<Response, ContractError> 
{
    let res = PROJECTSTATES.may_load(deps.storage, _project_id.u128().into());
    if res == Ok(None) { //not exist
        return Err(ContractError::NotRegisteredProject {});
    }

    if info.funds.is_empty() {
        return Err(ContractError::NeedCoin{});
    }

    let mut x = PROJECTSTATES.load(deps.storage, _project_id.u128().into())?;
    
    let to_address = x.project_wallet.clone();
    let amount = info.funds[0].clone();//Coin::new(123, "ucosm");//

    let bank = BankMsg::Send { to_address: to_address.to_string(), amount: info.funds };
    let res:Response = Response::new()
        .add_messages(vec![CosmosMsg::Bank(bank)]);

    if res.messages.len() == 0 {
        return Err(ContractError::COULDNOTTRANSFER {});
        //println!("{:?}", res.messages);
    }

    let new_baker:BackerState = BackerState{
        backer_wallet:_backer_wallet,
        amount: amount,
    };

    x.backer_states.push(new_baker);

    let act = |a: Option<ProjectState>| -> StdResult<ProjectState> { 
        Ok(ProjectState {
            project_wallet: a.clone().unwrap().project_wallet,
            project_id: a.clone().unwrap().project_id,
            backer_states: x.backer_states,
        })
    };

    // let act = |d: Option<ProjectState>| -> StdResult<ProjectState> {
    //     match d {
    //         Some(one) => Ok(ProjectState {
    //             project_wallet: one.project_wallet,
    //             project_id: one.project_id,
    //             backer_states: x.backer_states,
    //         }),
    //         None => Ok(ProjectState {
    //             project_wallet: "0".into(),
    //             project_id: Uint128::new(0),
    //             backer_states: Vec::new(),
    //         }),
    //     }
    // };

    PROJECTSTATES.update(deps.storage, _project_id.u128().into(), act)?;
    Ok(Response::new()
        .add_attribute("action", "add backer to project")
    )
}
pub fn execute_create_pot(
    deps: DepsMut,
    info: MessageInfo,
    target_addr: String,
    threshold: Uint128,
) -> Result<Response, ContractError> {
    // owner authentication
    let config = CONFIG.load(deps.storage)?;
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    // create and save pot
    let pot = Pot {
        target_addr: deps.api.addr_validate(target_addr.as_str())?,
        threshold,
        collected: Uint128::zero(),
    };
    save_pot(deps, &pot)?;

    Ok(Response::new()
        .add_attribute("action", "execute_create_pot")
        .add_attribute("target_addr", target_addr)
        .add_attribute("threshold_amount", threshold))
}

pub fn execute_receive(
    deps: DepsMut,
    info: MessageInfo,
    wrapped: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // cw20 address authentication
    let config = CONFIG.load(deps.storage)?;
    if config.cw20_addr != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let msg: ReceiveMsg = from_binary(&wrapped.msg)?;
    match msg {
        ReceiveMsg::Send { id } => receive_send(deps, id, wrapped.amount, info.sender),
    }
}

pub fn receive_send(
    deps: DepsMut,
    pot_id: Uint128,
    amount: Uint128,
    cw20_addr: Addr,
) -> Result<Response, ContractError> {
    // load pot
    let mut pot = POTS.load(deps.storage, pot_id.u128().into())?;

    pot.collected += amount;

    POTS.save(deps.storage, pot_id.u128().into(), &pot)?;

    let mut res = Response::new()
        .add_attribute("action", "receive_send")
        .add_attribute("pot_id", pot_id)
        .add_attribute("collected", pot.collected)
        .add_attribute("threshold", pot.threshold);

    if pot.collected >= pot.threshold {
        // Cw20Contract is a function helper that provides several queries and message builder.
        let cw20 = Cw20Contract(cw20_addr);
        // Build a cw20 transfer send msg, that send collected funds to target address
        let msg = cw20.call(Cw20ExecuteMsg::Transfer {
            recipient: pot.target_addr.into_string(),
            amount: pot.collected,
        })?;
        res = res.add_message(msg);
    }

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPot { id } => to_binary(&query_pot(deps, id)?),
        QueryMsg::GetProject{ id } => to_binary(&query_project(deps, id)?),
        QueryMsg::GetBacker{ id } => to_binary(&query_backer(deps, id)?),
    }
}
fn query_backer(deps:Deps, id:Uint128) -> StdResult<usize>{
    let x = PROJECTSTATES.load(deps.storage, id.u128().into())?;
    let res = x.backer_states.len();

    Ok(res)
}
fn query_project(deps:Deps, id:Uint128) -> StdResult<ProjectResponse>{
    let x = PROJECTSTATES.load(deps.storage, id.u128().into())?;
    
    let res = ProjectResponse{
        project_id: x.project_id, 
        project_wallet: x.project_wallet.clone()
    };

    Ok(res)
}
fn query_pot(deps: Deps, id: Uint128) -> StdResult<PotResponse> {
    let pot = POTS.load(deps.storage, id.u128().into())?;
    Ok(PotResponse {
        target_addr: pot.target_addr.into_string(),
        collected: pot.collected,
        threshold: pot.threshold,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{from_binary, Addr, CosmosMsg, WasmMsg};

    #[test]
    fn add_project(){
        let mut deps = mock_dependencies(&[]);
        
        let msg = InstantiateMsg{
            admin:None,
            cw20_addr: String::from(MOCK_CONTRACT_ADDR),
        };

        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
//add contract
        let msg = ExecuteMsg::AddContract{
            contract:String::from("wersome1"),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
        let msg = ExecuteMsg::AddContract{
            contract:String::from("wersome2"),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
//add project        
        let msg = ExecuteMsg::AddProject{
            project_id: Uint128::new(100),
            project_wallet: String::from("some"),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(res.messages.len(), 0);

        let msg = ExecuteMsg::AddProject{
            project_id: Uint128::new(101),
            project_wallet: String::from("some"),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(res.messages.len(), 0);

//back 2 projct
        let msg = ExecuteMsg::Back2Project{
            project_id: Uint128::new(100),
            backer_wallet: String::from("some"),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(res.messages.len(), 0);
        
        let msg = ExecuteMsg::Back2Project{
            project_id: Uint128::new(100),
            backer_wallet: String::from("some"),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(res.messages.len(), 0);
   
        let msg = QueryMsg::GetProject{id:Uint128::new(101)};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        
        let prj:ProjectResponse = from_binary(&res).unwrap();
        println!("project {:?}:{:?}", prj.project_id, prj.project_wallet );
        // assert_eq!(
        //     prj,
        //     ProjectResponse{
        //         project_id: Uint128::new(98),
        //         project_wallet: String::from("some"),                
        //     }
        // )

        let msg = QueryMsg::GetBacker{id:Uint128::new(100)};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let nlen:usize = from_binary(&res).unwrap();
        println!("backer count = {:?}", nlen);
    }
    #[test]
    fn create_pot() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            admin: None,
            cw20_addr: String::from(MOCK_CONTRACT_ADDR),
        };
        let info = mock_info("creator", &[]);

        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // should create pot
        let msg = ExecuteMsg::CreatePot {
            target_addr: String::from("Some"),
            threshold: Uint128::new(100),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.messages.len(), 0);

        // query pot
        let msg = QueryMsg::GetPot {
            id: Uint128::new(1),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let pot: Pot = from_binary(&res).unwrap();
        assert_eq!(
            pot,
            Pot {
                target_addr: Addr::unchecked("Some"),
                collected: Default::default(),
                threshold: Uint128::new(100)
            }
        );
    }

    #[test]
    fn test_receive_send() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            admin: None,
            cw20_addr: String::from("cw20"),
        };
        let mut info = mock_info("creator", &[]);

        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // should create pot
        let msg = ExecuteMsg::CreatePot {
            target_addr: String::from("Some"),
            threshold: Uint128::new(100),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(res.messages.len(), 0);

        let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: String::from("cw20"),
            amount: Uint128::new(55),
            msg: to_binary(&ReceiveMsg::Send {
                id: Uint128::new(1),
            })
            .unwrap(),
        });
        info.sender = Addr::unchecked("cw20");
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // query pot
        let msg = QueryMsg::GetPot {
            id: Uint128::new(1),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let pot: Pot = from_binary(&res).unwrap();
        assert_eq!(
            pot,
            Pot {
                target_addr: Addr::unchecked("Some"),
                collected: Uint128::new(55),
                threshold: Uint128::new(100)
            }
        );

        let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: String::from("cw20"),
            amount: Uint128::new(55),
            msg: to_binary(&ReceiveMsg::Send {
                id: Uint128::new(1),
            })
            .unwrap(),
        });
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        let msg = res.messages[0].clone().msg;
        assert_eq!(
            msg,
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: String::from("cw20"),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: String::from("Some"),
                    amount: Uint128::new(110)
                })
                .unwrap(),
                funds: vec![]
            })
        );

        // query pot
        let msg = QueryMsg::GetPot {
            id: Uint128::new(1),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let pot: Pot = from_binary(&res).unwrap();
        assert_eq!(
            pot,
            Pot {
                target_addr: Addr::unchecked("Some"),
                collected: Uint128::new(110),
                threshold: Uint128::new(100)
            }
        );
    }
}
