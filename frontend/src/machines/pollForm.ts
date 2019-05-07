import { Machine, assign, interpret } from 'xstate';

interface Schema {
    states: {
        dataEntry: {};
        submitting: {};
        submitOk: {};
        submitErr: {};
    };
}

interface Context {
    name: string;
    options: string[];
    minAnswers: number;
    maxAnswers: number;
    minWriteins: number;
    maxWriteins: number;
    voterListShouldAllow: boolean;
    voterList: string[];
    minVoterAge: number;
    minVoterHoldings: string[];
    openTime: Date;
    closeTime: Date;
}

type Event =
    | { type: 'SET_NAME'; value: string }
    | { type: 'SUBMIT' }
    | { type: 'ADD_OPTION' }
    | { type: 'DEL_OPTION'; index: number }
    | { type: 'SET_OPTION'; index: number; value: string }
    | { type: 'SET_MIN_ANSWERS'; value: number }
    | { type: 'SET_MAX_ANSWERS'; value: number }
    | { type: 'SET_MIN_WRITEINS'; value: number }
    | { type: 'SET_MAX_WRITEINS'; value: number }
    | { type: 'SET_VOTER_LIST_SHOULD_ALLOW'; value: boolean }
    | { type: 'SET_MIN_VOTER_AGE'; value: number }
    | { type: 'SET_OPEN_TIME'; value: Date }
    | { type: 'SET_CLOSE_TIME'; value: Date }
    | { type: 'ADD_VOTER'; value: string }
    | { type: 'DEL_VOTER'; index: number }
    | { type: 'ADD_VOTER_HOLDINGS'; value: string }
    | { type: 'DEL_VOTER_HOLDINGS'; index: number }
    | { type: 'SET_VOTER_HOLDINGS'; index: number; value: string };

export const machine = Machine<Context, Schema, Event>({
    key: 'pollForm',
    initial: 'dataEntry',
    context: {
        name: '',
        options: ['', ''],
        minAnswers: 1,
        maxAnswers: 1,
        minWriteins: 0,
        maxWriteins: 0,
        voterListShouldAllow: true,
        voterList: [],
        minVoterAge: 0,
        minVoterHoldings: [],
        openTime: new Date(),
        closeTime: new Date(),
    },
    states: {
        dataEntry: {
            on: {
                SET_NAME: {
                    actions: assign((c, e) => ({
                        name: e.value,
                    })),
                },
                ADD_OPTION: {
                    actions: assign((c, e) => {
                        c.options.push('');
                        return c;
                    }),
                },
                DEL_OPTION: {
                    actions: assign((c, e) => {
                        if (c.options.length >= e.index) {
                            c.options.splice(e.index, 1);
                        }
                        return c;
                    }),
                },
                SET_OPTION: {
                    actions: assign((c, e) => {
                        if (c.options.length >= e.index) {
                            c.options[e.index] = e.value;
                        }
                        return c;
                    }),
                },
                SET_OPEN_TIME: {
                    actions: assign((c, e) => ({
                        openTime: e.value,
                    })),
                },
                SET_CLOSE_TIME: {
                    actions: assign((c, e) => ({
                        closeTime: e.value,
                    })),
                },
                SUBMIT: 'submitting',
            },
        },
        submitting: {},
        submitOk: {},
        submitErr: {},
    },
});

export const service = interpret(machine, { devTools: true })
    .onTransition((state) => console.log(state.value))
    .start();
