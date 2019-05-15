import * as chains from '../chains';

export interface State {
    [host: string]: ExplorerState;
}

export interface ExplorerState {
    readonly displayName: string;
    readonly host: string;
    readonly chainId: string;
    readonly account: string;
    readonly transaction: string;
    readonly block: string;
}

export const wwwEosxIo: ExplorerState = {
    displayName: 'www.eosx.io',
    host: 'www.eosx.io',
    chainId: chains.eosMainnet.chainId,
    account: 'https://www.eosx.io/account/{name}',
    transaction: 'https://www.eosx.io/tx/{id}',
    block: 'https://www.eosx.io/block/{num}',
};

export const kylinEosxIo: ExplorerState = {
    displayName: 'kylin.eosx.io',
    host: 'kylin.eosx.io',
    chainId: chains.kylinTestnet.chainId,
    account: 'https://kylin.eosx.io/account/{name}',
    transaction: 'https://kylin.eosx.io/tx/{id}',
    block: 'https://kylin.eosx.io/block/{num}',
};

export const jungleEosxIo: ExplorerState = {
    displayName: 'jungle.eosx.io',
    host: 'jungle.eosx.io',
    chainId: chains.jungleTestnet.chainId,
    account: 'https://jungle.eosx.io/account/{name}',
    transaction: 'https://jungle.eosx.io/tx/{id}',
    block: 'https://jungle.eosx.io/block/{num}',
};

export const worbliEosxIo: ExplorerState = {
    displayName: 'worbli.eosx.io',
    host: 'worbli.eosx.io',
    chainId: chains.worbliMainnet.chainId,
    account: 'https://worbli.eosx.io/account/{name}',
    transaction: 'https://worbli.eosx.io/tx/{id}',
    block: 'https://worbli.eosx.io/block/{num}',
};

export const telosEosxIo: ExplorerState = {
    displayName: 'telos.eosx.io',
    host: 'telos.eosx.io',
    chainId: chains.telosMainnet.chainId,
    account: 'https://telos.eosx.io/account/{name}',
    transaction: 'https://telos.eosx.io/tx/{id}',
    block: 'https://telos.eosx.io/block/{num}',
};

export const telosTestEosxIo: ExplorerState = {
    displayName: 'telos-test.eosx.io',
    host: 'telos-test.eosx.io',
    chainId: chains.telosTestnet.chainId,
    account: 'https://telos-test.eosx.io/account/{name}',
    transaction: 'https://telos-test.eosx.io/tx/{id}',
    block: 'https://telos-test.eosx.io/block/{num}',
};

export const wwwExampleDev: ExplorerState = {
    displayName: 'www.example.dev',
    host: 'www.example.dev',
    chainId: chains.eosDevnet.chainId,
    account: 'https://www.example.dev/account/{name}',
    transaction: 'https://www.example.dev/tx/{id}',
    block: 'https://www.example.dev/block/{num}',
};

// export const telosTestExampleDev: Explorer = {
//     displayName: 'telos-test.example.dev',
//     host: 'telos-test.example.dev',
//     chainId: Chains.eosDevnet.chainId,
//     account: 'https://telos-test.example.dev/account/{name}',
//     transaction: 'https://telos-test.example.dev/tx/{id}',
//     block: 'https://telos-test.example.dev/block/{num}',
// };

// TODO bos.eosx.io
// TODO bos-test.eosx.io
// TODO meetone.eosx.io
// TODO meetone-test.eosx.io
// TODO eosq.app
// TODO kylin.eosq.app
// TODO jungle.eosq.app
// TODO bloks.io
// TODO jungle.bloks.io
// TODO kylin.bloks.io
// TODO worbli.bloks.io
// TODO bos.bloks.io
// TODO eosflare.io
// TODO eospark.com
// TODO jungle.eospark.com
// TODO kylin.eospark.com
// TODO bos.eospark.com

export const initialState: State = [
    wwwEosxIo,
    kylinEosxIo,
    jungleEosxIo,
    worbliEosxIo,
    telosEosxIo,
    telosTestEosxIo,
    wwwExampleDev,
].reduce((acc: State, cur: ExplorerState) => {
    acc[cur.host] = cur;
    return acc;
}, {});
