import Scatter from 'scatterjs-core';
import ScatterEOS from 'scatterjs-plugin-eosjs2';

export * from './actions';
export * from './reducer';
export * from './state';
export * from './sagas';
export * from './selectors';

Scatter.plugins(new ScatterEOS());
