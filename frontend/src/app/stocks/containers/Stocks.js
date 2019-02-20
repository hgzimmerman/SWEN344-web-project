import React from 'react';
import StocksView from '../components/StocksView.js';

export default class Stocks extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      stocks: null,
      isLoading: true
    }
    this.getStocks = this.getStocks.bind(this);

  }

  componentDidMount(){
    this.getStocks();

  }

  getStocks(){
    const url = 'https://api.iextrading.com/1.0/stock/market/list/gainers'
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          this.setState({
            stocks: res,
            isLoading: false
          });
          console.log(res)
        })

  }

  render(){
    return(
      (!this.state.isLoading)
      ? <StocksView
          stocks={this.state.stocks}
        />
      : <p>Loading Stocks...</p>


    );

  }

}
