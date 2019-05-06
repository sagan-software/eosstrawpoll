import * as Root from '../root';
import * as State from './state';

export const getAll = (state: Root.State): ReadonlyArray<State.Chain> =>
    Object.values(state.chains);

export const getAllOk = (state: Root.State): ReadonlyArray<State.ChainOk> =>
    getAll(state).reduce((acc: State.ChainOk[], chain) => {
        if (chain.status === State.Status.Ok) {
            acc.push(chain);
        }
        return acc;
    }, []);

export const getAllNotOk = (
    state: Root.State,
): ReadonlyArray<State.ChainDefault | State.ChainErr> =>
    getAll(state).reduce(
        (acc: Array<State.ChainDefault | State.ChainErr>, chain) => {
            if (
                chain.status === State.Status.Default ||
                chain.status === State.Status.Err
            ) {
                acc.push(chain);
            }
            return acc;
        },
        [],
    );

export const getAllDefault = (
    state: Root.State,
): ReadonlyArray<State.ChainDefault> =>
    getAll(state).reduce((acc: State.ChainDefault[], chain) => {
        if (chain.status === State.Status.Default) {
            acc.push(chain);
        }
        return acc;
    }, []);

export const getById = (chainId: string) => (
    state: Root.State,
): State.Chain | void => state.chains[chainId];

export const getByIdPrefix = (chainIdPrefix: string) => (
    state: Root.State,
): State.Chain | void =>
    getAll(state).filter((c) => c.chainId.startsWith(chainIdPrefix))[0];
