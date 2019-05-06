import * as RpcServers from '../rpcServers';
import * as State from './state';

export enum Type {
    Upsert = 'CHAINS/UPSERT',
    Remove = 'CHAINS/REMOVE',
    CheckAll = 'CHAINS/CHECK_ALL',
    Check = 'CHAINS/CHECK',
    SetOk = 'CHAINS/SET_OK',
    SetErr = 'CHAINS/SET_ERR',
}

export type Action = Upsert | Remove | CheckAll | Check | SetOk | SetErr;

export interface Upsert {
    readonly type: Type.Upsert;
    readonly chainId: string;
    readonly env: State.Env;
    readonly displayName: string;
    readonly contractName: string;
    readonly priority: State.Priority;
}

export interface Remove {
    readonly type: Type.Remove;
    readonly chainId: string;
}

export interface CheckAll {
    readonly type: Type.CheckAll;
}

export interface Check {
    readonly type: Type.Check;
    readonly chain: State.Chain;
    readonly server: RpcServers.ServerOk;
}

export interface SetOk {
    readonly type: Type.SetOk;
    readonly chainId: string;
    readonly coreSymbol: string;
}

export interface SetErr {
    readonly type: Type.SetErr;
    readonly chainId: string;
    readonly error: State.Err;
}
