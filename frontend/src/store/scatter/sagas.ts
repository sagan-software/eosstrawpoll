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
import * as rpcServers from '../rpcServers';
import {
    ActionType,
    connectOk,
    connectErr,
    connect,
    ConnectAction,
    LoginAction,
    login,
    loginOk,
    loginErr,
    logout,
    logoutOk,
    LogoutAction,
} from './actions';
import { getIdentity, isConnected } from './selectors';
import { IdentityStateType } from './state';

export function* saga() {
    yield takeLatest(ActionType.Connect, onConnect);
    yield takeLatest(ActionType.Login, onLogin);
    yield takeLatest(ActionType.Logout, onLogout);
}

function* onConnect({ appName }: ConnectAction) {
    try {
        const connected = yield call(Scatter.connect.bind(Scatter), appName);
        if (connected) {
            yield put(connectOk(appName, Scatter.identity));
        } else {
            yield put(connectErr(appName));
            yield cancel();
        }
    } catch (e) {
        console.error(e);
        yield put(connectErr(appName));
        yield cancel();
    }
}

function* onLogin({ options }: LoginAction) {
    try {
        const identity: Scatter.Identity = yield call(
            Scatter.login.bind(Scatter),
            options,
        );
        yield put(loginOk(identity));
        return identity;
    } catch (error) {
        console.error(error);
        yield put(loginErr(error));
        yield cancel();
    }
}

function* onLogout(action: LogoutAction) {
    try {
        yield call(Scatter.logout.bind(Scatter));
        yield put(logoutOk());
    } catch (e) {
        // TODO can this error?
        console.error(e);
        yield cancel();
    }
}

export function* transact<T>(
    rpcServer: rpcServers.OkRpcServerState,
    toActions: (account: Scatter.Account) => ReadonlyArray<T>,
) {
    const connected: ReturnType<typeof isConnected> = yield select(isConnected);
    if (!connected) {
        yield put(connect('eosstrawpoll'));
        const [_, err] = yield race([
            take(ActionType.ConnectOk),
            take(ActionType.ConnectErr),
        ]);
        if (err) {
            return yield cancel();
        }
    }

    const chainId = rpcServer.chainId;
    const identity: ReturnType<typeof getIdentity> = yield select(getIdentity);
    let needsLogin = false;
    if (identity && identity.type === IdentityStateType.LoggedIn) {
        const hasAccount = identity.accounts.some((a) => a.chainId === chainId);
        if (!hasAccount) {
            yield put(logout());
            yield take(ActionType.LogoutOk); // TODO LogoutErr ?
            needsLogin = true;
        }
    } else {
        needsLogin = true;
    }

    if (needsLogin) {
        yield put(
            login({
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
            }),
        );
        const [_, err] = yield race([
            take(ActionType.LoginOk),
            take(ActionType.LoginErr),
        ]);
        if (err) {
            return yield cancel();
        }
    }

    const id: ReturnType<typeof getIdentity> = yield select(getIdentity);
    if (id && id.type === IdentityStateType.LoggedIn) {
        const account = id.accounts.filter((a) => a.chainId === chainId)[0];
        const rpcServerUrl = rpcServers.serverToUrl(rpcServer);
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
