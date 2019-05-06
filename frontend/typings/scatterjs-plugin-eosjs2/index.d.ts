declare module 'scatterjs-plugin-eosjs2' {
    declare class ScatterEOS implements ScatterJS.Plugin {}
    export = ScatterEOS;
}

declare module 'scatterjs-core' {
    import eosjs from '../../node_modules/eosjs/dist';

    export function eos(network: any, api: any, options: any): eosjs.Api;
}
