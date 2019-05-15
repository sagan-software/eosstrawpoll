import { all, put, select, takeLatest } from 'redux-saga/effects';
import * as chains from './chains';
import * as rpcServers from './rpcServers';
import * as scatter from './scatter';

export enum ActionType {
    Init = 'APP/INIT',
}

export type Action = InitAction;

export interface InitAction {
    readonly type: ActionType.Init;
}

export function init(): InitAction {
    return {
        type: ActionType.Init,
    };
}

export function* saga() {
    yield takeLatest(ActionType.Init, onInit);
}

function* onInit() {
    yield put(scatter.connect('eosstrawpoll'));
    yield put(rpcServers.checkAll());
    yield* rpcServers.waitForAll();
    const allChains: ReturnType<typeof chains.getAllDefault> = yield select(
        chains.getAllDefault,
    );
    yield all(
        allChains.map((chain) =>
            put(chains.setErr(chain.chainId, chains.ChainError.NoRpcServer)),
        ),
    );
}
