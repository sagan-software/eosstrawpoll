import * as Eos from 'eosjs';
import isEqual from 'lodash.isequal';

type Abi = Eos.RpcInterfaces.Abi;

export const abi: Abi = require('./contract.abi.json');
// import wasm from '../../contract/build/src/contract.wasm';

export function isValidAbi(other: Abi): boolean {
    return (
        other &&
        other.version === abi.version &&
        isEqual(abi.actions, other.actions) &&
        isEqual(abi.structs, other.structs) &&
        isEqual(abi.tables, other.tables) &&
        isEqual(abi.types, other.types) &&
        isEqual(abi.variants, other.variants)
    );
}
