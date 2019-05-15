import { OkRpcServerState } from '../rpcServers';
import { ChainEnv, ChainState, ChainError } from './state';

export enum ActionType {
    Upsert = 'CHAINS/UPSERT',
    Remove = 'CHAINS/REMOVE',
    CheckAll = 'CHAINS/CHECK_ALL',
    Check = 'CHAINS/CHECK',
    SetOk = 'CHAINS/SET_OK',
    SetErr = 'CHAINS/SET_ERR',
}

export type Action =
    | UpsertAction
    | RemoveAction
    | CheckAllAction
    | CheckAction
    | SetOkAction
    | SetErrAction;

export interface UpsertAction {
    readonly type: ActionType.Upsert;
    readonly chainId: string;
    readonly env: ChainEnv;
    readonly displayName: string;
    readonly contractName: string;
}

export function upsert(
    chainId: string,
    env: ChainEnv,
    displayName: string,
    contractName: string,
): UpsertAction {
    return {
        type: ActionType.Upsert,
        chainId,
        env,
        displayName,
        contractName,
    };
}

export interface RemoveAction {
    readonly type: ActionType.Remove;
    readonly chainId: string;
}

export function remove(chainId: string): RemoveAction {
    return {
        type: ActionType.Remove,
        chainId,
    };
}

export interface CheckAllAction {
    readonly type: ActionType.CheckAll;
}

export function checkAll(): CheckAllAction {
    return { type: ActionType.CheckAll };
}

export interface CheckAction {
    readonly type: ActionType.Check;
    readonly chain: ChainState;
    readonly server: OkRpcServerState;
}

export function check(
    chain: ChainState,
    server: OkRpcServerState,
): CheckAction {
    return {
        type: ActionType.Check,
        chain,
        server,
    };
}

export interface SetOkAction {
    readonly type: ActionType.SetOk;
    readonly chainId: string;
    readonly coreSymbol: string;
}

export function setOk(chainId: string, coreSymbol: string): SetOkAction {
    return {
        type: ActionType.SetOk,
        chainId,
        coreSymbol,
    };
}

export interface SetErrAction {
    readonly type: ActionType.SetErr;
    readonly chainId: string;
    readonly error: ChainError;
}

export function setErr(chainId: string, error: ChainError): SetErrAction {
    return {
        type: ActionType.SetErr,
        chainId,
        error,
    };
}
