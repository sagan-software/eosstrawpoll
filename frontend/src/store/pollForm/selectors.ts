import * as root from '../root';
import * as chains from '../chains';
import * as rpcServers from '../rpcServers';

export const getState = (state: root.State) => state.pollForm;

export const getChain = (state: root.State) =>
    chains.getById(state.pollForm.chainId)(state);

export const getRpcServer = (state: root.State) =>
    rpcServers.getBestByChainId(state.pollForm.chainId)(state);
