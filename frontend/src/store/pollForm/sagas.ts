import { SubmitAction, ActionType, submitOk, submitErr } from './actions';
import { getState, getChain, getRpcServer } from './selectors';
import { takeLatest, call, select, put, cancel } from 'redux-saga/effects';
import * as chains from '../chains';
import * as rpcServers from '../rpcServers';
import * as scatter from '../scatter';

export function* sagas() {
    yield takeLatest(ActionType.Submit, onSubmit);
}

function* onSubmit(action: SubmitAction) {
    console.log('SUBMIT');
    const state: ReturnType<typeof getState> = yield select(getState);
    const chain: ReturnType<typeof getChain> = yield select(getChain);
    const rpcServer: ReturnType<typeof getRpcServer> = yield select(
        getRpcServer,
    );

    if (
        !chain ||
        chain.type !== chains.ChainStateType.Ok ||
        !rpcServer ||
        rpcServer.type !== rpcServers.RpcServerStateType.Ok
    ) {
        console.log(111, state, chain, rpcServer);
        // TODO
        return;
    }
    console.log(222);
    try {
        console.log(333);
        const result = yield* scatter.transact(rpcServer, (account) => {
            return [
                {
                    account: chain.contractName,
                    name: 'createpoll',
                    authorization: [
                        {
                            actor: account.name,
                            permission: account.authority,
                        },
                    ],
                    data: {
                        id: state.id,
                        account: account.name,
                        title: state.name,
                        options: state.options,
                        min_answers: state.minAnswers,
                        max_answers: state.maxAnswers,
                        min_writeins: state.minWriteins,
                        max_writeins: state.maxWriteins,
                        use_allow_list: state.useAllowList,
                        voter_list: state.voterList,
                        min_account_age: state.minVoterAge,
                        min_holdings: state.minVoterHoldings,
                        open_time: state.openTime || new Date(0),
                        close_time: state.closeTime || new Date(0),
                    },
                },
            ];
        });
        yield put(submitOk(result.transaction_id));
    } catch (error) {
        console.error(error);
        yield put(submitErr(error));
        yield cancel();
    }
}
