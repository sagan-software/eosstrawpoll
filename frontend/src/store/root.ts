import { Action, combineReducers } from 'redux';
import { spawn } from 'redux-saga/effects';
import * as App from './app';
import * as Chains from './chains';
import * as Explorers from './explorers';
import * as RpcServers from './rpcServers';
import * as Scatter from './scatter';

export const reducer = combineReducers({
    chains: Chains.reducer,
    explorers: Explorers.reducer,
    rpcServers: RpcServers.reducer,
    scatter: Scatter.reducer,
});

export type State = ReturnType<typeof reducer>;

export type Action =
    | App.Action
    | Chains.Action
    | Explorers.Action
    | RpcServers.Action
    | Scatter.Action;

export function* saga() {
    yield spawn(App.saga);
    yield spawn(Chains.saga);
    yield spawn(RpcServers.saga);
    yield spawn(Scatter.saga);
}
