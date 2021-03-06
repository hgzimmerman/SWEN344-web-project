import React from 'react';
import WeatherView from '../components/WeatherView.js';

export default class Weather extends React.Component {
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
    const url = 'https://api.openweathermap.org/data/2.5/weather?zip=14623,us&APPID=4c442afc1ade3bc91a9bb48fb1fd0e02&units=imperial'
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          this.setState({
            weather: res,
            isLoading: false
          });
        });

  }

  render(){
    return(
      <WeatherView
        isLoading={this.state.isLoading}
        weather={this.state.weather}
      />
    );

  }

}
