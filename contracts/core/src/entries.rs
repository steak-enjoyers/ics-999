use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, IbcBasicResponse, IbcChannelCloseMsg,
    IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcPacketAckMsg,
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractResult,
    execute,
    ibc,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query,
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    execute::init(deps, msg)
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> ContractResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query::config(deps)?),
        QueryMsg::Account {
            connection_id,
            controller,
        } => to_binary(&query::account(deps, connection_id, controller)?),
        QueryMsg::Accounts {
            start_after,
            limit,
        } => to_binary(&query::accounts(deps, start_after, limit)?),
        QueryMsg::ActiveChannel {
            connection_id,
        } => to_binary(&query::active_channel(deps, connection_id)?),
        QueryMsg::ActiveChannels {
            start_after,
            limit,
        } => to_binary(&query::active_channels(deps, start_after, limit)?),
    }
}

#[entry_point]
pub fn ibc_channel_open(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> ContractResult<IbcChannelOpenResponse> {
    match msg {
        IbcChannelOpenMsg::OpenInit {
            channel,
        } => ibc::open_init(deps, channel),
        IbcChannelOpenMsg::OpenTry {
            channel,
            counterparty_version,
        } => ibc::open_try(deps, channel, counterparty_version),
    }
}

#[entry_point]
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> ContractResult<IbcBasicResponse> {
    ibc::open_connect(deps, msg.channel(), msg.counterparty_version())
}

#[entry_point]
pub fn ibc_channel_close(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> ContractResult<IbcBasicResponse> {
    ibc::close(msg)
}

#[entry_point]
pub fn ibc_packet_receive(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketReceiveMsg,
) -> ContractResult<IbcReceiveResponse> {
    todo!();
}

#[entry_point]
pub fn ibc_packet_ack(
    _deps: DepsMut,
    _env: Env,
    _ack: IbcPacketAckMsg,
) -> ContractResult<IbcBasicResponse> {
    todo!();
}

#[entry_point]
pub fn ibc_packet_timeout(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> ContractResult<IbcBasicResponse> {
    todo!();
}
