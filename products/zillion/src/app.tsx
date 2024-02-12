import React from 'react';
import { Redirect, Route, Switch } from 'react-router-dom';
import Home from './components/home';
import Dashboard from './components/dashboard';
import Error404 from './components/error404';
import Explorer from './components/explorer';


function App() {
  return (
    <Switch>
      <Route exact path="/" render={(props) => <Home {...props} />} />
      <Route exact path="/address/:address" render={(props) => < Explorer {...props} />} />
      <Route exact path="/dashboard" render={(props) => <Dashboard {...props}/>} />
      <Route exact path="/oops" render={(props) => <Error404 {...props}/>} />
      <Route>
        {/* No route match - redirect to home page */}
        <Redirect to="/" />
      </Route>
    </Switch>
  );
}

export default App;
