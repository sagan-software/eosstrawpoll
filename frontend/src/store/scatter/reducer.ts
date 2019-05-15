import {
    ActionType,
    Action,
    ConnectAction,
    ConnectOkAction,
    ConnectErrAction,
    LoginAction,
    LoginOkAction,
    LoginErrAction,
    LogoutAction,
    LogoutOkAction,
    SuggestNetworkAction,
    SuggestNetworkOkAction,
    SuggestNetworkErrAction,
} from './actions';
import { State, initialState, StateType, IdentityStateType } from './state';

export function reducer(state = initialState, action: Action): State {
    switch (action.type) {
    case ActionType.Connect:
        return onConnect(state, action);
    case ActionType.ConnectOk:
        return onConnectOk(state, action);
    case ActionType.ConnectErr:
        return onConnectErr(state, action);
    case ActionType.Login:
        return onLogin(state, action);
    case ActionType.LoginOk:
        return onLoginOk(state, action);
    case ActionType.LoginErr:
        return onLoginErr(state, action);
    case ActionType.Logout:
        return onLogout(state, action);
    case ActionType.LogoutOk:
        return onLogoutOk(state, action);
    case ActionType.SuggestNetwork:
        return onSuggestNetwork(state, action);
    case ActionType.SuggestNetworkOk:
        return onSuggestNetworkOk(state, action);
    case ActionType.SuggestNetworkErr:
        return onSuggestNetworkErr(state, action);
    default:
        return state;
    }
}

function onConnect(state: State, action: ConnectAction): State {
    return {
        type: StateType.Connecting,
        appName: action.appName,
    };
}

function onConnectOk(state: State, action: ConnectOkAction): State {
    return {
        type: StateType.Connected,
        appName: action.appName,
        identity: action.identity
            ? { type: IdentityStateType.LoggedIn, ...action.identity }
            : { type: IdentityStateType.LoggedOut },
    };
}

function onConnectErr(state: State, action: ConnectErrAction): State {
    return {
        type: StateType.Unavailable,
        appName: action.appName,
    };
}

function onLogin(state: State, action: LoginAction): State {
    if (state.type === StateType.Connected) {
        return {
            ...state,
            identity: {
                type: IdentityStateType.LoggingIn,
                options: action.options,
            },
        };
    } else {
        return state;
    }
}

function onLoginOk(state: State, action: LoginOkAction): State {
    if (state.type === StateType.Connected) {
        return {
            ...state,
            identity: {
                type: IdentityStateType.LoggedIn,
                ...action.identity,
            },
        };
    } else {
        return state;
    }
}

function onLoginErr(state: State, action: LoginErrAction): State {
    if (state.type === StateType.Connected) {
        return {
            ...state,
            identity: {
                type: IdentityStateType.LoginError,
                error: action.error,
            },
        };
    } else {
        return state;
    }
}

function onLogout(state: State, action: LogoutAction): State {
    if (state.type === StateType.Connected) {
        return {
            ...state,
        };
    } else {
        return state;
    }
}

function onLogoutOk(state: State, action: LogoutOkAction): State {
    if (state.type === StateType.Connected) {
        return {
            ...state,
            identity: {
                type: IdentityStateType.LoggedOut,
            },
        };
    } else {
        return state;
    }
}

function onSuggestNetwork(state: State, action: SuggestNetworkAction): State {
    // TODO
    return state;
}

function onSuggestNetworkOk(
    state: State,
    action: SuggestNetworkOkAction,
): State {
    // TODO
    return state;
}

function onSuggestNetworkErr(
    state: State,
    action: SuggestNetworkErrAction,
): State {
    // TODO
    return state;
}
