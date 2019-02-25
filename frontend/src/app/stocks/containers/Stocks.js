import React from 'react';
import StocksView from '../components/StocksView.js';

export default class Stocks extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      stock: '',
      isLoading: true,
      error: false
    }

  }

  getStock(stock){
    const url = `https://api.iextrading.com/1.0/stock/market/batch?symbols=${stock}&types=quote`;
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          if (Object.entries(res).length === 0){
            this.setState({
              error: true,
              isLoading: false
            })

          }
          else {
            this.setState({
              stock: res,
              isLoading: false,
              error: false,
            });
          }
        });

  }

  render(){
    return(
      <StocksView
        stock={this.state.stock}
        getStock={this.getStock}
        isLoading={this.state.isLoading}
        error={this.state.error}
      />
    );

  }

}
