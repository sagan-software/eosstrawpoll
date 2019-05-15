import Scatter from 'scatterjs-core';

export enum StateType {
    Default = 'DEFAULT',
    Connecting = 'CONNECTING',
    Connected = 'CONNECTED',
    Unavailable = 'UNAVAILABLE',
}

export type State =
    | DefaultState
    | ConnectingState
    | ConnectedState
    | UnavailableState;

export interface DefaultState {
    readonly type: StateType.Default;
}

export interface ConnectingState {
    readonly type: StateType.Connecting;
    readonly appName: string;
}

export interface ConnectedState {
    readonly type: StateType.Connected;
    readonly appName: string;
    readonly identity: IdentityState;
}

export interface UnavailableState {
    readonly type: StateType.Unavailable;
    readonly appName: string;
}

export const initialState: State = {
    type: StateType.Default,
};

export type IdentityState =
    | LoggedOutState
    | LoggedInState
    | LoggingInState
    | LoggingOutState
    | LoginErrorState;

export enum IdentityStateType {
    LoggedOut,
    LoggedIn,
    LoggingIn,
    LoggingOut,
    LoginError,
}

export interface LoggedOutState {
    readonly type: IdentityStateType.LoggedOut;
}

export interface LoggedInState extends Scatter.Identity {
    readonly type: IdentityStateType.LoggedIn;
}

export interface LoggingInState {
    readonly type: IdentityStateType.LoggingIn;
    readonly options: Scatter.LoginOptions;
}

export interface LoggingOutState extends Scatter.Identity {
    readonly type: IdentityStateType.LoggingOut;
}

export interface LoginErrorState {
    readonly type: IdentityStateType.LoginError;
    readonly error: Scatter.LoginError;
}

export type FullNetwork = Scatter.FullNetwork;
export type PartialNetwork = Scatter.PartialNetwork;
