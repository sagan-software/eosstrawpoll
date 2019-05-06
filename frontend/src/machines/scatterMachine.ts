import { Machine, interpret, assign, actions } from 'xstate';
import Scatter from 'scatterjs-core';

export interface Schema {
    states: {
        connecting: {};
        connectOk: {
            states: {
                unknown: {};
                loggedOut: {};
                loggedIn: {};
                loggingOut: {};
                loggingIn: {};
                loginErr: {};
                logoutErr: {};
            };
        };
        connectErr: {};
    };
}

export interface Context {
    identity: Scatter.Identity | void;
}

export type Event =
    | { type: 'CONNECT'; appName: string }
    | { type: 'CONNECT_OK'; appName: string }
    | { type: 'CONNECT_ERR'; appName: string }
    | { type: 'CONNECT_RETRY'; appName: string }
    | { type: 'LOGIN'; options: Scatter.LoginOptions }
    | { type: 'LOGIN_OK' }
    | { type: 'LOGIN_ERR' }
    | { type: 'LOGOUT' }
    | { type: 'LOGOUT_OK' }
    | { type: 'LOGOUT_ERR' }
    | { type: 'LOGIN_RETRY' };

export const machine = Machine<Context, Schema, Event>(
    {
        id: 'scatter',
        context: { identity: void 0 },
        initial: 'connecting',
        strict: true,
        states: {
            connecting: {
                invoke: {
                    id: 'scatterConnect',
                    src: async (context, event) => {
                        const connected = await Scatter.connect('eosstrawpoll');
                        if (connected) {
                            return Promise.resolve();
                        } else {
                            return Promise.reject();
                        }
                    },
                    onError: {
                        target: 'connectErr',
                        actions: actions.log(),
                    },
                    onDone: {
                        target: 'connectOk.unknown',
                        actions: ['getIdentity'],
                    },
                },
            },
            connectOk: {
                states: {
                    unknown: {
                        on: {
                            '': [
                                { target: 'loggedIn', cond: 'hasAccount' },
                                { target: 'loggedOut' },
                            ],
                        },
                    },
                    loggedOut: {
                        on: {
                            LOGIN: 'loggingIn',
                        },
                    },
                    loggedIn: {
                        on: {
                            LOGOUT: 'loggingOut',
                        },
                    },
                    loggingOut: {
                        on: {
                            LOGOUT_OK: 'loggedOut',
                            LOGOUT_ERR: 'logoutErr',
                        },
                    },
                    loggingIn: {
                        invoke: {
                            id: 'scatterLogin',
                            src: (context, event) => {
                                return Scatter.login(event.options);
                            },
                            onError: {
                                target: 'loginErr',
                            },
                            onDone: {
                                target: 'loggedIn',
                                actions: ['getIdentity'],
                            },
                        },
                    },
                    loginErr: {
                        on: {
                            LOGIN_RETRY: 'loggingIn',
                        },
                    },
                    logoutErr: {},
                },
            },
            connectErr: {
                on: {
                    CONNECT_RETRY: 'connecting',
                },
            },
        },
    },
    {
        actions: {
            getIdentity: assign({
                identity: (_context, _event) => Scatter.identity,
            }),
        },
        guards: {
            hasAccount: (context, event) => {
                return context.identity
                    ? context.identity.accounts.length > 0
                    : false;
            },
        },
    },
);

export const service = interpret(machine, { devTools: true })
    .onTransition((state) => console.log(state.value))
    .start();
