import { randomName } from '../eos';

export enum StateType {
    Default,
    Submitting,
    SubmitOk,
    SubmitErr,
}

export type State =
    | DefaultState
    | SubmittingState
    | SubmitOkState
    | SubmitErrState;

export interface BaseState {
    readonly id: string;
    readonly name: string;
    readonly options: string[];
    readonly minAnswers: number;
    readonly maxAnswers: number;
    readonly minWriteins: number;
    readonly maxWriteins: number;
    readonly useAllowList: boolean;
    readonly voterList: string[];
    readonly minVoterAge: number;
    readonly minVoterHoldings: string[];
    readonly openTime: Date | null;
    readonly closeTime: Date | null;
    readonly chainId: string;
}

export interface DefaultState extends BaseState {
    readonly type: StateType.Default;
}

export interface SubmittingState extends BaseState {
    readonly type: StateType.Submitting;
}

export interface SubmitOkState extends BaseState {
    readonly type: StateType.SubmitOk;
    readonly transactionId: string;
}

export interface SubmitErrState extends BaseState {
    readonly type: StateType.SubmitErr;
    readonly error: any;
}

export const initialState: State = {
    type: StateType.Default,
    id: randomName(),
    name: '',
    options: ['', ''],
    minAnswers: 1,
    maxAnswers: 1,
    minWriteins: 0,
    maxWriteins: 0,
    useAllowList: true,
    voterList: [],
    minVoterAge: 0,
    minVoterHoldings: [],
    openTime: null,
    closeTime: null,
    chainId: '',
};
