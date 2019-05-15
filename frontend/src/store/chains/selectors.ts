import { State } from '../root';
import {
    OkChainState,
    ChainState,
    DefaultChainState,
    ErrChainState,
    ChainStateType,
} from './state';

export const getAll = (state: State): ReadonlyArray<ChainState> =>
    Object.values(state.chains);

export const getAllOk = (state: State): ReadonlyArray<OkChainState> =>
    getAll(state).reduce((acc: OkChainState[], chain) => {
        if (chain.type === ChainStateType.Ok) {
            acc.push(chain);
        }
        return acc;
    }, []);

export const getAllNotOk = (
    state: State,
): ReadonlyArray<DefaultChainState | ErrChainState> =>
    getAll(state).reduce(
        (acc: Array<DefaultChainState | ErrChainState>, chain) => {
            if (
                chain.type === ChainStateType.Default ||
                chain.type === ChainStateType.Err
            ) {
                acc.push(chain);
            }
            return acc;
        },
        [],
    );

export const getAllDefault = (state: State): ReadonlyArray<DefaultChainState> =>
    getAll(state).reduce((acc: DefaultChainState[], chain) => {
        if (chain.type === ChainStateType.Default) {
            acc.push(chain);
        }
        return acc;
    }, []);

export const getById = (chainId: string) => (state: State): ChainState | void =>
    state.chains[chainId];

export const getByIdPrefix = (chainIdPrefix: string) => (
    state: State,
): ChainState | void =>
    getAll(state).filter((c) => c.chainId.startsWith(chainIdPrefix))[0];
