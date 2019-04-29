import React from 'react';
import HomeView from '../components/HomeView.js';

export default class Home extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      feed:[],
      post: '',
      weather: null,
      isLoading: true
    };
    this.getWeather = this.getWeather.bind(this);

  }

  componentDidMount(){
    this.getWeather().then(() => this.getStocks()).then(() => this.getFeed());

  }

  getFeed() {
    const jwt = localStorage.getItem("jwt");
    const url = `api/twitter_proxy/feed`;

    return fetch(url,
      {
        method: 'GET',
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
          'jwt': jwt
        }
      }
    )
    .then((res) => res.json())
        .then((res) => {
          this.setState({
            feed: JSON.parse(res.feed_response)
          });
        }
      );
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



  render(){
    return(
      <HomeView
        feed={this.state.feed}
        post={this.state.post}
        postFeed={this.postFeed}
        isLoading={this.state.isLoading}
        weather={this.state.weather}
        stocks={this.state.stocks}
        sellStock={this.sellStock}
      />
    );

  }

}
