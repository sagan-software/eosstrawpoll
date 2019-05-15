import { State } from '../root';
import { StateType, IdentityState, IdentityStateType } from './state';

export const getState = (state: State) => state.scatter;

export const getStateType = (state: State) => state.scatter.type;

export const isConnected = (state: State) =>
    getStateType(state) === StateType.Connected;

export const getIdentity = (state: State): IdentityState =>
    state.scatter.type === StateType.Connected
        ? state.scatter.identity
        : { type: IdentityStateType.LoggedOut };

export const isLoggedIn = (state: State) => {
    const identity = getIdentity(state);
    return identity.type === IdentityStateType.LoggedIn;
};
