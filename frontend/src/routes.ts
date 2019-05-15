export type Route =
    | HomeRoute
    | SettingsRoute
    | ChainRoute
    | ProfileRoute
    | PollRoute;

export enum RouteType {
    Home = 'HOME',
    Settings = 'SETTINGS',
    Chain = 'CHAIN',
    Profile = 'PROFILE',
    Poll = 'POLL',
}

export interface HomeRoute {
    readonly type: RouteType.Home;
}

export function home(): HomeRoute {
    return { type: RouteType.Home };
}

export interface SettingsRoute {
    readonly type: RouteType.Settings;
}

export function settings(): SettingsRoute {
    return { type: RouteType.Settings };
}

export interface ChainRoute {
    readonly type: RouteType.Chain;
    readonly chainIdPrefix: string;
}

export function chain(chainIdPrefix: string): ChainRoute {
    return {
        type: RouteType.Chain,
        chainIdPrefix,
    };
}

export interface ProfileRoute {
    readonly type: RouteType.Profile;
    readonly chainIdPrefix: string;
    readonly accountName: string;
}

export function profile(
    chainIdPrefix: string,
    accountName: string,
): ProfileRoute {
    return {
        type: RouteType.Profile,
        chainIdPrefix,
        accountName,
    };
}

export interface PollRoute {
    readonly type: RouteType.Poll;
    readonly chainIdPrefix: string;
    readonly pollName: string;
    readonly showResults: boolean;
}

export function poll(
    chainIdPrefix: string,
    pollName: string,
    showResults: boolean,
): PollRoute {
    return {
        type: RouteType.Poll,
        chainIdPrefix,
        pollName,
        showResults,
    };
}

export function getRouteTemplate(type: RouteType): string {
    switch (type) {
    case RouteType.Home:
        return '/';
    case RouteType.Settings:
        return '/settings';
    case RouteType.Chain:
        return `/:chainIdPrefix/`;
    case RouteType.Profile:
        return `/:chainIdPrefix/:accountName/`;
    case RouteType.Poll:
        return `/:chainIdPrefix/:pollName/`;
    default:
        return '/';
    }
}

export function getRouteString(route: Route): string {
    switch (route.type) {
    case RouteType.Home:
        return '/';
    case RouteType.Settings:
        return '/settings';
    case RouteType.Chain:
        return `/${route.chainIdPrefix}/`;
    case RouteType.Profile:
        return `/${normalizeChainIdPrefix(route.chainIdPrefix)}/${
                route.accountName
            }`;
    case RouteType.Poll:
        return `/${normalizeChainIdPrefix(route.chainIdPrefix)}/${
                route.pollName
            }`;
    default:
        return '/';
    }
}

export function normalizeChainIdPrefix(chainIdPrefix: string): string {
    return chainIdPrefix.slice(0, 12);
}
