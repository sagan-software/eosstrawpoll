import Scatter from 'scatterjs-core';

export type FullNetwork = Scatter.FullNetwork;
export type PartialNetwork = Scatter.PartialNetwork;

export type State = Default | Connecting | Connected | Unavailable;

export enum Status {
    Default,
    Connecting,
    Connected,
    Unavailable,
}

export interface Default {
    readonly status: Status.Default;
}

export interface Connecting {
    readonly status: Status.Connecting;
    readonly appName: string;
}

export interface Connected {
    readonly status: Status.Connected;
    readonly appName: string;
    readonly identity: Identity;
}

export type Identity =
    | LoggedOut
    | LoggedIn
    | LoggingIn
    | LoggingOut
    | LoginError;

export enum IdentityStatus {
    LoggedOut,
    LoggedIn,
    LoggingIn,
    LoggingOut,
    LoginError,
}

export interface LoggedOut {
    readonly status: IdentityStatus.LoggedOut;
}

export interface LoggedIn extends Scatter.Identity {
    readonly status: IdentityStatus.LoggedIn;
}

export interface LoggingIn {
    readonly status: IdentityStatus.LoggingIn;
    readonly options: Scatter.LoginOptions;
}

export interface LoggingOut extends Scatter.Identity {
    readonly status: IdentityStatus.LoggingOut;
}

export interface LoginError {
    readonly status: IdentityStatus.LoginError;
    readonly error: Scatter.LoginError;
}

export interface Unavailable {
    readonly status: Status.Unavailable;
    readonly appName: string;
}

export const initialState: State = {
    status: Status.Default,
};
