import * as Action from './action';
import * as State from './state';

export function reducer(
    state = State.initialState,
    action: Action.Action,
): State.State {
    switch (action.type) {
    case Action.Type.Upsert:
        return onUpsert(state, action);
    case Action.Type.Remove:
        return onRemove(state, action);
    default:
        return state;
    }
}

function onUpsert(state: State.State, action: Action.Upsert): State.State {
    return {
        ...state,
    };
}

function onRemove(state: State.State, action: Action.Remove): State.State {
    return {
        ...state,
    };
}
