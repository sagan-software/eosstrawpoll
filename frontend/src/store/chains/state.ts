export interface State {
    [chainId: string]: Chain;
}

export type Chain = ChainDefault | ChainChecking | ChainOk | ChainErr;

interface ChainBase {
    readonly status: Status;
    readonly chainId: string;
    readonly env: Env;
    readonly displayName: string;
    readonly contractName: string;
    readonly priority: Priority;
}

export interface ChainDefault extends ChainBase {
    readonly status: Status.Default;
}

export interface ChainChecking extends ChainBase {
    readonly status: Status.Checking;
    readonly coreSymbol?: string;
}

export interface ChainOk extends ChainBase {
    readonly status: Status.Ok;
    readonly coreSymbol: string;
}

export interface ChainErr extends ChainBase {
    readonly status: Status.Err;
    readonly error: Err;
}

export enum Env {
    Mainnet,
    Testnet,
    Devnet,
}

export enum Priority {
    High,
    Medium,
    Low,
}

export enum Status {
    Default,
    Checking,
    Ok,
    Err,
}

export enum Err {
    NoRpcServer,
    NoCoreSymbol,
    NoContractAccount,
    InvalidContractAbi,
}

export const eosMainnet: ChainDefault = {
    status: Status.Default,
    env: Env.Mainnet,
    chainId: 'aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906',
    displayName: 'EOS Mainnet',
    contractName: 'weosfund.x',
    priority: Priority.High,
};

export const eosDevnet: ChainDefault = {
    status: Status.Default,
    env: Env.Devnet,
    chainId: 'cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f',
    displayName: 'EOS Localnet',
    contractName: 'dappcontract',
    priority: Priority.Low,
};

export const telosMainnet: ChainDefault = {
    status: Status.Default,
    env: Env.Mainnet,
    chainId: '4667b205c6838ef70ff7988f6e8257e8be0e1284a2f59699054a018f743b1d11',
    displayName: 'Telos Mainnet',
    contractName: 'weosfund.x',
    priority: Priority.Medium,
};

export const telosTestnet: ChainDefault = {
    status: Status.Default,
    env: Env.Testnet,
    chainId: 'e17615decaecd202a365f4c029f206eee98511979de8a5756317e2469f2289e3',
    displayName: 'Telos Testnet',
    contractName: 'weosfund.x',
    priority: Priority.Low,
};

export const worbliMainnet: ChainDefault = {
    status: Status.Default,
    env: Env.Mainnet,
    chainId: '73647cde120091e0a4b85bced2f3cfdb3041e266cbbe95cee59b73235a1b3b6f',
    displayName: 'Worbli Mainnet',
    contractName: 'weosfund.x',
    priority: Priority.Medium,
};

export const jungleTestnet: ChainDefault = {
    status: Status.Default,
    env: Env.Testnet,
    chainId: 'e70aaab8997e1dfce58fbfac80cbbb8fecec7b99cf982a9444273cbc64c41473',
    displayName: 'Jungle Testnet',
    contractName: 'weosfund.x',
    priority: Priority.Low,
};

export const kylinTestnet: ChainDefault = {
    status: Status.Default,
    env: Env.Testnet,
    chainId: '5fff1dae8dc8e2fc4d5b23b2c7665c97f9e9d8edf2b6485a86ba311c25639191',
    displayName: 'Kylin Testnet',
    contractName: 'weosfund.x',
    priority: Priority.Low,
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
