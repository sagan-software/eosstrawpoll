import { createSelector } from 'reselect';
import * as Root from '../root';
import * as State from './state';

export const getAll = (rootState: Root.State): ReadonlyArray<State.Server> =>
    Object.values(rootState.rpcServers);

export const getByStatus = (status: State.Status) => (
    state: Root.State,
): ReadonlyArray<State.Server> =>
    getAll(state).filter((s) => s.status === status);

export const filterByStatus = (
    rpcServers: State.Server[],
    status: State.Status,
) => rpcServers.filter((rpcServer) => rpcServer.status === status);

export const filterByChainIdPrefix = (
    rpcServers: State.Server[],
    chainIdPrefix: string,
) =>
    rpcServers.filter(
        (rpcServer) =>
            rpcServer.status === State.Status.Ok &&
            rpcServer.chainId.startsWith(chainIdPrefix),
    );

export const sortByPing = (rpcServers: State.Server[]) =>
    rpcServers.sort((a, b) => {
        if (a.status === State.Status.Ok) {
            if (b.status === State.Status.Ok) {
                return a.ping - b.ping;
            } else {
                return -1;
            }
        } else {
            if (b.status === State.Status.Ok) {
                return 1;
            } else {
                return 0;
            }
        }
    });

export const getBest = (
    state: Root.State,
): { [chainId: string]: State.ServerOk } =>
    getAll(state).reduce(
        (acc: { [chainId: string]: State.ServerOk }, server) => {
            if (server.status === State.Status.Ok) {
                const chainId = server.chainId;
                const existing = acc[chainId];
                if (!existing || (existing && existing.ping > server.ping)) {
                    acc[chainId] = server;
                }
            }
            return acc;
        },
        {},
    );

export const getBestByChainId = (chainId: string) => (
    state: Root.State,
): State.ServerOk | void => getBest(state)[chainId];

export const getBestByChainIdPrefix = (chainIdPrefix: string) => (
    state: Root.State,
): State.ServerOk | void =>
    Object.values(getBest(state)).filter((s) =>
        s.chainId.startsWith(chainIdPrefix),
    )[0];

// export const getBestRpcServer = (chainIdPrefix: string) =>
//     createSelector(
//         getRpcServers,
//         (rpcServers) =>
//             filterRpcServersByChainIdPrefix(rpcServers, chainIdPrefix),
//         sortRpcServersByPing,
//         (rpcServers) => rpcServers[0],
//     );
