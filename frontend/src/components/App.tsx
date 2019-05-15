import React, { useEffect } from 'react';
import * as Router from 'react-router-dom';
import { useDispatch, app } from '../store';
import { getRouteTemplate, RouteType } from '../routes';
import * as Site from './Site';

const HomePage = React.lazy(() => import('./HomePage'));
const SettingsPage = React.lazy(() => import('./SettingsPage'));
const NotFoundPage = React.lazy(() => import('./NotFoundPage'));

function App() {
    const dispatch = useDispatch();
    useEffect(() => {
        dispatch(app.init());
    }, [dispatch]);
    return (
        <React.Suspense fallback={<Site.Loading />}>
            <Router.BrowserRouter>
                <Router.Switch>
                    <Router.Route
                        exact
                        path={getRouteTemplate(RouteType.Home)}
                        component={HomePage}
                    />
                    <Router.Route
                        exact
                        path={getRouteTemplate(RouteType.Settings)}
                        component={SettingsPage}
                    />
                    <Router.Route component={NotFoundPage} />
                </Router.Switch>
            </Router.BrowserRouter>
        </React.Suspense>
    );
}

export default App;
