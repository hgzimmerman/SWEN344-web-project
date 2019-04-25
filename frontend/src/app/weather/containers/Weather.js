import React from 'react';
import WeatherTable from '../components/WeatherTable.js';

export default class Weather extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      weather: null,
      isLoading: true,
      error: false
    }
    this.getWeather = this.getWeather.bind(this);
  }

  getWeather(zipCode){
    const url = `https://api.openweathermap.org/data/2.5/weather?zip=${zipCode},us&APPID=4c442afc1ade3bc91a9bb48fb1fd0e02&units=imperial`
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          if (Object.entries(res).length === 0) {
            this.setState({
              error: true,
              isLoading: false
            });
          }
          else {
            this.setState({
              weather: res,
              isLoading: false
            });
          }
        });

  }

  render(){
    return(
      <WeatherTable
        isLoading={this.state.isLoading}
        weather={this.state.weather}
      />
    );

  }

}
