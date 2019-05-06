import * as Eos from 'eosjs';
import {
    call,
    cancel,
    put,
    race,
    select,
    take,
    takeLatest,
} from 'redux-saga/effects';
import Scatter from 'scatterjs-core';
import * as RpcServers from '../rpcServers';
import * as Action from './action';
import { getIdentity, isConnected } from './selectors';
import * as State from './state';

export function* saga() {
    yield takeLatest(Action.Type.Connect, onConnect);
    yield takeLatest(Action.Type.Login, onLogin);
    yield takeLatest(Action.Type.Logout, onLogout);
}

function* onConnect({ appName }: Action.Connect) {
    try {
        const connected = yield call(Scatter.connect.bind(Scatter), appName);
        if (connected) {
            yield put<Action.ConnectOk>({
                type: Action.Type.ConnectOk,
                appName,
                identity: Scatter.identity,
            });
        } else {
            yield put<Action.ConnectErr>({
                type: Action.Type.ConnectErr,
                appName,
            });
            yield cancel();
        }
    } catch (e) {
        console.error(e);
        yield put<Action.ConnectErr>({
            type: Action.Type.ConnectErr,
            appName,
        });
        yield cancel();
    }
}

function* onLogin({ options }: Action.Login) {
    try {
        const identity: Scatter.Identity = yield call(
            Scatter.login.bind(Scatter),
            options,
        );
        yield put<Action.LoginOk>({
            type: Action.Type.LoginOk,
            identity,
        });
        return identity;
    } catch (error) {
        console.error(error);
        yield put<Action.LoginErr>({
            type: Action.Type.LoginErr,
            error,
        });
        yield cancel();
    }
}

function* onLogout(action: Action.Logout) {
    try {
        yield call(Scatter.logout.bind(Scatter));
        yield put<Action.LogoutOk>({
            type: Action.Type.LogoutOk,
        });
    } catch (e) {
        // TODO can this error?
        console.error(e);
        yield cancel();
    }
}

export function* transact<T>(
    rpcServer: RpcServers.ServerOk,
    toActions: (account: Scatter.Account) => ReadonlyArray<T>,
) {
    const connected: ReturnType<typeof isConnected> = yield select(isConnected);
    if (!connected) {
        yield put<Action.Connect>({
            type: Action.Type.Connect,
            appName: 'weos.fund',
        });
        const [_, err] = yield race([
            take(Action.Type.ConnectOk),
            take(Action.Type.ConnectErr),
        ]);
        if (err) {
            return yield cancel();
        }
    }

    const chainId = rpcServer.chainId;
    const identity: ReturnType<typeof getIdentity> = yield select(getIdentity);
    let needsLogin = false;
    if (identity && identity.status === State.IdentityStatus.LoggedIn) {
        const hasAccount = identity.accounts.some((a) => a.chainId === chainId);
        if (!hasAccount) {
            yield put<Action.Logout>({ type: Action.Type.Logout });
            yield take(Action.Type.LogoutOk); // TODO LogoutErr ?
            needsLogin = true;
        }
    } else {
        needsLogin = true;
    }

    if (needsLogin) {
        yield put<Action.Login>({
            type: Action.Type.Login,
            options: {
                accounts: [
                    {
                        name: chainId,
                        protocol: rpcServer.protocol,
                        host: rpcServer.host,
                        port: rpcServer.port,
                        chainId,
                        blockchain: 'eos',
                    },
                ],
            },
        });
        const [_, err] = yield race([
            take(Action.Type.LoginOk),
            take(Action.Type.LoginErr),
        ]);
        if (err) {
            return yield cancel();
        }
    }

    const id: ReturnType<typeof getIdentity> = yield select(getIdentity);
    if (id && id.status === State.IdentityStatus.LoggedIn) {
        const account = id.accounts.filter((a) => a.chainId === chainId)[0];
        const rpcServerUrl = RpcServers.serverToUrl(rpcServer);
        const rpc = new Eos.JsonRpc(rpcServerUrl);
        const eos: Eos.Api = Scatter.eos({ chainId }, Eos.Api, {
            rpc,
            beta3: true,
        });
        const result = yield call(
            eos.transact.bind(eos),
            {
                actions: toActions(account),
            },
            {
                blocksBehind: 3,
                expireSeconds: 30,
            },
        );
        return {
            ...result,
            account,
        };
    } else {
        // TODO
    }
}
