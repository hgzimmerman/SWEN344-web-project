import React, { Component } from 'react';
import { Switch, Route } from 'react-router-dom';
<<<<<<< HEAD
import CustomNavbar from './app/layout/components/CustomNavbar.js';
import Home from './app/home/components/Home.js';
import CalendarView from './app/calendar/components/CalendarView.js';
import Stocks from './app/stocks/containers/Stocks.js';
=======
import Home from './app/home/components/Home.js';
>>>>>>> 8260ea595b66949fe44859d17a7afb5ae07d1fa8
import Login from './app/auth/components/Login.js';
import './App.css';

const Main = () => (
  <main>
<<<<<<< HEAD
    <CustomNavbar />
    <Switch>
      <Route exact path='/' component={Home}/>
      <Route exact path='/login' component={Login}/>
      <Route exact path='/calendar' component={CalendarView}/>
      <Route exact path='/stocks' component={Stocks}/>
=======
    <Switch>
      <Route exact path='/' component={Home}/>
      <Route exact path='/login' component={Login}/>
>>>>>>> 8260ea595b66949fe44859d17a7afb5ae07d1fa8
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
