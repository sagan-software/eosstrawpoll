import * as Root from '../root';
import * as State from './state';

export const getAll = (state: Root.State): ReadonlyArray<State.Explorer> =>
    Object.values(state.explorers);

export const getByChainId = (chainId: string) => (
    state: Root.State,
): ReadonlyArray<State.Explorer> =>
    getAll(state).filter((e) => e.chainId === chainId);
