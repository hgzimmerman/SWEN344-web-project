import React from 'react';
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
    this.getWeather().then(() => this.getStocks()).then(() => this.getSelfAdaptiveAd());
  }

  getWeather(){
    const url = 'https://api.openweathermap.org/data/2.5/weather?zip=14623,us&APPID=4c442afc1ade3bc91a9bb48fb1fd0e02&units=imperial';

    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          this.setState({
            weather: res,
          });
        });

  }

  getStocks(){
    const url = `https://api.iextrading.com/1.0/stock/market/batch?symbols=amzn,aapl,googl,fb&types=quote`;
    let stocksArr = [];
    let i = 0;

    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          while(i < 4){
            stocksArr.push(res[Object.keys(res)[i]]);
            i++;
          }
          this.setState({
            stocks: stocksArr,
            isLoading: false
          });

        });

  }

  getSelfAdaptiveAd(){
    const url = 'https://vm344c.se.rit.edu/api/advertisement';

    return fetch(url, { method: 'GET'})
      .then((res) => res.blob())
        .then((res) => {
          this.setState({
            ad: res,
            isLoading: true
          })
        })
  }

  sellStock(stock, quantity, price){
    let owned = 3;
    if (quantity === ''){
      alert("Can't sell zero shares!")
    }
    else if (quantity > owned){
      alert("Can't sell more shares than you own!")
    }
    else {
      alert(`Sold ${quantity}x ${stock} shares!`)
    }

  }

  render(){
    return(
      <HomeView
        isLoading={this.state.isLoading}
        weather={this.state.weather}
        stocks={this.state.stocks}
        sellStock={this.sellStock}
        ad={this.state.ad}
      />
    );

  }

}
