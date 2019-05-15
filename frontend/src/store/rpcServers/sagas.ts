import * as Eos from 'eosjs';
import {
    call,
    cancel,
    put,
    race,
    select,
    take,
    takeEvery,
    takeLatest,
} from 'redux-saga/effects';
import * as Chains from '../chains';
import {
    ActionType,
    SetOkAction,
    CheckAction,
    setOk,
    check,
    SetErrAction,
    setErr,
    CheckAllAction,
} from './actions';
import { getAll } from './selector';
import { serverToUrl, RpcServerState, RpcServerStateType } from './state';
import * as chains from '../chains';

export function* saga() {
    yield takeLatest(ActionType.CheckAll, onCheckAll);
    yield takeEvery(ActionType.Check, onCheck);
    yield takeEvery(ActionType.SetOk, onSetOk);
}

function* onCheck(action: CheckAction) {
    try {
        const url = serverToUrl(action);
        const rpc = new Eos.JsonRpc(url);
        const start = new Date();
        const info = yield call(rpc.get_info.bind(rpc));
        const end = new Date();
        const ping = end.getTime() - start.getTime();
        yield put(
            setOk(
                {
                    protocol: action.protocol,
                    host: action.host,
                    port: action.port,
                },
                ping,
                info.chain_id,
            ),
        );
    } catch (error) {
        yield put(
            setErr(
                {
                    protocol: action.protocol,
                    host: action.host,
                    port: action.port,
                },
                error.message,
            ),
        );
        yield cancel();
    }
}

export function* onCheckAll(action: CheckAllAction) {
    const servers: ReturnType<typeof getAll> = yield select(getAll);
    for (let i = servers.length; i--; ) {
        const server = servers[i];
        yield put(
            check({
                protocol: server.protocol,
                host: server.host,
                port: server.port,
            }),
        );
    }
}

function* onSetOk(action: SetOkAction) {
    const getChain = Chains.getById(action.chainId);
    const chain: ReturnType<typeof getChain> = yield select(getChain);
    if (!chain) {
        // TODO Add unknown chain?
    } else if (chain.type === chains.ChainStateType.Default) {
        yield put(
            chains.check(chain, {
                ...action,
                type: RpcServerStateType.Ok,
            }),
        );
    }
}

export function* waitForServers(servers: ReadonlyArray<RpcServerState>) {
    const pending = servers.filter(
        (s) =>
            s.type === RpcServerStateType.Default ||
            s.type === RpcServerStateType.Checking,
    );
    const ids = pending.map(serverToUrl);
    let count = pending.length;

    while (true) {
        if (count === 0) {
            return;
        }

        const [ok, err]: [SetOkAction | void, SetErrAction | void] = yield race(
            [take(ActionType.SetOk), take(ActionType.SetErr)],
        );
        if (ok) {
            const url = serverToUrl(ok);
            if (ids.includes(url)) {
                count--;
            }
        } else if (err) {
            const url = serverToUrl(err);
            if (ids.includes(url)) {
                count--;
            }
        }
    }
}

export function* waitForAll() {
    const chains: ReturnType<typeof getAll> = yield select(getAll);
    yield* waitForServers(chains);
}
