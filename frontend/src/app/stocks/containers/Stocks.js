import React from 'react';
import StocksView from '../components/StocksView.js';

export default class Stocks extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      stock: '',
      data: '',
      isLoading: true,
      error: false
    }

  }

  getChart(stock){
    const url = `https://api.iextrading.com/1.0/stock/${stock}/chart`;
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
            this.setState({
              data: res,
              isLoading: false,
              error: false,
            });

        });

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
            });
          }
          else {
            this.setState({
              stock: res,
            });
            this.getChart(stock);
          }
        });

  }

  buyStock(stock, quantity, price){
    const url = '/api/market/stock/transact';
    const headers = {
      'Authorization: ': 'bearer token'
    }
    const body = JSON.stringify({
      "uuid": "temp",
      "user_uuid": "temp",
      "stock_uuid": "temp",
      "quantity": quantity,
      "price_of_stock_at_time_of_trading": price,
      "record_time": 'date'
    });

    return fetch(url, { method: 'POST', headers, body })
      .then((res) => res.json())
        .then((res) => {
          if (res === 200){
            alert(`Bought ${quantity}x ${stock} shares!`)
          }
          else {
            alert('There was a problem with the transaction. Try again later!')
          }
        }).catch((error) => {
          alert('There was a problem with the transaction. Try again later!')
        });

  }

  render(){
    return(
      <StocksView
        stock={this.state.stock}
        data={this.state.data}
        getStock={this.getStock}
        getChart={this.getChart}
        buyStock={this.buyStock}
        isLoading={this.state.isLoading}
        error={this.state.error}
      />
    );

  }

}
