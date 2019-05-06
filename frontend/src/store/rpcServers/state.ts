export interface State {
    [url: string]: Server;
}

export type Server = ServerUnknown | ServerChecking | ServerOk | ServerErr;

export enum Protocol {
    Https = 'https',
    Http = 'http',
}

export enum Status {
    Default,
    Checking,
    Ok,
    Err,
}

export interface ServerBase {
    readonly protocol: Protocol;
    readonly host: string;
    readonly port: number;
}

export interface ServerUnknown extends ServerBase {
    readonly status: Status.Default;
}

export interface ServerChecking extends ServerBase {
    readonly status: Status.Checking;
    readonly ping?: number;
    readonly chainId?: string;
}

export interface ServerOk extends ServerBase {
    readonly status: Status.Ok;
    readonly ping: number;
    readonly chainId: string;
}

export interface ServerErr extends ServerBase {
    readonly status: Status.Err;
    readonly message: string;
}

export function serverToUrl({
    protocol,
    host,
    port,
}: Server | ServerBase): string {
    return `${protocol}://${host}:${port}`;
}

export function serverFromUrl(url: string): ServerUnknown {
    const { protocol, hostname, port } = new URL(url);
    const isHttps = protocol.startsWith('https');
    return {
        status: Status.Default,
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
