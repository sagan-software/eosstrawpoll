import CssBaseline from '@material-ui/core/CssBaseline';
import { ThemeProvider } from '@material-ui/styles';
import React from 'react';
import Helmet from 'react-helmet';
import * as Store from '../store';
import { theme } from '../styles';
import App from './App';

const store = Store.makeStore();

function Root() {
    return (
        <Store.StoreContext.Provider value={store}>
            <Helmet titleTemplate='%s — weos.fund'>
                <link
                    href='https://fonts.googleapis.com/css?family=IBM+Plex+Sans+Condensed:700|IBM+Plex+Sans:400,700'
                    rel='stylesheet'
                />
            </Helmet>
            <ThemeProvider theme={theme}>
                <CssBaseline />
                <App />
            </ThemeProvider>
        </Store.StoreContext.Provider>
    );
}

export default Root;
