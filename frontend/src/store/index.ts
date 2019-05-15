import { applyMiddleware, createStore, Store } from 'redux';
import { composeWithDevTools } from 'redux-devtools-extension';
import { create } from 'redux-react-hook';
import createSagaMiddleware from 'redux-saga';
import * as app from './app';
import * as chains from './chains';
import * as pollForm from './pollForm';
import * as explorers from './explorers';
import * as root from './root';
import * as rpcServers from './rpcServers';
import * as scatter from './scatter';

export function makeStore(): Store<root.State, root.Action> {
    const sagaMiddleware = createSagaMiddleware();
    const enhancer = applyMiddleware(sagaMiddleware);
    const store = createStore<root.State, root.Action, {}, {}>(
        root.reducer,
        composeWithDevTools({ name: 'eosstrawpoll' })(enhancer),
    );
    sagaMiddleware.run(root.saga);
    return store;
}

export const { StoreContext, useDispatch, useMappedState } = create<
    root.State,
    root.Action,
    Store<root.State, root.Action>
>();

export { app, chains, explorers, pollForm, root, rpcServers, scatter };
export default {
    app,
    chains,
    explorers,
    pollForm,
    root,
    rpcServers,
    scatter,
    useDispatch,
    useMappedState,
    StoreContext,
};
