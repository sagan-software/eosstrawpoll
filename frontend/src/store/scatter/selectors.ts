import * as Root from '../root';
import * as State from './state';

export const getState = (state: Root.State) => state.scatter;

export const getStatus = (state: Root.State) => state.scatter.status;

export const isConnected = (state: Root.State) =>
    getStatus(state) === State.Status.Connected;

export const getIdentity = (state: Root.State): State.Identity =>
    state.scatter.status === State.Status.Connected
        ? state.scatter.identity
        : { status: State.IdentityStatus.LoggedOut };

export const isLoggedIn = (state: Root.State) => {
    const identity = getIdentity(state);
    return identity.status === State.IdentityStatus.LoggedIn;
};
