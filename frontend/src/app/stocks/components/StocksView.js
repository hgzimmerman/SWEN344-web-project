import React from 'react';
import StockTable from './StockTable.js';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import StockChart from './StockChart.js';
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
      error: this.props.error
    }
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

  render(){

    return(
      <div className="App">
        <div style={styles.searchBar}>
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
          onClick={() => this.getStock(this.state.stockName)}
          variant="contained"
          style={styles.button}
        >
          Search
        </Button>
        <div style={{textAlign: 'center'}} >

        </div>
        <div>
          {
            (!this.state.isLoading)
              ? (this.state.error)
                ? <p>
                    Stock
                    <span style={styles.bold}> "{this.state.stockName}" </span>
                    does not exist
                  </p>
                : (this.state.stock !== undefined)
                  ? <div>
                      <StockTable
                        stock={this.state.stock[this.state.stockName.toUpperCase()].quote}
                      />
                      <StockChart
                        stock={this.state.stockName.toUpperCase()}
                        data={this.state.data}
                      />
                    </div>
                  : <div></div>
              : <div></div>

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
    width: 150
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
