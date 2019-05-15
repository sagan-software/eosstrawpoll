export interface State {
    [chainId: string]: ChainState;
}

export enum ChainStateType {
    Default = 'DEFAULT',
    Checking = 'CHECKING',
    Ok = 'OK',
    Err = 'ERR',
}

export type ChainState =
    | DefaultChainState
    | CheckingChainState
    | OkChainState
    | ErrChainState;

interface ChainStateBase {
    readonly type: ChainStateType;
    readonly chainId: string;
    readonly env: ChainEnv;
    readonly displayName: string;
    readonly contractName: string;
}

export interface DefaultChainState extends ChainStateBase {
    readonly type: ChainStateType.Default;
}

export interface CheckingChainState extends ChainStateBase {
    readonly type: ChainStateType.Checking;
    readonly coreSymbol?: string;
}

export interface OkChainState extends ChainStateBase {
    readonly type: ChainStateType.Ok;
    readonly coreSymbol: string;
}

export interface ErrChainState extends ChainStateBase {
    readonly type: ChainStateType.Err;
    readonly error: ChainError;
}

export enum ChainEnv {
    Mainnet = 'MAINNET',
    Testnet = 'TESTNET',
    Devnet = 'DEVNET',
}

export enum ChainError {
    NoRpcServer = 'NO_RPC_SERVER',
    NoCoreSymbol = 'NO_CORE_SYMBOL',
    NoContractAccount = 'NO_CONTRACT_ACCOUNT',
    InvalidContractAbi = 'INVALID_CONTRACT_ABI',
}

export const eosMainnet: DefaultChainState = {
    type: ChainStateType.Default,
    env: ChainEnv.Mainnet,
    chainId: 'aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906',
    displayName: 'EOS Mainnet',
    contractName: 'weosfund.x',
};

export const eosDevnet: DefaultChainState = {
    type: ChainStateType.Default,
    env: ChainEnv.Devnet,
    chainId: 'cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f',
    displayName: 'EOS Localnet',
    contractName: 'dappcontract',
};

export const telosMainnet: DefaultChainState = {
    type: ChainStateType.Default,
    env: ChainEnv.Mainnet,
    chainId: '4667b205c6838ef70ff7988f6e8257e8be0e1284a2f59699054a018f743b1d11',
    displayName: 'Telos Mainnet',
    contractName: 'weosfund.x',
};

export const telosTestnet: DefaultChainState = {
    type: ChainStateType.Default,
    env: ChainEnv.Testnet,
    chainId: 'e17615decaecd202a365f4c029f206eee98511979de8a5756317e2469f2289e3',
    displayName: 'Telos Testnet',
    contractName: 'weosfund.x',
};

export const worbliMainnet: DefaultChainState = {
    type: ChainStateType.Default,
    env: ChainEnv.Mainnet,
    chainId: '73647cde120091e0a4b85bced2f3cfdb3041e266cbbe95cee59b73235a1b3b6f',
    displayName: 'Worbli Mainnet',
    contractName: 'weosfund.x',
};

export const jungleTestnet: DefaultChainState = {
    type: ChainStateType.Default,
    env: ChainEnv.Testnet,
    chainId: 'e70aaab8997e1dfce58fbfac80cbbb8fecec7b99cf982a9444273cbc64c41473',
    displayName: 'Jungle Testnet',
    contractName: 'weosfund.x',
};

export const kylinTestnet: DefaultChainState = {
    type: ChainStateType.Default,
    env: ChainEnv.Testnet,
    chainId: '5fff1dae8dc8e2fc4d5b23b2c7665c97f9e9d8edf2b6485a86ba311c25639191',
    displayName: 'Kylin Testnet',
    contractName: 'weosfund.x',
};

// TODO: meetOneMainnet
// TODO: meetOneTestnet
// TODO: meetOneDevnet
// TODO: telosDevnet
// TODO: bosMainnet
// TODO: bosTestnet
// TODO: bosDevnet
// TODO: worbliDevnet

export const initialState: State = [
    eosMainnet,
    jungleTestnet,
    eosDevnet,
    telosMainnet,
    telosTestnet,
    worbliMainnet,
].reduce((acc: State, cur) => {
    acc[cur.chainId] = cur;
    return acc;
}, {});
