import * as State from './state';

export enum Type {
    Add = 'RPC_SERVERS/ADD',
    Check = 'RPC_SERVERS/CHECK',
    CheckAll = 'RPC_SERVERS/CHECK_ALL',
    SetOk = 'RPC_SERVERS/SET_OK',
    SetErr = 'RPC_SERVERS/SET_ERR',
    Remove = 'RPC_SERVERS/REMOVE',
}

export type Action = Add | Check | CheckAll | SetOk | SetErr | Remove;

export interface Add extends State.ServerBase {
    readonly type: Type.Add;
}

export interface Check extends State.ServerBase {
    readonly type: Type.Check;
}

export interface CheckAll {
    readonly type: Type.CheckAll;
}

export interface SetOk extends State.ServerOk {
    readonly type: Type.SetOk;
}

export interface SetErr extends State.ServerErr {
    readonly type: Type.SetErr;
}

export interface Remove extends State.ServerBase {
    readonly type: Type.Remove;
}
