export type Route = Home;

export enum Type {
    Home,
}

export interface Home {
    readonly type: Type.Home;
}

export function home(): Home {
    return { type: Type.Home };
}

export function getRouteTemplate(type: Type): string {
    switch (type) {
    case Type.Home:
        return '/';
    default:
        return '/';
    }
}

export function getRouteString(route: Route): string {
    switch (route.type) {
    case Type.Home:
        return '/';
    default:
        return '/';
    }
}

export function normalizeChainIdPrefix(chainIdPrefix: string): string {
    return chainIdPrefix.slice(0, 12);
}
