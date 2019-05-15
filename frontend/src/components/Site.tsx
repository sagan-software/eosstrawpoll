import Avatar from '@material-ui/core/Avatar';
import MuiButton, {
    ButtonProps as MuiButtonProps,
} from '@material-ui/core/Button';
import CircularProgress from '@material-ui/core/CircularProgress';
import MuiLink, { LinkProps as MuiLinkProps } from '@material-ui/core/Link';
import MuiTypography from '@material-ui/core/Typography';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import MuiIconButton, {
    IconButtonProps as MuiIconButtonProps,
} from '@material-ui/core/IconButton';
import MenuItem from '@material-ui/core/MenuItem';
import AccountCircle from '@material-ui/icons/AccountCircle';
import MenuIcon from '@material-ui/icons/Menu';
import Settings from '@material-ui/icons/Settings';
import Tooltip from '@material-ui/core/Tooltip';
import Menu from '@material-ui/core/Menu';
import classNames from 'classnames';
import Box from '@material-ui/core/Box';
import React, { useCallback } from 'react';
import * as Router from 'react-router-dom';
import { home, settings, Route, RouteType, getRouteString } from '../routes';
import * as Store from '../store';
import { makeStyles } from '../styles';
import UserNav from './UserNav';

export type Props<E> = React.DetailedHTMLProps<React.HTMLAttributes<E>, E>;

export function Header(props: Props<HTMLElement>) {
    return (
        <AppBar position='static'>
            <Toolbar>
                <Box display='flex' flex={1}>
                    <MuiIconButton
                        edge='start'
                        color='inherit'
                        aria-label='Menu'
                    >
                        <MenuIcon />
                    </MuiIconButton>

                    <Box
                        flex={1}
                        display='flex'
                        justifyContent='center'
                        alignItems='center'
                    >
                        <Logo />
                    </Box>
                    <div>
                        <Tooltip title='Settings' aria-label='Settings'>
                            <IconButton
                                to={settings()}
                                // aria-owns={open ? 'menu-appbar' : undefined}

                                // onClick={handleMenu}
                                color='inherit'
                            >
                                <Settings />
                            </IconButton>
                        </Tooltip>
                        <UserNav />
                    </div>
                </Box>
            </Toolbar>
        </AppBar>
    );
}

export interface ButtonProps extends MuiButtonProps {
    readonly to: Route;
}

export function Button({ to, ...props }: ButtonProps) {
    const toStr = getRouteString(to);
    const Inner = React.forwardRef((innerProps: any, ref) => (
        <Router.Link {...innerProps} to={toStr} innerRef={ref} />
    ));
    return (
        <MuiButton {...props} component={Inner}>
            {props.children}
        </MuiButton>
    );
}

export interface IconButtonProps extends MuiIconButtonProps {
    readonly to: Route;
}

export function IconButton({ to, ...props }: IconButtonProps) {
    const toStr = getRouteString(to);
    const Inner = React.forwardRef((innerProps: any, ref) => (
        <Router.Link {...innerProps} to={toStr} innerRef={ref} />
    ));
    return (
        <MuiIconButton {...props} component={Inner}>
            {props.children}
        </MuiIconButton>
    );
}

export interface LinkProps extends MuiLinkProps {
    readonly to: Route;
}

export function Link(props: LinkProps) {
    const to = getRouteString(props.to);
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
        fontSize: theme.typography.h5.fontSize,
        textAlign: 'center',
        fontWeight: 700,
    },
}));

export function Logo(props: Props<HTMLElement>) {
    const classes = useLogoStyles();
    return (
        <Link
            className={classNames(classes.logo, props.className)}
            to={home()}
            color='inherit'
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
    footer: {},
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
        <>
            <Header>
                <SiteNav />
                <Logo />
                <UserNav />
            </Header>
            <main>{props.children}</main>
            <Footer />
        </>
    );
}

export function Loading(props: Props<HTMLElement>) {
    return <CircularProgress size={100} />;
}
