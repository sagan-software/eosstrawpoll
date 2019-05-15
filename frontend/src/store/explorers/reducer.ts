import { ActionType, UpsertAction, RemoveAction, Action } from './actions';
import { State, initialState } from './state';

export function reducer(state = initialState, action: Action): State {
    switch (action.type) {
    case ActionType.Upsert:
        return onUpsert(state, action);
    case ActionType.Remove:
        return onRemove(state, action);
    default:
        return state;
    }
}

function onUpsert(state: State, action: UpsertAction): State {
    return {
        ...state,
        [action.host]: {
            displayName: action.displayName,
            host: action.host,
            chainId: action.chainId,
            account: action.account,
            transaction: action.transaction,
            block: action.block,
        },
    };
}

function onRemove(state: State, action: RemoveAction): State {
    delete state[action.host];
    return {
        ...state,
    };
}
