import React, { Component } from 'react';
import { Switch, Route } from 'react-router-dom';
import Home from './app/home/components/Home.js';
import Login from './app/auth/components/Login.js';
import './App.css';

const Main = () => (
  <main>
    <Switch>
      <Route exact path='/' component={Home}/>
      <Route exact path='/login' component={Login}/>
    </Switch>
  </main>
);

class App extends Component {
  render() {
    return (
      <Main />
    );
  }
}

export default App;
