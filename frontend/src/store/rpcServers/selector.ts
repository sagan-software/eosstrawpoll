import { State } from '../root';
import { RpcServerStateType, RpcServerState, OkRpcServerState } from './state';

export const getAll = (state: State): ReadonlyArray<RpcServerState> =>
    Object.values(state.rpcServers);

export const getByStateType = (type: RpcServerStateType) => (
    state: State,
): ReadonlyArray<RpcServerState> => getAll(state).filter((s) => s.type === type);

export const filterByStatus = (
    rpcServers: RpcServerState[],
    type: RpcServerStateType,
) => rpcServers.filter((rpcServer) => rpcServer.type === type);

export const filterByChainIdPrefix = (
    rpcServers: RpcServerState[],
    chainIdPrefix: string,
) =>
    rpcServers.filter(
        (rpcServer) =>
            rpcServer.type === RpcServerStateType.Ok &&
            rpcServer.chainId.startsWith(chainIdPrefix),
    );

export const sortByPing = (rpcServers: RpcServerState[]) =>
    rpcServers.sort((a, b) => {
        if (a.type === RpcServerStateType.Ok) {
            if (b.type === RpcServerStateType.Ok) {
                return a.ping - b.ping;
            } else {
                return -1;
            }
        } else {
            if (b.type === RpcServerStateType.Ok) {
                return 1;
            } else {
                return 0;
            }
        }
    });

export const getBest = (
    state: State,
): { [chainId: string]: OkRpcServerState } =>
    getAll(state).reduce(
        (acc: { [chainId: string]: OkRpcServerState }, server) => {
            if (server.type === RpcServerStateType.Ok) {
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
    state: State,
): OkRpcServerState | void => getBest(state)[chainId];

export const getBestByChainIdPrefix = (chainIdPrefix: string) => (
    state: State,
): OkRpcServerState | void =>
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
