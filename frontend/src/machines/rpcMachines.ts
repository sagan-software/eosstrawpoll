import { Machine, interpret, assign, actions } from 'xstate';
import * as Eos from 'eosjs';

export interface Schema {
    states: {
        checking: {};
        ok: {};
        err: {};
    };
}

export interface Context {
    protocol: 'https' | 'http';
    host: string;
    port: number;
    ping?: number;
    chainId?: string;
    rpc?: Eos.JsonRpc;
}

export function contextToUrl({ protocol, host, port }: Context): string {
    return `${protocol}://${host}:${port}`;
}

export function contextFromUrl(url: string): Context {
    const { protocol, hostname, port } = new URL(url);
    const isHttps = protocol.startsWith('https');
    return {
        protocol: isHttps ? 'https' : 'http',
        host: hostname,
        port: port ? parseInt(port, 10) : isHttps ? 443 : 80,
    };
}

export type Event = { type: 'OK' } | { type: 'ERR' } | { type: 'CHECK' };

export const machine = Machine<Context, Schema, Event>({
    id: 'rpc',
    context: {
        protocol: 'https',
        host: '',
        port: 80,
    },
    initial: 'checking',
    strict: true,
    states: {
        checking: {
            invoke: {
                id: 'rpcChecking',
                src: async (context, event) => {
                    const url = contextToUrl(context);
                    const rpc = new Eos.JsonRpc(url);
                    const start = new Date();
                    const info = await rpc.get_info();
                    const end = new Date();
                    const ping = end.getTime() - start.getTime();
                    return {
                        ping,
                        chainId: info.chain_id,
                        rpc,
                    };
                },
                onError: {
                    target: 'err',
                },
                onDone: {
                    target: 'ok',
                    actions: assign((context, event) => ({
                        ping: event.data.ping,
                        chainId: event.data.chainId,
                        rpc: event.data.rpc,
                    })),
                },
            },
        },
        ok: {
            on: {
                CHECK: 'checking',
            },
        },
        err: {
            on: {
                CHECK: 'checking',
            },
        },
    },
});
