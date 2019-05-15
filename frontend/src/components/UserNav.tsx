import Button from '@material-ui/core/Button';
import CircularProgress from '@material-ui/core/CircularProgress';
import IconButton from '@material-ui/core/IconButton';
import MenuItem from '@material-ui/core/MenuItem';
import AccountCircle from '@material-ui/icons/AccountCircle';
import MenuIcon from '@material-ui/icons/Menu';
import Menu from '@material-ui/core/Menu';
import Settings from '@material-ui/icons/Settings';
import Tooltip from '@material-ui/core/Tooltip';
import React, { useCallback } from 'react';
import { useMappedState, useDispatch, scatter } from '../store';

export default function UserNav() {
    const state = useMappedState(useCallback(scatter.getState, []));
    switch (state.type) {
    case scatter.StateType.Connecting:
        return <Connecting />;
    case scatter.StateType.Unavailable:
        return <ConnectError />;
    case scatter.StateType.Connected:
        return (
                <>
                    <Tooltip title='Account'>
                        <IconButton
                            // aria-owns={open ? 'menu-appbar' : undefined}
                            aria-haspopup='true'
                            // onClick={handleMenu}
                            color='inherit'
                        >
                            <AccountCircle />
                        </IconButton>
                    </Tooltip>
                    <Menu
                        open={false}
                        id='menu-appbar'
                        // anchorEl={anchorEl}
                        anchorOrigin={{
                            vertical: 'top',
                            horizontal: 'right',
                        }}
                        transformOrigin={{
                            vertical: 'top',
                            horizontal: 'right',
                        }}
                        // open={open}
                        // onClose={handleClose}
                    >
                        <MenuItem>Profile</MenuItem>
                        <MenuItem>My account</MenuItem>
                    </Menu>
                </>
            );
    default:
        return <>default</>;
    }
}

function Connecting() {
    return (
        <Button color='inherit' disabled>
            <CircularProgress />
        </Button>
    );
}

function ConnectError() {
    const dispatch = useDispatch();
    const onClick = useCallback(
        (e) => {
            e.preventDefault();
            dispatch(scatter.connect('eosstrawpoll'));
        },
        [dispatch],
    );
    return (
        <Button color='inherit' onClick={onClick}>
            Retry
        </Button>
    );
}

function LoggedOut() {
    // const [current, send] = useService(scatter.service);
    const onClick = useCallback(() => {
        // send({
        //     type: 'LOGIN',
        //     options: {
        //         accounts: [
        //             {
        //                 chainId:
        //                     'aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906',
        //             },
        //         ],
        //     },
        // });
    }, []);
    return (
        <Button color='primary' onClick={onClick}>
            Login
        </Button>
    );
}
