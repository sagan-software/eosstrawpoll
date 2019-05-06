import { all, put, select, takeLatest } from 'redux-saga/effects';
import * as Chains from './chains';
import * as RpcServers from './rpcServers';
import * as Scatter from './scatter';

export enum Type {
    Init = 'APP/INIT',
}

export type Action = Init;

export interface Init {
    readonly type: Type.Init;
}

export function* saga() {
    yield takeLatest(Type.Init, init);
}

function* init() {
    yield put<Scatter.Connect>({
        type: Scatter.Type.Connect,
        appName: 'weos.fund',
    });
    yield put<RpcServers.CheckAll>({
        type: RpcServers.Type.CheckAll,
    });
    yield* RpcServers.waitForAll();
    const chains: ReturnType<typeof Chains.getAllDefault> = yield select(
        Chains.getAllDefault,
    );
    yield all(
        chains.map((chain) =>
            put<Chains.SetErr>({
                type: Chains.Type.SetErr,
                chainId: chain.chainId,
                error: Chains.Err.NoRpcServer,
            }),
        ),
    );
}
