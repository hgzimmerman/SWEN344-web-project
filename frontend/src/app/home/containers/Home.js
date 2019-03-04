import React from 'react';
import Loader from 'react-loader-spinner';
import HomeView from '../components/HomeView.js';

export default class Home extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      weather: null,
      isLoading: true
    }
    this.getWeather = this.getWeather.bind(this);
  }

  componentDidMount(){
    this.getWeather();
  }

  getWeather(){
    const url = 'http://api.openweathermap.org/data/2.5/weather?zip=14623,us&APPID=4c442afc1ade3bc91a9bb48fb1fd0e02&units=imperial'
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          this.setState({
            weather: res,
            isLoading: false
          });
        });

  }

  getStocks(){
    const url = 'http://api.openweathermap.org/data/2.5/weather?zip=14623,us&APPID=4c442afc1ade3bc91a9bb48fb1fd0e02&units=imperial'
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          this.setState({
            weather: res,
          });
        });

  }

  render(){
    return(
      <HomeView
        isLoading={this.state.isLoading}
        weather={this.state.weather}
      />
    );
  }

}
