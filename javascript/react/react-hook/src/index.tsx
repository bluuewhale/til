import React from 'react';
import ReactDOM from 'react-dom';
import UseInput from './UseInput';
import UseTab from './UseTab';
import UseEffect from './UseEffect';

ReactDOM.render(
  <React.StrictMode>
    <UseInput />
    <UseTab />
    <UseEffect />
  </React.StrictMode>,
  document.getElementById('root')
);
