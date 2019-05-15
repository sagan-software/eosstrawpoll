import Scatter from 'scatterjs-core';

export enum ActionType {
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
    | ConnectAction
    | ConnectOkAction
    | ConnectErrAction
    | LoginAction
    | LoginOkAction
    | LoginErrAction
    | LogoutAction
    | LogoutOkAction
    | SuggestNetworkAction
    | SuggestNetworkOkAction
    | SuggestNetworkErrAction;

export interface ConnectAction {
    readonly type: ActionType.Connect;
    readonly appName: string;
}

export function connect(appName: string): ConnectAction {
    return {
        type: ActionType.Connect,
        appName,
    };
}

export interface ConnectOkAction {
    readonly type: ActionType.ConnectOk;
    readonly appName: string;
    readonly identity: Scatter.Identity | void;
}

export function connectOk(
    appName: string,
    identity: Scatter.Identity | void,
): ConnectOkAction {
    return {
        type: ActionType.ConnectOk,
        appName,
        identity,
    };
}

export interface ConnectErrAction {
    readonly type: ActionType.ConnectErr;
    readonly appName: string;
}

export function connectErr(appName: string): ConnectErrAction {
    return {
        type: ActionType.ConnectErr,
        appName,
    };
}

export interface LoginAction {
    readonly type: ActionType.Login;
    readonly options: Scatter.LoginOptions;
}

export function login(options: Scatter.LoginOptions): LoginAction {
    return {
        type: ActionType.Login,
        options,
    };
}

export interface LoginOkAction {
    readonly type: ActionType.LoginOk;
    readonly identity: Scatter.Identity;
}

export function loginOk(identity: Scatter.Identity): LoginOkAction {
    return {
        type: ActionType.LoginOk,
        identity,
    };
}

export interface LoginErrAction {
    readonly type: ActionType.LoginErr;
    readonly error: Scatter.LoginError;
}

export function loginErr(error: Scatter.LoginError): LoginErrAction {
    return {
        type: ActionType.LoginErr,
        error,
    };
}

export interface LogoutAction {
    readonly type: ActionType.Logout;
}

export function logout(): LogoutAction {
    return {
        type: ActionType.Logout,
    };
}

export interface LogoutOkAction {
    readonly type: ActionType.LogoutOk;
}

export function logoutOk(): LogoutOkAction {
    return {
        type: ActionType.LogoutOk,
    };
}

export interface SuggestNetworkAction {
    readonly type: ActionType.SuggestNetwork;
}

export function suggestNetwork(): SuggestNetworkAction {
    return {
        type: ActionType.SuggestNetwork,
    };
}

export interface SuggestNetworkOkAction {
    readonly type: ActionType.SuggestNetworkOk;
}

export function suggestNetworkOk(): SuggestNetworkOkAction {
    return {
        type: ActionType.SuggestNetworkOk,
    };
}

export interface SuggestNetworkErrAction {
    readonly type: ActionType.SuggestNetworkErr;
    readonly error: Scatter.SuggestNetworkError;
}

export function suggestNetworkErr(
    error: Scatter.SuggestNetworkError,
): SuggestNetworkErrAction {
    return {
        type: ActionType.SuggestNetworkErr,
        error,
    };
}
