declare module 'scatterjs-core' {
    // export class Network {
    //     constructor(...args: any[]);
    //     public fullhost(): string;
    //     public unique(): string;
    //     public static fromJson(b: NetworkJson): Network;
    //     public static placeholder(): Network;
    // }

    export interface PartialNetwork {
        name?: string;
        protocol?: string;
        host?: string;
        port?: number;
        blockchain?: string;
        chainId?: string;
        token?: string;
    }

    export interface FullNetwork {
        name: string;
        protocol: string;
        host: string;
        port: number;
        blockchain: string;
        chainId: string;
        token?: string;
    }

    export class Plugin {
        constructor(...args: any[]);

        // isSignatureProvider(): any;

        // isValid(): any;

        // static fromJson(b: any): any;

        // static placeholder(): any;
    }

    export function plugins(plugin: Plugin);

    export interface ConnectOptions {
        network?: PartialNetwork;
    }

    export function connect(
        appName: string,
        options?: ConnectOptions,
    ): Promise<boolean>;

    export interface LoginOptions {
        accounts: PartialNetwork[];
    }

    export function login(options: LoginOptions): Promise<Identity>;

    export function logout(): Promise<boolean>;

    export function account(blockchain: string): Account | void;

    export interface Error<T> {
        readonly type: T;
        readonly message: string;
        readonly code: number;
    }

    export enum LoginErrorType {
        IdentityRejected = 'identity_rejected',
        NoNetwork = 'no_network',
    }

    export type LoginError = Error<LoginErrorType>;

    export interface Identity {
        readonly hash: string;
        readonly kyc: boolean;
        readonly name: string;
        readonly publicKey: string;
        readonly accounts: ReadonlyArray<Account>;
    }

    export const identity: Identity | void;

    export interface Account {
        readonly authority: string;
        readonly blockchain: string;
        readonly chainId: string;
        readonly isHardware: boolean;
        readonly name: string;
        readonly publicKey: string;
    }

    export enum SuggestNetworkErrorType {
        BadNetwork = 'bad_network',
    }

    export type SuggestNetworkError = Error<SuggestNetworkErrorType>;

    export function suggestNetwork(network: FullNetwork): Promise<boolean>;
}
