import React from 'react';
import WeatherTable from '../components/WeatherTable.js';
import {authenticatedFetchDe} from "../../../config/auth";

export default class Weather extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      weather: null,
      isLoading: true,
      zipCode: '14623',
      error: false
    };
    this.getWeather = this.getWeather.bind(this);
  }

  componentDidMount() {
    this.getZipCode().then(() => this.getWeather(this.state.zipCode));
  }

  getZipCode() {
    const url = "/api/user/zip";
    return authenticatedFetchDe(url)
      .then(response => {
        const defaultZipCode = "14623";
        if (response !== null && response !== undefined) {
          this.setState({zipCode: response})
        } else {
          this.setState({zipCode: defaultZipCode})
        }
      });
  }

  getWeather(zipCode) {
    this.setState({
      isLoading: true
    });
    const url = `https://api.openweathermap.org/data/2.5/weather?zip=${zipCode},us&APPID=4c442afc1ade3bc91a9bb48fb1fd0e02&units=imperial`;
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          if (Object.entries(res).length === 0) {
            console.log("Error, could not get weather");
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
      <WeatherTable
        isLoading={this.state.isLoading}
        weather={this.state.weather}
        error={this.state.error}
        getWeather={this.getWeather}
      />
    );
  }

}
