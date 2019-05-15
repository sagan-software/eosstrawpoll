export enum ActionType {
    SetName = 'POLL_FORM/SET_NAME',
    AddOption = 'POLL_FORM/ADD_OPTION',
    DelOption = 'POLL_FORM/DEL_OPTION',
    SetOption = 'POLL_FORM/SET_OPTION',
    SetMinAnswers = 'POLL_FORM/SET_MIN_ANSWERS',
    SetMaxAnswers = 'POLL_FORM/SET_MAX_ANSWERS',
    SetMinWriteins = 'POLL_FORM/SET_MIN_WRITEINS',
    SetMaxWriteins = 'POLL_FORM/SET_MAX_WRITEINS',
    SetUseAllowList = 'POLL_FORM/SET_USE_ALLOW_LIST',
    SetMinVoterAgeSec = 'POLL_FORM/SET_MIN_VOTER_AGE_SEC',
    SetOpenTime = 'POLL_FORM/SET_OPEN_TIME',
    SetCloseTime = 'POLL_FORM/SET_CLOSE_TIME',
    AddVoter = 'POLL_FORM/ADD_VOTER',
    DelVoter = 'POLL_FORM/DEL_VOTER',
    AddMinVoterHoldings = 'POLL_FORM/ADD_MIN_VOTER_HOLDINGS',
    DelMinVoterHoldings = 'POLL_FORM/DEL_MIN_VOTER_HOLDINGS',
    SetMinVoterHoldings = 'POLL_FORM/SET_MIN_VOTER_HOLDINGS',
    SetChainId = 'POLL_FORM/SET_CHAIN_ID',
    Submit = 'POLL_FORM/SUBMIT',
    SubmitOk = 'POLL_FORM/SUBMIT_OK',
    SubmitErr = 'POLL_FORM/SUBMIT_ERR',
}

export type Action =
    | SetNameAction
    | AddOptionAction
    | DelOptionAction
    | SetOptionAction
    | SetMinAnswersAction
    | SetMaxAnswersAction
    | SetMinWriteinsAction
    | SetMaxWriteinsAction
    | SetUseAllowListAction
    | SetMinVoterAgeSecAction
    | SetOpenTimeAction
    | SetCloseTimeAction
    | AddVoterAction
    | DelVoterAction
    | AddMinVoterHoldingsAction
    | DelMinVoterHoldingsAction
    | SetMinVoterHoldingsAction
    | SetChainIdAction
    | SubmitAction
    | SubmitOkAction
    | SubmitErrAction;

export interface SetNameAction {
    readonly type: ActionType.SetName;
    readonly value: string;
}

export function setName(value: string): SetNameAction {
    return {
        type: ActionType.SetName,
        value,
    };
}

export interface AddOptionAction {
    readonly type: ActionType.AddOption;
}

export function addOption(): AddOptionAction {
    return { type: ActionType.AddOption };
}

export interface DelOptionAction {
    readonly type: ActionType.DelOption;
    readonly index: number;
}

export function delOption(index: number): DelOptionAction {
    return {
        type: ActionType.DelOption,
        index,
    };
}

export interface SetOptionAction {
    readonly type: ActionType.SetOption;
    readonly index: number;
    readonly value: string;
}

export function setOption(index: number, value: string): SetOptionAction {
    return {
        type: ActionType.SetOption,
        index,
        value,
    };
}

export interface SetMinAnswersAction {
    readonly type: ActionType.SetMinAnswers;
    readonly value: number;
}

export function setMinAnswers(value: number): SetMinAnswersAction {
    return {
        type: ActionType.SetMinAnswers,
        value,
    };
}

export interface SetMaxAnswersAction {
    readonly type: ActionType.SetMaxAnswers;
    readonly value: number;
}

export function setMaxAnswers(value: number): SetMaxAnswersAction {
    return {
        type: ActionType.SetMaxAnswers,
        value,
    };
}

