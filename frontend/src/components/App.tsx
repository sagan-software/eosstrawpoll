import React, { useEffect } from 'react';
import * as Router from 'react-router-dom';
import * as Route from '../route';
import * as Site from './Site';

const HomePage = React.lazy(() => import('./HomePage'));
const NotFoundPage = React.lazy(() => import('./NotFoundPage'));

function App() {
    return (
        <React.Suspense fallback={<Site.Loading />}>
            <Router.BrowserRouter>
                <Router.Switch>
                    <Router.Route
                        exact
                        path={Route.getRouteTemplate(Route.Type.Home)}
                        component={HomePage}
                    />
                    <Router.Route component={NotFoundPage} />
                </Router.Switch>
            </Router.BrowserRouter>
        </React.Suspense>
    );
}

export default App;
