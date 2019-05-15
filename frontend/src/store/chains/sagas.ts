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
import { serverToUrl } from '../rpcServers';
import {
    CheckAction,
    CheckAllAction,
    setErr,
    setOk,
    ActionType,
    SetErrAction,
    SetOkAction,
} from './actions';
import { getAll, getById } from './selectors';
import { ChainError, ChainStateType, ChainState } from './state';

export function* saga() {
    yield takeEvery(ActionType.Check, onCheck);
}

function* onCheck({ chain, server }: CheckAction) {
    const chainId = chain.chainId;
    const url = serverToUrl(server);
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
        yield put(setErr(chainId, ChainError.NoCoreSymbol));
        return yield cancel();
    }

    let contract: Eos.RpcInterfaces.GetAbiResult;
    try {
        contract = yield call(rpc.get_abi.bind(rpc), chain.contractName);
    } catch (e) {
        yield put(setErr(chain.chainId, ChainError.NoContractAccount));
        return yield cancel();
    }

    if (contract.abi && Contract.isValidAbi(contract.abi)) {
        yield put(setOk(chain.chainId, coreSymbol));
    } else {
        yield put(setErr(chain.chainId, ChainError.InvalidContractAbi));
    }
}

function* checkAll(action: CheckAllAction) {
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

export function* waitForChains(chains: ReadonlyArray<ChainState>) {
    const pendingChains = chains.filter(
        (c) =>
            c.type === ChainStateType.Default ||
            c.type === ChainStateType.Checking,
    );
    const chainIds = pendingChains.map((c) => c.chainId);

    let count = chainIds.length;

    while (true) {
        if (count === 0) {
            return;
        }

        const [ok, err]: [SetOkAction | void, SetErrAction | void] = yield race(
            [take(ActionType.SetOk), take(ActionType.SetErr)],
        );
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
