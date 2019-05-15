import { Action, combineReducers } from 'redux';
import { spawn } from 'redux-saga/effects';
import * as app from './app';
import * as chains from './chains';
import * as explorers from './explorers';
import * as pollForm from './pollForm';
import * as rpcServers from './rpcServers';
import * as scatter from './scatter';

export const reducer = combineReducers({
    chains: chains.reducer,
    explorers: explorers.reducer,
    pollForm: pollForm.reducer,
    rpcServers: rpcServers.reducer,
    scatter: scatter.reducer,
});

export type State = ReturnType<typeof reducer>;

export type Action =
    | app.Action
    | chains.Action
    | pollForm.Action
    | explorers.Action
    | rpcServers.Action
    | scatter.Action;

export function* saga() {
    yield spawn(app.saga);
    yield spawn(chains.saga);
    yield spawn(pollForm.sagas);
    yield spawn(rpcServers.saga);
    yield spawn(scatter.saga);
}