export interface SetMinWriteinsAction {
    readonly type: ActionType.SetMinWriteins;
    readonly value: number;
}

export function setMinWriteins(value: number): SetMinWriteinsAction {
    return {
        type: ActionType.SetMinWriteins,
        value,
    };
}

export interface SetMaxWriteinsAction {
    readonly type: ActionType.SetMaxWriteins;
    readonly value: number;
}

export function setMaxWriteins(value: number): SetMaxWriteinsAction {
    return {
        type: ActionType.SetMaxWriteins,
        value,
    };
}

export interface SetUseAllowListAction {
    readonly type: ActionType.SetUseAllowList;
    readonly value: boolean;
}

export function setUseAllowList(value: boolean): SetUseAllowListAction {
    return {
        type: ActionType.SetUseAllowList,
        value,
    };
}

export interface SetMinVoterAgeSecAction {
    readonly type: ActionType.SetMinVoterAgeSec;
    readonly value: number;
}

export function setMinVoterAgeSec(value: number): SetMinVoterAgeSecAction {
    return {
        type: ActionType.SetMinVoterAgeSec,
        value,
    };
}

export interface SetOpenTimeAction {
    readonly type: ActionType.SetOpenTime;
    readonly value: Date;
}

export function setOpenTime(value: Date): SetOpenTimeAction {
    return {
        type: ActionType.SetOpenTime,
        value,
    };
}

export interface SetCloseTimeAction {
    readonly type: ActionType.SetCloseTime;
    readonly value: Date;
}

export function setCloseTime(value: Date): SetCloseTimeAction {
    return {
        type: ActionType.SetCloseTime,
        value,
    };
}

export interface AddVoterAction {
    readonly type: ActionType.AddVoter;
    readonly value: string;
}

export function addVoter(value: string): AddVoterAction {
    return {
        type: ActionType.AddVoter,
        value,
    };
}

export interface DelVoterAction {
    readonly type: ActionType.DelVoter;
    readonly index: number;
}

export function delVoter(index: number): DelVoterAction {
    return {
        type: ActionType.DelVoter,
        index,
    };
}

export interface AddMinVoterHoldingsAction {
    readonly type: ActionType.AddMinVoterHoldings;
    readonly value: string; // TODO EOSIO asset
}

export function addMinVoterHoldings(value: string): AddMinVoterHoldingsAction {
    return {
        type: ActionType.AddMinVoterHoldings,
        value,
    };
}

export interface DelMinVoterHoldingsAction {
    readonly type: ActionType.DelMinVoterHoldings;
    readonly index: number;
}

export function delMinVoterHoldings(index: number): DelMinVoterHoldingsAction {
    return {
        type: ActionType.DelMinVoterHoldings,
        index,
    };
}

export interface SetMinVoterHoldingsAction {
    readonly type: ActionType.SetMinVoterHoldings;
    readonly index: number;
    readonly value: string; // TODO EOSIO asset
}

export function setMinVoterHoldings(
    index: number,
    value: string,
): SetMinVoterHoldingsAction {
    return {
        type: ActionType.SetMinVoterHoldings,
        index,
        value,
    };
}

export interface SetChainIdAction {
    readonly type: ActionType.SetChainId;
    readonly value: string;
}

export function setChainId(value: string): SetChainIdAction {
    return {
        type: ActionType.SetChainId,
        value,
    };
}

export interface SubmitAction {
    readonly type: ActionType.Submit;
}

export function submit(): SubmitAction {
    return {
        type: ActionType.Submit,
    };
}

export interface SubmitOkAction {
    readonly type: ActionType.SubmitOk;
    readonly transactionId: string;
}

export function submitOk(transactionId: string): SubmitOkAction {
    return { type: ActionType.SubmitOk, transactionId };
}

export interface SubmitErrAction {
    readonly type: ActionType.SubmitErr;
    readonly error: any;
}

export function submitErr(error: any): SubmitErrAction {
    return { type: ActionType.SubmitErr, error };
}
