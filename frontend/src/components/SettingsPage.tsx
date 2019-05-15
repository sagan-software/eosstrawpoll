import React, { useCallback, useState } from 'react';
import * as Site from './Site';
import Container from '@material-ui/core/Container';
import Grid from '@material-ui/core/Grid';
import Fade from '@material-ui/core/Fade';
import { makeStyles, Theme } from '@material-ui/core/styles';
import ExpansionPanel from '@material-ui/core/ExpansionPanel';
import ExpansionPanelDetails from '@material-ui/core/ExpansionPanelDetails';
import ExpansionPanelSummary from '@material-ui/core/ExpansionPanelSummary';
import Typography from '@material-ui/core/Typography';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import Box from '@material-ui/core/Box';
import MaterialTable from 'material-table';
import {
    useMappedState,
    useDispatch,
    rpcServers,
    chains,
    explorers,
} from '../store';

const useStyles = makeStyles((theme: Theme) => ({
    root: {
        width: '100%',
    },
    heading: {
        fontSize: theme.typography.pxToRem(15),
        flexBasis: '33.33%',
        flexShrink: 0,
    },
    secondaryHeading: {
        fontSize: theme.typography.pxToRem(15),
        color: theme.palette.text.secondary,
    },
}));

export default function SettingsPage() {
    const rpcServersState = useMappedState(useCallback(rpcServers.getAll, []));
    const chainsState = useMappedState(useCallback(chains.getAll, []));
    const explorersState = useMappedState(useCallback(explorers.getAll, []));
    const dispatch = useDispatch();
    const classes = useStyles();
    const [expanded, setExpanded] = useState<string | null>(null);

    const handleChange = (panel: string) => (
        event: React.ChangeEvent<{}>,
        isExpanded: boolean,
    ) => {
        setExpanded(isExpanded ? panel : null);
    };
    return (
        <Site.Skeleton>
            <Container maxWidth='lg'>
                <Box m={4}>
                    <Typography variant='h4' gutterBottom align='center'>
                        Settings
                    </Typography>
                </Box>
                <Grid container spacing={5}>
                    <MaterialTable
                        title='RPC Servers'
                        columns={[
                            { title: 'URL', field: 'url' },
                            { title: 'Ping', field: 'ping', editable: 'never' },
                            {
                                title: 'Chain ID',
                                field: 'chainId',
                                editable: 'never',
                            },
                        ]}
                        data={rpcServersState.map((rpcServer) => ({
                            url: rpcServers.serverToUrl(rpcServer),
                            ping:
                                rpcServer.type ===
                                rpcServers.RpcServerStateType.Ok
                                    ? rpcServer.ping
                                    : -1,
                            chainId:
                                rpcServer.type ===
                                rpcServers.RpcServerStateType.Ok
                                    ? rpcServer.chainId
                                    : '',
                        }))}
                        editable={{
                            onRowAdd: (newData) =>
                                new Promise((resolve, reject) => {
                                    setTimeout(() => {
                                        //   {
                                        //     const data = this.state.data;
                                        //     data.push(newData);
                                        //     this.setState({ data }, () => resolve());
                                        //   }
                                        resolve();
                                    }, 1000);
                                }),
                            onRowUpdate: (newData, oldData) =>
                                new Promise((resolve, reject) => {
                                    setTimeout(() => {
                                        //   {
                                        //     const data = this.state.data;
                                        //     const index = data.indexOf(oldData);
                                        //     data[index] = newData;
                                        //     this.setState({ data }, () => resolve());
                                        //   }
                                        resolve();
                                    }, 1000);
                                }),
                            onRowDelete: (oldData) =>
                                new Promise((resolve, reject) => {
                                    setTimeout(() => {
                                        //   {
                                        //     let data = this.state.data;
                                        //     const index = data.indexOf(oldData);
                                        //     data.splice(index, 1);
                                        //     this.setState({ data }, () => resolve());
                                        //   }
                                        resolve();
                                    }, 1000);
                                }),
                        }}
                    />
                    <MaterialTable
                        title='Chains'
                        columns={[
                            { title: 'Name', field: 'name' },
                            { title: 'Type', field: 'env' },
                            { title: 'Contract', field: 'contract' },
                            {
                                title: 'Chain ID',
                                field: 'chainId',
                                editable: 'never',
                            },
                        ]}
                        data={chainsState.map((chain) => ({
                            name: chain.displayName,
                            env:
                                chain.env === chains.ChainEnv.Mainnet
                                    ? 'Main net'
                                    : chain.env === chains.ChainEnv.Testnet
                                    ? 'Test net'
                                    : 'Dev net',
                            contract: chain.contractName,
                            chainId: chain.chainId,
                        }))}
                        editable={{
                            onRowAdd: (newData) =>
                                new Promise((resolve, reject) => {
                                    setTimeout(() => {
                                        //   {
                                        //     const data = this.state.data;
                                        //     data.push(newData);
                                        //     this.setState({ data }, () => resolve());
                                        //   }
                                        resolve();
                                    }, 1000);
                                }),
                            onRowUpdate: (newData, oldData) =>
                                new Promise((resolve, reject) => {
                                    setTimeout(() => {
                                        //   {
                                        //     const data = this.state.data;
                                        //     const index = data.indexOf(oldData);
                                        //     data[index] = newData;
                                        //     this.setState({ data }, () => resolve());
                                        //   }
                                        resolve();
                                    }, 1000);
                                }),
                            onRowDelete: (oldData) =>
                                new Promise((resolve, reject) => {
                                    setTimeout(() => {
                                        //   {
                                        //     let data = this.state.data;
                                        //     const index = data.indexOf(oldData);
                                        //     data.splice(index, 1);
                                        //     this.setState({ data }, () => resolve());
                                        //   }
                                        resolve();
                                    }, 1000);
                                }),
                        }}
                    />
                    <MaterialTable
                        title='Explorers'
                        columns={[
                            { title: 'Host', field: 'host' },
                            { title: 'Chain ID', field: 'chainId' },
                        ]}
                        data={explorersState.map((explorer) => ({
                            host: explorer.host,
                            chainId: explorer.chainId,
                            raw: explorer,
                        }))}
                        editable={{
                            onRowAdd: (newData) =>
                                new Promise((resolve, reject) => {
                                    console.log(newData);
                                    resolve();
                                }),
                            onRowUpdate: (newData, oldData) =>
                                new Promise((resolve, reject) => {
                                    console.log(newData, oldData);
                                    setTimeout(() => {
                                        //   {
                                        //     const data = this.state.data;
                                        //     const index = data.indexOf(oldData);
                                        //     data[index] = newData;
                                        //     this.setState({ data }, () => resolve());
                                        //   }
                                        resolve();
                                    }, 1000);
                                }),
                            onRowDelete: (oldData) =>
                                new Promise((resolve, reject) => {
                                    setTimeout(() => {
                                        //   {
                                        //     let data = this.state.data;
                                        //     const index = data.indexOf(oldData);
                                        //     data.splice(index, 1);
                                        //     this.setState({ data }, () => resolve());
                                        //   }
                                        resolve();
                                    }, 1000);
                                }),
                        }}
                    />
                </Grid>
            </Container>
        </Site.Skeleton>
    );
}
