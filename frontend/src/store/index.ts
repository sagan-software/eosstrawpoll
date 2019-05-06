import { applyMiddleware, createStore, Store } from 'redux';
import { composeWithDevTools } from 'redux-devtools-extension';
import { create } from 'redux-react-hook';
import createSagaMiddleware from 'redux-saga';
import * as App from './app';
import * as Chains from './chains';
import * as Explorers from './explorers';
import * as Root from './root';
import * as RpcServers from './rpcServers';
import * as Scatter from './scatter';

export function makeStore(): Store<Root.State, Root.Action> {
    const sagaMiddleware = createSagaMiddleware();
    const enhancer = applyMiddleware(sagaMiddleware);
    const store = createStore<Root.State, Root.Action, {}, {}>(
        Root.reducer,
        composeWithDevTools({ name: 'weos.fund' })(enhancer),
    );
    sagaMiddleware.run(Root.saga);
    return store;
}

export const { StoreContext, useDispatch, useMappedState } = create<
    Root.State,
    Root.Action,
    Store<Root.State, Root.Action>
>();

export { App, Chains, Explorers, Root, RpcServers, Scatter };
export default {
    App,
    Chains,
    Explorers,
    Root,
    RpcServers,
    Scatter,
    useDispatch,
    useMappedState,
    StoreContext,
};
