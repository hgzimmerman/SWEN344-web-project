import React from 'react';
import HomeView from '../components/HomeView.js';
import {authenticatedFetchDe} from "../../../config/auth";

export default class Home extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      weather: null,
      zipCode: '14623', // default to RIT
      isLoading: true,
      stocks: null,
      backendStocks: null,
      events: null
    };
    this.getWeather = this.getWeather.bind(this);
    this.getZipCode = this.getZipCode.bind(this);
    this.getBackendStocks = this.getBackendStocks.bind(this);
    this.getStocks = this.getStocks.bind(this);
    this.finalizeLoading = this.finalizeLoading.bind(this);
  }

  componentDidMount(){
    Promise.all([
      this.getZipCode().then(zipCode => this.getWeather(zipCode)),
      this.getBackendStocks().then(stocks => this.getStocks(stocks)),
      this.getEventsToday()
    ])
      .then(() => this.finalizeLoading())
  }

  finalizeLoading() {
    this.setState({isLoading: false})
  }
   /**
   * Gets the zip code from the backend.
   * Will default to the rochester zip code if the user hasn't set their zip code yet.
   */
  getZipCode() {
    const defaultZipCode = "14623"; // RIT's zip
    const url = "/api/user/zip";
    return authenticatedFetchDe(url)
      .then(response => {
        if (response !== null && response !== undefined) {
          this.setState({zipCode: response})
        } else {
          this.setState({zipCode: defaultZipCode})
        }
        return response
      })
      .catch(() => {
          console.error("Couldn't get zip code, defaulting to default zip code");
          this.setState({zipCode: defaultZipCode})
        }
      )
  }

  getWeather(zipCode){
    const url = `https://api.openweathermap.org/data/2.5/weather?zip=${zipCode},us&APPID=4c442afc1ade3bc91a9bb48fb1fd0e02&units=imperial`;
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
      .then((res) => {
        this.setState({
          weather: res,
        });
        return null;
      })
      .catch(() => {
        console.error(`Could not find weather for zip code: ${zipCode}`);
      })
  }

  getEventsToday() {
    let now = new Date();
    let midnight = new Date();
    midnight.setHours(24,0,0,0);
    const url = `api/calendar/event/events?start=${encodeURIComponent(now.toISOString())}&stop=${encodeURIComponent(midnight.toISOString())}`;
    return authenticatedFetchDe(url)
      .then(eventsResponse => {
        this.setState({
          events: eventsResponse
        })
      })
  }

  /**
   * Gets four stocks you own.
   */
  getBackendStocks() {
    const url = "/api/market/stock";
    return authenticatedFetchDe(url)
      .then(stocks => {
        let limitedStocks = stocks.slice(0, 4);
        this.setState({backendStocks: limitedStocks});
        return limitedStocks;
      });
  }

  getStocks(stocks){
    if (stocks.length > 0) {
      console.log("getting iex stock data");
      const stockSymbols = stocks.map(stock => {
        return stock.stock.symbol;
      });
      const symbolsString = stockSymbols.join(",");
      const url = `https://api.iextrading.com/1.0/stock/market/batch?symbols=${symbolsString}&types=quote`;
      console.log(`stock fetching url: ${url}`);


      return fetch(url, {method: 'GET'})
        .then((res) => res.json())
        .then((res) => {
          let stocksArr = [];
          let i = 0;
          while (i < stockSymbols.length) {
            stocksArr.push(res[Object.keys(res)[i]]);
            i++;
          }
          this.setState({
            stocks: stocksArr,
          });
          return null;
        });
    } else {
      this.setState({
          stocks: []
      })
    }
  }


  render(){
    return(
      <HomeView
        feed={this.state.feed}
        isLoading={this.state.isLoading}
        weather={this.state.weather}
        weatherError={this.state.weatherError}
        zipCode={this.state.zipCode}
        stocks={this.state.stocks}
        events={this.state.events}
      />
    );

  }

}
