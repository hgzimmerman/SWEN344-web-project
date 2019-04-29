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
    if (stock){
      console.log(typeof stock)
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
    else {
      alert('Please enter a valid stock')
    }


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
