import { Menu, MenuItem } from '@material-ui/core';
import Button from '@material-ui/core/Button';
import CircularProgress from '@material-ui/core/CircularProgress';
import React, { useCallback } from 'react';
import { makeStyles } from '../styles';
import * as scatter from '../machines/scatter';
import { useService } from '@xstate/react';

const useUserNavStyles = makeStyles((theme) => ({
    userNav: {
        flex: 1,
        display: 'flex',
        justifyContent: 'flex-end',
        '& > *': {
            marginLeft: theme.spacing(4),
        },
    },
}));

export default function UserNav() {
    const classes = useUserNavStyles();
    const [current, send] = useService(scatter.service);
    return (
        <nav className={classes.userNav}>
            {current.matches('connecting') ? (
                <Connecting />
            ) : current.matches('connectErr') ? (
                <ConnectError />
            ) : current.matches({ connectOk: 'loggedOut' }) ? (
                <LoggedOut />
            ) : current.matches({ connectOk: 'loggedIn' }) ? (
                'logged in'
            ) : current.matches({ connectOk: 'loggingIn' }) ? (
                'logging in'
            ) : current.matches({ connectOk: 'loggingOut' }) ? (
                'logging out'
            ) : (
                'other'
            )}
        </nav>
    );
}

function Connecting() {
    return (
        <Button color='primary' disabled>
            <CircularProgress />
        </Button>
    );
}

function ConnectError() {
    const [current, send] = useService(scatter.service);
    const onClick = useCallback(() => {
        send({
            type: 'CONNECT_RETRY',
            appName: 'eosstrawpoll',
        });
    }, [send]);
    return (
        <Button color='primary' onClick={onClick}>
            Retry
        </Button>
    );
}

function LoggedOut() {
    const [current, send] = useService(scatter.service);
    const onClick = useCallback(() => {
        send({
            type: 'LOGIN',
            options: {
                accounts: [
                    {
                        chainId:
                            'aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906',
                    },
                ],
            },
        });
    }, [send]);
    return (
        <Button color='primary' onClick={onClick}>
            Login
        </Button>
    );
}
