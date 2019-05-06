import React from 'react';
import { render } from 'react-dom';
import Root from './components/Root';
import * as serviceWorker from './serviceWorker';

render(<Root />, document.getElementById('root'));

if (process.env.NODE_ENV === 'production') {
    serviceWorker.register();
} else {
    serviceWorker.unregister();
}
