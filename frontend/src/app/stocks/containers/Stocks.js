import React from 'react';
import StocksView from '../components/StocksView.js';
import Loader from 'react-loader-spinner';

export default class Stocks extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      stock: '',
      isLoading: true
    }

  }

  // componentDidMount(){
  //   this.getStocks();
  //
  // }

  getStock(stock){
    const url = `https://api.iextrading.com/1.0/stock/market/batch?symbols=${stock}&types=quote`;

    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          this.setState({
            stock: res,
            isLoading: false
          });
          console.log(res[stock].quote.companyName);
        })

  }

  render(){
    return(
        <StocksView
            stock={this.state.stock}
            getStock={this.getStock}
            isLoading={this.state.isLoading}
          />



    );

  }

}
