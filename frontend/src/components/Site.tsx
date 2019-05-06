import { Menu, MenuItem } from '@material-ui/core';
import Avatar from '@material-ui/core/Avatar';
import MuiButton, {
    ButtonProps as MuiButtonProps,
} from '@material-ui/core/Button';
import CircularProgress from '@material-ui/core/CircularProgress';
import MuiLink, { LinkProps as MuiLinkProps } from '@material-ui/core/Link';
import MuiTypography from '@material-ui/core/Typography';
import classNames from 'classnames';
import React, { useCallback } from 'react';
import * as Router from 'react-router-dom';
import * as Route from '../route';
import * as Store from '../store';
import { makeStyles } from '../styles';
import * as scatterMachine from '../machines/scatterMachine';
import { useService } from '@xstate/react';
import UserNav from './UserNav';

export type Props<E> = React.DetailedHTMLProps<React.HTMLAttributes<E>, E>;

const useContainerStyles = makeStyles({
    container: {
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        display: 'flex',
        flexDirection: 'column',
    },
});

export function Container(props: Props<HTMLDivElement>) {
    const classes = useContainerStyles();
    return (
        <div
            {...props}
            className={classNames(classes.container, props.className)}
        >
            {props.children}
        </div>
    );
}

const useHeaderStyles = makeStyles((theme) => ({
    header: {
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        padding: theme.spacing(2),
        paddingLeft: theme.spacing(4),
        paddingRight: theme.spacing(4),
        borderBottomStyle: 'solid',
        borderBottomWidth: 1,
        borderBottomColor: theme.palette.grey[300],
        backgroundColor: theme.palette.common.white,
    },
}));

export function Header(props: Props<HTMLElement>) {
    const classes = useHeaderStyles();
    return (
        <header
            {...props}
            className={classNames(classes.header, props.className)}
        >
            {props.children}
        </header>
    );
}

const useMainStyles = makeStyles({
    main: {
        flex: 1,
    },
});

export function Main(props: Props<HTMLElement>) {
    const classes = useMainStyles();
    return (
        <main {...props} className={classNames(classes.main, props.className)}>
            {props.children}
        </main>
    );
}

export interface ButtonProps extends MuiButtonProps {
    readonly to: Route.Route;
}

export function Button({ to, ...props }: ButtonProps) {
    const toStr = Route.getRouteString(to);
    const Inner = React.forwardRef((innerProps: any, ref) => (
        <Router.Link {...innerProps} to={toStr} innerRef={ref} />
    ));
    return (
        <MuiButton {...props} component={Inner}>
            {props.children}
        </MuiButton>
    );
}

export interface LinkProps extends MuiLinkProps {
    readonly to: Route.Route;
}

export function Link(props: LinkProps) {
    const to = Route.getRouteString(props.to);
    const Inner = (innerProps: any) => <Router.Link {...innerProps} to={to} />;
    return (
        <MuiLink component={Inner} {...props}>
            {props.children}
        </MuiLink>
    );
}

const useLogoStyles = makeStyles((theme) => ({
    logo: {
        flex: 1,
        fontFamily: '\'IBM Plex Sans Condensed\', sans-serif',
        fontSize: theme.typography.h4.fontSize,
        textAlign: 'center',
        fontWeight: 700,
    },
}));

export function Logo(props: Props<HTMLElement>) {
    const classes = useLogoStyles();
    return (
        <Link
            className={classNames(classes.logo, props.className)}
            to={Route.home()}
            color='primary'
            underline='none'
        >
            EOS Straw Poll
        </Link>
    );
}

const useSiteNavStyles = makeStyles((theme) => ({
    siteNav: {
        flex: 1,
        display: 'flex',
        '& > *': {
            marginRight: theme.spacing(4),
        },
    },
}));

export function SiteNav(props: Props<HTMLElement>) {
    const classes = useSiteNavStyles();
    return (
        <nav
            {...props}
            className={classNames(classes.siteNav, props.className)}
        />
    );
}

const useFooterStyles = makeStyles((theme) => ({
    footer: {
        borderTopStyle: 'solid',
        borderTopWidth: 2,
        borderTopColor: theme.palette.grey[600],
    },
}));

export function Footer(props: Props<HTMLElement>) {
    const classes = useFooterStyles();
    return (
        <footer
            {...props}
            className={classNames(classes.footer, props.className)}
        >
            <div>
                <p>
                    IPFS Hash{' '}
                    <code>QmYaXiNQT8qhsw2nN86yhRgSUaA8uit8DXaQFZuEcweuTz</code>
                </p>
            </div>
        </footer>
    );
}

export function Skeleton(props: Props<HTMLElement>) {
    return (
        <Container>
            <Header>
                <SiteNav />
                <Logo />
                <UserNav />
            </Header>
            <Main>{props.children}</Main>
            <Footer />
        </Container>
    );
}

export function Loading(props: Props<HTMLElement>) {
    const classes = useLogoStyles();
    return (
        <Container>
            <Header>
                <MuiLink
                    href='#'
                    className={classes.logo}
                    color='primary'
                    underline='none'
                >
                    EOS Straw Poll
                </MuiLink>
            </Header>
            <Main>
                <CircularProgress size={100} />
            </Main>
        </Container>
    );
}
