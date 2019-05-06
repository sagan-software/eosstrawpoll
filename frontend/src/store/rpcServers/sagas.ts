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
import * as Action from './action';
import { getAll } from './selector';
import * as State from './state';

export function* saga() {
    yield takeLatest(Action.Type.CheckAll, onCheckAll);
    yield takeEvery(Action.Type.Check, onCheck);
    yield takeEvery(Action.Type.SetOk, onSetOk);
}

function* onCheck(action: Action.Check) {
    try {
        const url = State.serverToUrl(action);
        const rpc = new Eos.JsonRpc(url);
        const start = new Date();
        const info = yield call(rpc.get_info.bind(rpc));
        const end = new Date();
        const ping = end.getTime() - start.getTime();
        yield put<Action.SetOk>({
            type: Action.Type.SetOk,
            protocol: action.protocol,
            host: action.host,
            port: action.port,
            status: State.Status.Ok,
            ping,
            chainId: info.chain_id,
        });
    } catch (error) {
        yield put<Action.SetErr>({
            type: Action.Type.SetErr,
            protocol: action.protocol,
            host: action.host,
            port: action.port,
            status: State.Status.Err,
            message: error.message,
        });
        yield cancel();
    }
}

export function* onCheckAll(action: Action.CheckAll) {
    const servers: ReturnType<typeof getAll> = yield select(getAll);
    for (let i = servers.length; i--; ) {
        const server = servers[i];
        yield put<Action.Check>({
            type: Action.Type.Check,
            protocol: server.protocol,
            host: server.host,
            port: server.port,
        });
    }
}

function* onSetOk(action: Action.SetOk) {
    const getChain = Chains.getById(action.chainId);
    const chain: ReturnType<typeof getChain> = yield select(getChain);
    if (!chain) {
        // Add unknown chain?
    } else if (chain.status === Chains.Status.Default) {
        yield put<Chains.Check>({
            type: Chains.Type.Check,
            chain,
            server: action,
        });
    }
}

export function* waitForServers(servers: ReadonlyArray<State.Server>) {
    const pending = servers.filter(
        (s) =>
            s.status === State.Status.Default ||
            s.status === State.Status.Checking,
    );
    const ids = pending.map(State.serverToUrl);
    let count = pending.length;

    while (true) {
        if (count === 0) {
            return;
        }

        const [ok, err]: [
            Action.SetOk | void,
            Action.SetErr | void
        ] = yield race([take(Action.Type.SetOk), take(Action.Type.SetErr)]);
        if (ok) {
            const url = State.serverToUrl(ok);
            if (ids.includes(url)) {
                count--;
            }
        } else if (err) {
            const url = State.serverToUrl(err);
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
