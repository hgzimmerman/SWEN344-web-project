import React, { Component } from 'react';
import { Switch, Route } from 'react-router-dom';
import CustomNavbar from './app/layout/components/CustomNavbar.js';
import Home from './app/home/components/Home.js';
import CalendarView from './app/calendar/components/CalendarView.js';
import Stocks from './app/stocks/containers/Stocks.js';
import Login from './app/auth/components/Login.js';
import Weather from './app/weather/containers/Weather.js';
import './App.css';

const Main = () => (
  <main>
    <CustomNavbar />
    <Switch>
      <Route exact path='/' component={Home}/>
      <Route exact path='/login' component={Login}/>
      <Route exact path='/calendar' component={CalendarView}/>
      <Route exact path='/stocks' component={Stocks}/>
      <Route exact path='/weather' component={Weather}/>
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
