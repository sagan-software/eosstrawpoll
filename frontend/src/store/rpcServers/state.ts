export interface State {
    [url: string]: RpcServerState;
}

export enum RpcServerStateType {
    Default = 'DEFAULT',
    Checking = 'CHECKING',
    Ok = 'OK',
    Err = 'ERR',
}

export type RpcServerState =
    | UnknownRpcServerState
    | CheckingRpcServerState
    | OkRpcServerState
    | ErrRpcServerState;

export enum Protocol {
    Https = 'https',
    Http = 'http',
}

export interface BaseRpcServerState {
    readonly protocol: Protocol;
    readonly host: string;
    readonly port: number;
}

export interface UnknownRpcServerState extends BaseRpcServerState {
    readonly type: RpcServerStateType.Default;
}

export interface CheckingRpcServerState extends BaseRpcServerState {
    readonly type: RpcServerStateType.Checking;
    readonly ping?: number;
    readonly chainId?: string;
}

export interface OkRpcServerState extends BaseRpcServerState {
    readonly type: RpcServerStateType.Ok;
    readonly ping: number;
    readonly chainId: string;
}

export interface ErrRpcServerState extends BaseRpcServerState {
    readonly type: RpcServerStateType.Err;
    readonly message: string;
}

export function serverToUrl({
    protocol,
    host,
    port,
}: RpcServerState | BaseRpcServerState): string {
    return `${protocol}://${host}:${port}`;
}

export function serverFromUrl(url: string): UnknownRpcServerState {
    const { protocol, hostname, port } = new URL(url);
    const isHttps = protocol.startsWith('https');
    return {
        type: RpcServerStateType.Default,
        protocol: isHttps ? Protocol.Https : Protocol.Http,
        host: hostname,
        port: port ? parseInt(port, 10) : isHttps ? 443 : 80,
    };
}

export const initialState: State = [
    'https://api.eosnewyork.io',
    'https://127.0.0.1:8889',
    'https://api.telosfoundation.io',
    'https://api.worbli.io',
    'https://api.jungle.alohaeos.com',
    'https://jungle2.cryptolions.io',
    'https://telos.caleos.io',
].reduce((acc: State, url) => {
    const server = serverFromUrl(url);
    const newUrl = serverToUrl(server);
    acc[newUrl] = server;
    return acc;
}, {});
