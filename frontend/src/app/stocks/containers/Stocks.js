import React from 'react';
import StocksView from '../components/StocksView.js';
import {authenticatedFetchDe} from "../../../config/auth";

export default class Stocks extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      stock: '', // The string being searched for
      data: '', // The data for the searched for stock.
      stocks: null, // All stocks owned by the user
      isLoading: true,
      error: false
    };
    this.transactStock = this.transactStock.bind(this);
    this.getOwnedStocks = this.getOwnedStocks.bind(this);
  }

  componentDidMount() {
    this.getOwnedStocks();
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
      console.log(typeof stock);
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

  /**
   * Buys or sells a quantity of stocks
   * @param symbol The symbol of the stock
   * @param quantity The number of shares to sell or buy (can be negative)
   */
  transactStock(symbol, quantity) {
    const url = '/api/market/stock/transact';
    const body = JSON.stringify({
      symbol,
      quantity: Number(quantity)
    });
    authenticatedFetchDe(url, {method: "POST", body})
      .then(() => {
        this.getOwnedStocks() // Update the whole stock table after performing a sale or purchase.
      })
  }

  /**
   * Gets stocks from IEX
   * @param symbols An array of strings for stock symbols.
   * @returns {*} A promise resolving to an array of iex quotes
   */
  static getStocksIEX(symbols){
    if (symbols.length > 0) {
      console.log("getting iex stock data");
      const symbolsString = symbols.join(",");
      const url = `https://api.iextrading.com/1.0/stock/market/batch?symbols=${symbolsString}&types=quote`;

      return fetch(url, {method: 'GET'})
        .then((res) => res.json())
        .then((res) => {
          let stocksArr = [];
          let i = 0;
          while (i < symbols.length) {
            stocksArr.push(res[Object.keys(res)[i]]);
            i++;
          }
          return stocksArr;
        });
    } else {
      return Promise.resolve([]);
    }
  }

  /**
   * Gets the owned stocks, and then associates the owned stock data with data from IEX
   */
  getOwnedStocks() {
    const url = '/api/market/stock/performance';
    authenticatedFetchDe(url)
      .then(backendResponse => {
        let symbols = backendResponse.map(stock => {
          return stock.stock.stock.symbol;
        });

        return Stocks.getStocksIEX(symbols)
          .then(iexResponse => {
            const combinedStocks = backendResponse.map(stock => {
              const symbol = stock.stock.stock.symbol;
              const iexStock = iexResponse.find(iexStock => iexStock.quote.symbol.toUpperCase() === symbol.toUpperCase());
              return {
                backendStock: stock.stock,
                currentPrice: stock.price,
                iexStock,
                performance: stock.performance
              }
            });
            console.log(JSON.stringify(combinedStocks));
            this.setState({stocks: combinedStocks})
          });
      })
  }

  render(){
    return(
      <StocksView
        stock={this.state.stock}
        stocks={this.state.stocks}
        data={this.state.data}
        getStock={this.getStock}
        getChart={this.getChart}
        transactStock={this.transactStock}
        isLoading={this.state.isLoading}
        error={this.state.error}
      />
    );

  }

}
