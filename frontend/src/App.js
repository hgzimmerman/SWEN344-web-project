import React, { Component } from 'react';
import { Switch, Route } from 'react-router-dom';
import CustomNavbar from './app/layout/components/CustomNavbar.js';
import Home from './app/home/components/Home.js';
import CalendarView from './app/calendar/components/CalendarView.js';
import StocksView from './app/stocks/components/StocksView.js';
import Login from './app/auth/components/Login.js';
import './App.css';

const Main = () => (
  <main>
    <CustomNavbar />
    <Switch>
      <Route exact path='/' component={Home}/>
      <Route exact path='/login' component={Login}/>
      <Route exact path='/calendar' component={CalendarView}/>
      <Route exact path='/stocks' component={StocksView}/>
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
