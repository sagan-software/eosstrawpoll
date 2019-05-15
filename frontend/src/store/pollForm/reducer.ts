import {
    Action,
    ActionType,
    SetNameAction,
    AddOptionAction,
    DelOptionAction,
    SetOptionAction,
    SetMinAnswersAction,
    SetMaxAnswersAction,
    SetMinWriteinsAction,
    SetMaxWriteinsAction,
    SetUseAllowListAction,
    SetMinVoterAgeAction,
    SetOpenTimeAction,
    SetCloseTimeAction,
    AddVoterAction,
    DelVoterAction,
    AddMinVoterHoldingsAction,
    DelMinVoterHoldingsAction,
    SetMinVoterHoldingsAction,
    SetChainIdAction,
    SubmitAction,
    SubmitOkAction,
    SubmitErrAction,
} from './actions';
import { State, initialState, StateType } from './state';

export function reducer(state = initialState, action: Action): State {
    switch (action.type) {
    case ActionType.SetName:
        return onSetName(state, action);
    case ActionType.AddOption:
        return onAddOption(state, action);
    case ActionType.DelOption:
        return onDelOption(state, action);
    case ActionType.SetOption:
        return onSetOption(state, action);
    case ActionType.SetMinAnswers:
        return onSetMinAnswers(state, action);
    case ActionType.SetMaxAnswers:
        return onSetMaxAnswers(state, action);
    case ActionType.SetMinWriteins:
        return onSetMinWriteins(state, action);
    case ActionType.SetMaxWriteins:
        return onSetMaxWriteins(state, action);
    case ActionType.SetUseAllowList:
        return onSetUseAllowList(state, action);
    case ActionType.SetMinVoterAge:
        return onSetMinVoterAge(state, action);
    case ActionType.SetOpenTime:
        return onSetOpenTime(state, action);
    case ActionType.SetCloseTime:
        return onSetCloseTime(state, action);
    case ActionType.AddVoter:
        return onAddVoter(state, action);
    case ActionType.DelVoter:
        return onDelVoter(state, action);
    case ActionType.AddMinVoterHoldings:
        return onAddMinVoterHoldings(state, action);
    case ActionType.DelMinVoterHoldings:
        return onDelMinVoterHoldings(state, action);
    case ActionType.SetMinVoterHoldings:
        return onSetMinVoterHoldings(state, action);
    case ActionType.SetChainId:
        return onSetChainId(state, action);
    case ActionType.Submit:
        return onSubmit(state, action);
    case ActionType.SubmitOk:
        return onSubmitOk(state, action);
    case ActionType.SubmitErr:
        return onSubmitErr(state, action);
    default:
        return state;
    }
}

function onSetName(state: State, action: SetNameAction): State {
    return {
        ...state,
        name: action.value,
    };
}

function onAddOption(state: State, action: AddOptionAction): State {
    state.options.push('');
    return {
        ...state,
        options: [...state.options],
    };
}

function onDelOption(state: State, action: DelOptionAction): State {
    if (action.index < state.options.length) {
        state.options.splice(action.index, 1);
    }
    return { ...state, options: [...state.options] };
}

function onSetOption(state: State, action: SetOptionAction): State {
    if (action.index < state.options.length) {
        state.options[action.index] = action.value;
    }
    return { ...state, options: [...state.options] };
}

function onSetMinAnswers(state: State, action: SetMinAnswersAction): State {
    return {
        ...state,
        minAnswers: action.value,
    };
}

function onSetMaxAnswers(state: State, action: SetMaxAnswersAction): State {
    return {
        ...state,
        maxAnswers: action.value,
    };
}

function onSetMinWriteins(state: State, action: SetMinWriteinsAction): State {
    return {
        ...state,
        minWriteins: action.value,
    };
}

function onSetMaxWriteins(state: State, action: SetMaxWriteinsAction): State {
    return {
        ...state,
        maxWriteins: action.value,
    };
}

function onSetUseAllowList(state: State, action: SetUseAllowListAction): State {
    return {
        ...state,
        useAllowList: action.value,
    };
}

function onSetMinVoterAge(state: State, action: SetMinVoterAgeAction): State {
    return {
        ...state,
        minVoterAge: action.value,
    };
}

function onSetOpenTime(state: State, action: SetOpenTimeAction): State {
    return {
        ...state,
        openTime: action.value,
    };
}

function onSetCloseTime(state: State, action: SetCloseTimeAction): State {
    return {
        ...state,
        closeTime: action.value,
    };
}

function onAddVoter(state: State, action: AddVoterAction): State {
    state.voterList.push(action.value);
    return {
        ...state,
        voterList: [...state.voterList],
    };
}

function onDelVoter(state: State, action: DelVoterAction): State {
    if (action.index < state.voterList.length) {
        state.voterList.splice(action.index, 1);
    }
    return {
        ...state,
        voterList: [...state.voterList],
    };
}

function onAddMinVoterHoldings(
    state: State,
    action: AddMinVoterHoldingsAction,
): State {
    state.minVoterHoldings.push(action.value);
    return {
        ...state,
        minVoterHoldings: [...state.minVoterHoldings],
    };
}

function onDelMinVoterHoldings(
    state: State,
    action: DelMinVoterHoldingsAction,
): State {
    if (action.index < state.minVoterHoldings.length) {
        state.minVoterHoldings.splice(action.index, 1);
    }
    return {
        ...state,
        minVoterHoldings: [...state.minVoterHoldings],
    };
}

function onSetMinVoterHoldings(
    state: State,
    action: SetMinVoterHoldingsAction,
): State {
    if (action.index < state.minVoterHoldings.length) {
        state.minVoterHoldings[action.index] = action.value;
    }
    return {
        ...state,
        minVoterHoldings: [...state.minVoterHoldings],
    };
}

function onSetChainId(state: State, action: SetChainIdAction): State {
    return {
        ...state,
        chainId: action.value,
    };
}

function onSubmit(state: State, action: SubmitAction): State {
    return {
        ...state,
        type: StateType.Submitting,
    };
}

function onSubmitOk(state: State, action: SubmitOkAction): State {
    return {
        ...state,
        type: StateType.SubmitOk,
        transactionId: action.transactionId,
    };
}

function onSubmitErr(state: State, action: SubmitErrAction): State {
    return {
        ...state,
        type: StateType.SubmitErr,
        error: action.error,
    };
}
