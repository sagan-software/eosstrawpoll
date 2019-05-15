import { ExplorerState } from './state';

export enum ActionType {
    Upsert = 'EXPLORERS/UPSERT',
    Remove = 'EXPLORERS/REMOVE',
}

export type Action = UpsertAction | RemoveAction;

export interface UpsertAction extends ExplorerState {
    readonly type: ActionType.Upsert;
}

export function upsert(explorerState: ExplorerState): UpsertAction {
    return {
        ...explorerState,
        type: ActionType.Upsert,
    };
}

export interface RemoveAction {
    readonly type: ActionType.Remove;
    readonly host: string;
}

export function remove(host: string): RemoveAction {
    return {
        type: ActionType.Remove,
        host,
    };
}
