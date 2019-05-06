import * as Eos from 'eosjs';
import {
    all,
    call,
    cancel,
    put,
    race,
    select,
    spawn,
    take,
    takeEvery,
} from 'redux-saga/effects';
import * as Contract from '../../contract';
import * as RpcServers from '../rpcServers';
import * as Action from './action';
import { getAll, getById } from './selectors';
import * as State from './state';

export function* saga() {
    yield takeEvery(Action.Type.Check, onCheck);
}

function* onCheck({ chain, server }: Action.Check) {
    const chainId = chain.chainId;
    const url = RpcServers.serverToUrl(server);
    const rpc = new Eos.JsonRpc(url);
    let coreSymbol: string;
    try {
        const systemAccount: { core_liquid_balance: string } = yield call(
            rpc.get_account.bind(rpc),
            'eosio',
        );
        const coreLiquidBalance = systemAccount.core_liquid_balance;
        coreSymbol = coreLiquidBalance.split(' ')[1];
    } catch (e) {
        yield put<Action.SetErr>({
            type: Action.Type.SetErr,
            chainId,
            error: State.Err.NoCoreSymbol,
        });
        return yield cancel();
    }

    let contract: Eos.RpcInterfaces.GetAbiResult;
    try {
        contract = yield call(rpc.get_abi.bind(rpc), chain.contractName);
    } catch (e) {
        yield put<Action.SetErr>({
            type: Action.Type.SetErr,
            chainId: chain.chainId,
            error: State.Err.NoContractAccount,
        });
        return yield cancel();
    }

    if (contract.abi && Contract.isValidAbi(contract.abi)) {
        yield put<Action.SetOk>({
            type: Action.Type.SetOk,
            chainId: chain.chainId,
            coreSymbol,
        });
    } else {
        yield put<Action.SetErr>({
            type: Action.Type.SetErr,
            chainId: chain.chainId,
            error: State.Err.InvalidContractAbi,
        });
    }
}

function* checkAll(action: Action.CheckAll) {
    const chains = yield select(getAll);
}

export function* waitForChainId(chainId: string) {
    const getChain = getById(chainId);
    const chain: ReturnType<typeof getChain> = yield select(getChain);
    if (!chain) {
        console.error(
            'Attempted to load a project for an unknown chain ID',
            chainId,
        );
        return yield cancel();
    }
    yield* waitForChains([chain]);
}

export function* waitForChains(chains: ReadonlyArray<State.Chain>) {
    const pendingChains = chains.filter(
        (c) =>
            c.status === State.Status.Default ||
            c.status === State.Status.Checking,
    );
    const chainIds = pendingChains.map((c) => c.chainId);

    let count = chainIds.length;

    while (true) {
        if (count === 0) {
            return;
        }

        const [ok, err]: [
            Action.SetOk | void,
            Action.SetErr | void
        ] = yield race([take(Action.Type.SetOk), take(Action.Type.SetErr)]);
        if (ok && chainIds.includes(ok.chainId)) {
            count--;
        } else if (err && chainIds.includes(err.chainId)) {
            count--;
        }
    }
}

export function* waitForAll() {
    const chains: ReturnType<typeof getAll> = yield select(getAll);
    yield* waitForChains(chains);
}
