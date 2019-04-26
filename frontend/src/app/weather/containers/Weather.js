import React from 'react';
import WeatherView from '../components/WeatherView.js';

export default class Weather extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      weather: '',
      isLoading: true,
      zipCode: '',
      error: false
    }
    this.getWeather = this.getWeather.bind(this);
  }

  getWeather(zipCode) {
    this.setState({
      isLoading: true
    });
    const url = `https://api.openweathermap.org/data/2.5/weather?zip=${zipCode},us&APPID=4c442afc1ade3bc91a9bb48fb1fd0e02&units=imperial`
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          if (Object.entries(res).length === 0) {
            console.log("Error");
            this.setState({
              error: true,
              isLoading: false
            });
          }
          else {
            console.log("Weather retrieved " + res);
            console.log(res.weather);
            this.setState({
              weather: res,
              isLoading: false
            });
          }
        });
  }

  render() {
    return(
      <WeatherView
        isLoading={this.state.isLoading}
        weather={this.state.weather}
        error={this.state.error}
        getWeather={this.getWeather}
      />
    );
  }

}
