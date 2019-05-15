import {
    BaseRpcServerState,
    OkRpcServerState,
    ErrRpcServerState,
} from './state';

export enum ActionType {
    Upsert = 'RPC_SERVERS/UPSERT',
    Check = 'RPC_SERVERS/CHECK',
    CheckAll = 'RPC_SERVERS/CHECK_ALL',
    SetOk = 'RPC_SERVERS/SET_OK',
    SetErr = 'RPC_SERVERS/SET_ERR',
    Remove = 'RPC_SERVERS/REMOVE',
}

export type Action =
    | UpsertAction
    | CheckAction
    | CheckAllAction
    | SetOkAction
    | SetErrAction
    | RemoveAction;

export interface UpsertAction extends BaseRpcServerState {
    readonly type: ActionType.Upsert;
}

export function upsert(base: BaseRpcServerState): UpsertAction {
    return {
        ...base,
        type: ActionType.Upsert,
    };
}

export interface CheckAction extends BaseRpcServerState {
    readonly type: ActionType.Check;
}

export function check(base: BaseRpcServerState): CheckAction {
    return {
        ...base,
        type: ActionType.Check,
    };
}

export interface CheckAllAction {
    readonly type: ActionType.CheckAll;
}

export function checkAll(): CheckAllAction {
    return {
        type: ActionType.CheckAll,
    };
}

export interface SetOkAction extends BaseRpcServerState {
    readonly type: ActionType.SetOk;
    readonly ping: number;
    readonly chainId: string;
}

export function setOk(
    base: BaseRpcServerState,
    ping: number,
    chainId: string,
): SetOkAction {
    return {
        ...base,
        type: ActionType.SetOk,
        ping,
        chainId,
    };
}

export interface SetErrAction extends BaseRpcServerState {
    readonly type: ActionType.SetErr;
    readonly message: string;
}

export function setErr(
    base: BaseRpcServerState,
    message: string,
): SetErrAction {
    return {
        ...base,
        type: ActionType.SetErr,
        message,
    };
}

export interface RemoveAction extends BaseRpcServerState {
    readonly type: ActionType.Remove;
}

export function remove(base: BaseRpcServerState): RemoveAction {
    return {
        ...base,
        type: ActionType.Remove,
    };
}
