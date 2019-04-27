import React from 'react';
import HomeView from '../components/HomeView.js';
import {authenticatedFetchDe} from "../../../config/auth";

export default class Home extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      feed:[],
      post: '',
      weather: null,
      zipCode: '14623',
      isLoading: true
    };
    this.getWeather = this.getWeather.bind(this);

  }

  componentDidMount(){
    this.getZipCode().then(zipCode => this.getWeather(zipCode))
      .then(() => this.getStocks());
  }

   /**
   * Gets the zip code from the backend.
   * Will default to the rochester zip code if the user hasn't set their zip code yet.
   */
  getZipCode() {
    const defaultZipCode = "14623";
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


  postFeed() {
    if (this.state.post !== "") {
      const jwt = localStorage.getItem("jwt");
      const url = `api/twitter_proxy/tweet/`;
      return fetch(url,
        {
          method: 'POST',
          headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            jwt: jwt,
            text: this.state.post
          })
        })
        .then((res) => res.json())
          .then((res) =>
            {
              if (res === 200) {
                alert("Tweet successfully posted");
                this.setState({
                  isLoading: false,
                  error: false
                });
              } else {
                alert("There was an error posting your tweet");
                this.setState({
                    isLoading: false,
                    error: true
                })
              }
            }
        );
    } else {
      alert('You most enter text to post');
    }
  }


  render(){
    return(
      <HomeView
        id='homeComponent'
        feed={this.state.feed}
        post={this.state.post}
        postFeed={this.postFeed}
        isLoading={this.state.isLoading}
        weather={this.state.weather}
        zipCode={this.state.zipCode}
        stocks={this.state.stocks}
        sellStock={this.sellStock}
      />
    );

  }

}
