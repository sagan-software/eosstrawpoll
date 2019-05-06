import Scatter from 'scatterjs-core';

export enum Type {
    Connect = 'SCATTER/CONNECT',
    ConnectOk = 'SCATTER/CONNECT_OK',
    ConnectErr = 'SCATTER/CONNECT_ERR',
    Login = 'SCATTER/LOGIN',
    LoginOk = 'SCATTER/LOGIN_OK',
    LoginErr = 'SCATTER/LOGIN_ERR',
    Logout = 'SCATTER/LOGOUT',
    LogoutOk = 'SCATTER/LOGOUT_OK',
    SuggestNetwork = 'SCATTER/SUGGEST_NETWORK',
    SuggestNetworkOk = 'SCATTER/SUGGEST_NETWORK_OK',
    SuggestNetworkErr = 'SCATTER/SUGGEST_NETWORK_ERR',
}

export type Action =
    | Connect
    | ConnectOk
    | ConnectErr
    | Login
    | LoginOk
    | LoginErr
    | Logout
    | LogoutOk
    | SuggestNetwork
    | SuggestNetworkOk
    | SuggestNetworkErr;

export interface Connect {
    readonly type: Type.Connect;
    readonly appName: string;
}

export interface ConnectOk {
    readonly type: Type.ConnectOk;
    readonly appName: string;
    readonly identity: Scatter.Identity | void;
}

export interface ConnectErr {
    readonly type: Type.ConnectErr;
    readonly appName: string;
}

export interface Login {
    readonly type: Type.Login;
    readonly options: Scatter.LoginOptions;
}

export interface LoginOk {
    readonly type: Type.LoginOk;
    readonly identity: Scatter.Identity;
}

export interface LoginErr {
    readonly type: Type.LoginErr;
    readonly error: Scatter.LoginError;
}

export interface Logout {
    readonly type: Type.Logout;
}

export interface LogoutOk {
    readonly type: Type.LogoutOk;
}

export interface SuggestNetwork {
    readonly type: Type.SuggestNetwork;
}

export interface SuggestNetworkOk {
    readonly type: Type.SuggestNetworkOk;
}

export interface SuggestNetworkErr {
    readonly type: Type.SuggestNetworkErr;
    readonly error: Scatter.SuggestNetworkError;
}
