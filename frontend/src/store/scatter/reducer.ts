import * as Action from './action';
import * as State from './state';

export function reducer(
    state = State.initialState,
    action: Action.Action,
): State.State {
    switch (action.type) {
    case Action.Type.Connect:
        return onConnect(state, action);
    case Action.Type.ConnectOk:
        return onConnectOk(state, action);
    case Action.Type.ConnectErr:
        return onConnectErr(state, action);
    case Action.Type.Login:
        return onLogin(state, action);
    case Action.Type.LoginOk:
        return onLoginOk(state, action);
    case Action.Type.LoginErr:
        return onLoginErr(state, action);
    case Action.Type.Logout:
        return onLogout(state, action);
    case Action.Type.LogoutOk:
        return onLogoutOk(state, action);
    case Action.Type.SuggestNetwork:
        return onSuggestNetwork(state, action);
    case Action.Type.SuggestNetworkOk:
        return onSuggestNetworkOk(state, action);
    case Action.Type.SuggestNetworkErr:
        return onSuggestNetworkErr(state, action);
    default:
        return state;
    }
}

function onConnect(state: State.State, action: Action.Connect): State.State {
    return {
        status: State.Status.Connecting,
        appName: action.appName,
    };
}

function onConnectOk(
    state: State.State,
    action: Action.ConnectOk,
): State.State {
    return {
        status: State.Status.Connected,
        appName: action.appName,
        identity: action.identity
            ? { status: State.IdentityStatus.LoggedIn, ...action.identity }
            : { status: State.IdentityStatus.LoggedOut },
    };
}

function onConnectErr(
    state: State.State,
    action: Action.ConnectErr,
): State.State {
    return {
        status: State.Status.Unavailable,
        appName: action.appName,
    };
}

function onLogin(state: State.State, action: Action.Login): State.State {
    if (state.status === State.Status.Connected) {
        return {
            ...state,
            identity: {
                status: State.IdentityStatus.LoggingIn,
                options: action.options,
            },
        };
    } else {
        return state;
    }
}

function onLoginOk(state: State.State, action: Action.LoginOk): State.State {
    if (state.status === State.Status.Connected) {
        return {
            ...state,
            identity: {
                status: State.IdentityStatus.LoggedIn,
                ...action.identity,
            },
        };
    } else {
        return state;
    }
}

function onLoginErr(state: State.State, action: Action.LoginErr): State.State {
    if (state.status === State.Status.Connected) {
        return {
            ...state,
            identity: {
                status: State.IdentityStatus.LoginError,
                error: action.error,
            },
        };
    } else {
        return state;
    }
}

function onLogout(state: State.State, action: Action.Logout): State.State {
    if (state.status === State.Status.Connected) {
        return {
            ...state,
        };
    } else {
        return state;
    }
}

function onLogoutOk(state: State.State, action: Action.LogoutOk): State.State {
    if (state.status === State.Status.Connected) {
        return {
            ...state,
            identity: {
                status: State.IdentityStatus.LoggedOut,
            },
        };
    } else {
        return state;
    }
}

function onSuggestNetwork(
    state: State.State,
    action: Action.SuggestNetwork,
): State.State {
    // TODO
    return state;
}

function onSuggestNetworkOk(
    state: State.State,
    action: Action.SuggestNetworkOk,
): State.State {
    // TODO
    return state;
}

function onSuggestNetworkErr(
    state: State.State,
    action: Action.SuggestNetworkErr,
): State.State {
    // TODO
    return state;
}
