import { State } from '../root';
import { ExplorerState } from './state';

export const getAll = (state: State): ReadonlyArray<ExplorerState> =>
    Object.values(state.explorers);

export const getByChainId = (chainId: string) => (
    state: State,
): ReadonlyArray<ExplorerState> =>
    getAll(state).filter((e) => e.chainId === chainId);
