import {
    Action,
    ActionType,
    UpsertAction,
    RemoveAction,
    CheckAction,
    SetOkAction,
    SetErrAction,
} from './actions';
import {
    State,
    initialState,
    ChainStateType,
    CheckingChainState,
    OkChainState,
    ErrChainState,
} from './state';

export function reducer(state = initialState, action: Action): State {
    switch (action.type) {
    case ActionType.Upsert:
        return onUpsert(state, action);
    case ActionType.Remove:
        return onRemove(state, action);
    case ActionType.Check:
        return onCheck(state, action);
    case ActionType.SetOk:
        return onSetOk(state, action);
    case ActionType.SetErr:
        return onSetErr(state, action);
    default:
        return state;
    }
}

function onUpsert(state: State, action: UpsertAction): State {
    return {
        ...state,
        [action.chainId]: {
            type: ChainStateType.Default,
            env: action.env,
            chainId: action.chainId,
            displayName: action.displayName,
            contractName: action.contractName,
        },
    };
}

function onRemove(state: State, action: RemoveAction): State {
    if (action.chainId in state) {
        delete state[action.chainId];
        return { ...state };
    } else {
        return state;
    }
}

function onCheck(state: State, { server }: CheckAction): State {
    const chainId = server.chainId;
    if (chainId in state) {
        const oldChain = state[chainId];
        const newChain: CheckingChainState = {
            type: ChainStateType.Checking,
            env: oldChain.env,
            chainId,
            displayName: oldChain.displayName,
            contractName: oldChain.contractName,
        };
        return {
            ...state,
            [chainId]: newChain,
        };
    } else {
        return state;
    }
}

function onSetOk(state: State, { chainId, coreSymbol }: SetOkAction): State {
    if (chainId in state) {
        const oldChain = state[chainId];
        const newChain: OkChainState = {
            type: ChainStateType.Ok,
            env: oldChain.env,
            chainId,
            displayName: oldChain.displayName,
            contractName: oldChain.contractName,
            coreSymbol,
        };
        return {
            ...state,
            [chainId]: newChain,
        };
    } else {
        return state;
    }
}

function onSetErr(state: State, { chainId, error }: SetErrAction): State {
    if (chainId in state) {
        const oldChain = state[chainId];
        const newChain: ErrChainState = {
            type: ChainStateType.Err,
            env: oldChain.env,
            chainId,
            displayName: oldChain.displayName,
            contractName: oldChain.contractName,
            error,
        };
        return {
            ...state,
            [chainId]: newChain,
        };
    } else {
        return state;
    }
}
