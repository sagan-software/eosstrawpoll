import {
    ActionType,
    UpsertAction,
    CheckAction,
    CheckAllAction,
    SetOkAction,
    SetErrAction,
    RemoveAction,
    Action,
} from './actions';
import {
    initialState,
    State,
    serverToUrl,
    RpcServerStateType,
    UnknownRpcServerState,
    CheckingRpcServerState,
    OkRpcServerState,
    ErrRpcServerState,
} from './state';

export function reducer(state = initialState, action: Action): State {
    switch (action.type) {
    case ActionType.Upsert:
        return onUpsert(state, action);
    case ActionType.Check:
        return onCheck(state, action);
    case ActionType.SetOk:
        return onSetOk(state, action);
    case ActionType.SetErr:
        return onSetErr(state, action);
    case ActionType.Remove:
        return onRemove(state, action);
    default:
        return state;
    }
}

function onUpsert(state: State, action: UpsertAction): State {
    const url = serverToUrl(action);
    if (url in state.rpcServers) {
        return state;
    } else {
        const rpcServer: UnknownRpcServerState = {
            type: RpcServerStateType.Default,
            protocol: action.protocol,
            host: action.host,
            port: action.port,
        };
        return {
            ...state,
            [url]: rpcServer,
        };
    }
}

function onCheck(state: State, action: CheckAction): State {
    const rpcServerUrl = serverToUrl(action);
    const rpcServer: CheckingRpcServerState = {
        ...(state[rpcServerUrl] || {}),
        type: RpcServerStateType.Checking,
        protocol: action.protocol,
        host: action.host,
        port: action.port,
    };
    return {
        ...state,
        [rpcServerUrl]: rpcServer,
    };
}

function onSetOk(state: State, action: SetOkAction): State {
    const rpcServerUrl = serverToUrl(action);
    const rpcServer: OkRpcServerState = {
        type: RpcServerStateType.Ok,
        protocol: action.protocol,
        host: action.host,
        port: action.port,
        ping: action.ping,
        chainId: action.chainId,
    };
    return {
        ...state,
        [rpcServerUrl]: rpcServer,
    };
}

function onSetErr(state: State, action: SetErrAction): State {
    const rpcServerUrl = serverToUrl(action);
    const rpcServer: ErrRpcServerState = {
        type: RpcServerStateType.Err,
        protocol: action.protocol,
        host: action.host,
        port: action.port,
        message: action.message,
    };
    return {
        ...state,
        [rpcServerUrl]: rpcServer,
    };
}

function onRemove(state: State, action: RemoveAction): State {
    const url = serverToUrl(action);
    if (url in state) {
        delete state[url];
        return { ...state };
    }
    return state;
}
