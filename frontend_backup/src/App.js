import React, { Component } from 'react';
import {
  Switch,
  Route,
} from 'react-router-dom';
import CustomNavbar from './app/layout/components/CustomNavbar.js';
import Home from './app/home/containers/Home.js';
import CalendarView from './app/calendar/components/CalendarView.js';
import Stocks from './app/stocks/containers/Stocks.js';
import Login from './app/auth/components/Login.js';
import Weather from './app/weather/containers/Weather.js';
import { PrivateRoute } from '../src/config/auth.js';
import './App.css';

const Main = () => (
  <main>
    <CustomNavbar />
    <Switch>
      <Route exact path='/login' component={Login}/>
      <PrivateRoute exact path='/' component={Home}/>
      <PrivateRoute exact path='/calendar' component={CalendarView}/>
      <PrivateRoute exact path='/stocks' component={Stocks}/>
      <PrivateRoute exact path='/weather' component={Weather}/>
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
