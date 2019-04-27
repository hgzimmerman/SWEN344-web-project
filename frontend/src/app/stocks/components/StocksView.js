import React from 'react';
import StockTable from './StockTable.js';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import StockChart from './StockChart.js';
import BuyStockModal from './BuyStockModal.js';
import '../../../App.css';

export default class StocksView extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      stock: this.props.stock,
      stockInfo: this.props.stock,
      stockName: '',
      data: this.props.data,
      isLoading: this.props.isLoading,
      error: this.props.error,
      visible: false
    };
    this.onSearchStock = this.onSearchStock.bind(this);
    this.getStock = this.props.getStock.bind(this);
    this.getChart = this.props.getChart.bind(this);

  }

  onSearchStock(e){
    this.setState({
      stockName: e.target.value,
      isLoading: true
    });

  }

  openModal(){
    this.setState({ visible: true })
  }

  closeModal(){
    this.setState({ visible: false })

  }

  render(){

    return(
      <div className="App" id='stocksViewRoot'>
        <div style={styles.searchBar} id='stocksViewSearch'>
          <TextField
            id="outlined-with-placeholder"
            label="Search"
            placeholder="Stock Symbol…"
            margin="normal"
            variant="outlined"
            onChange={this.onSearchStock}
            style={{width: '50%'}}
          />
        </div>

        <Button
          id='stocksViewSearchBtn'
          onClick={() => this.getStock(this.state.stockName)}
          variant="contained"
          style={styles.button}
        >
          Search
        </Button>
        <div style={{textAlign: 'center'}} >

        </div>
        <div id='stocksViewContainer'>
          {
            (!this.state.isLoading)
              ? (this.state.error)
                ? <p id='stocksViewTitle'>
                    Stock
                    <span style={styles.bold}> "{this.state.stockName}" </span>
                    does not exist
                  </p>
                : (this.state.stock !== undefined)
                  ? <div>
                      <StockTable
                        id='stockTable'
                        stock={this.state.stock[this.state.stockName.toUpperCase()].quote}
                      />
                      <Button
                        onClick={() => this.openModal()}
                        variant="contained"
                        style={styles.buyButton}
                      >
                        {`Buy ${this.state.stockName} shares`}
                      </Button>
                      <StockChart
                        stock={this.state.stockName.toUpperCase()}
                        data={this.state.data}
                      />
                      {this.state.visible && (
                        <BuyStockModal
                          visible={this.state.visible}
                          stock={this.state.stockName.toUpperCase()}
                          price={this.state.stock[this.state.stockName.toUpperCase()].quote.latestPrice}
                          transactStock={this.props.transactStock}
                          closeModal={() => this.closeModal()}
                        />
                      )}
                    </div>
                  : <></>
              : <></>

          }
        </div>
      </div>
    );

  }

}

const styles = {
  root: {
    width: '100%',
    marginTop: 20,
    overflowX: 'auto',
  },
  button: {
    backgroundColor: '#00A6DD',
    color: 'white',
    height: 50,
    width: 200
  },
  buyButton: {
    backgroundColor: '#1C0F13',
    color: 'white',
    height: 50,
    width: 200,
    marginBottom: 20
  },
  table: {
    minWidth: 700,
  },
  panel: {
    padding: 30
  },
  searchBar: {
    marginTop: 40,
    marginBottom: 10
  },
  bold: {
    fontWeight: 'bold'
  }

}
