import * as State from './state';

export enum Type {
    Upsert = 'EXPLORERS/UPSERT',
    Remove = 'EXPLORERS/REMOVE',
}

export type Action = Upsert | Remove;

export interface Upsert extends State.Explorer {
    readonly type: Type.Upsert;
}

export interface Remove {
    readonly type: Type.Remove;
    readonly host: string;
}
