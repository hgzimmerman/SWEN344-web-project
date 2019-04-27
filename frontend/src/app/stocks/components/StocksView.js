import React from 'react';
import StockTable from './StockTable.js';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import StockChart from './StockChart.js';
import BuyStockModal from './BuyStockModal.js';
import '../../../App.css';
import OwnedStocksTable from "./OwnedStocksTable";
import SellStockModal from "./SellStockModal";

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
      buyStockModalVisible: false,
      modalStockName: '',
      sellStockModalVisible: false,
    };
    this.onSearchStock = this.onSearchStock.bind(this);
    this.getStock = this.props.getStock.bind(this);
    this.getChart = this.props.getChart.bind(this);

    this.openBuyModal = this.openBuyModal.bind(this);
    this.closeBuyModal = this.closeBuyModal.bind(this);

    this.openSellModal = this.openSellModal.bind(this);
    this.closeSellModal = this.closeSellModal.bind(this);
  }

  onSearchStock(e){
    this.setState({
      stockName: e.target.value,
      isLoading: true
    });

  }

  openBuyModal(stockName){
    console.log("Opening buy modal");
    this.setState({
      buyStockModalVisible: true,
      modalStockName: stockName
    });
    this.forceUpdate();
    console.log(JSON.stringify(this.state.buyStockModalVisible));
  }

  closeBuyModal(){
    this.setState({ buyStockModalVisible: false })
  }


  openSellModal(stockName){
    console.log("Opening sell modal");
    this.setState({
      sellStockModalVisible: true,
      modalStockName: stockName
    })
  }

  closeSellModal(){
    this.setState({ sellStockModalVisible: false })
  }

  renderStockSearch() {
    return (
      <>
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
                        onClick={() => this.openBuyModal(this.state.stockName)}
                        variant="contained"
                        style={styles.buyButton}
                      >
                        {`Buy ${this.state.stockName} shares`}
                      </Button>
                      <StockChart
                        stock={this.state.stockName.toUpperCase()}
                        data={this.state.data}
                      />

                    </div>
                  : <></>
              : <></>
            }
        </div>
      </>

    )
  }

  renderModals() {
    return(
      <>
        <BuyStockModal
          visible={this.state.buyStockModalVisible}
          stock={this.state.modalStockName}
          transactStock={this.props.transactStock}
          closeModal={this.closeBuyModal}
        />

        <SellStockModal
          visible={this.state.sellStockModalVisible}
          stock={this.state.modalStockName}
          transactStock={this.props.transactStock}
          closeModal={this.closeSellModal}
        />
      </>
    )
  }

  renderOwnedStocks() {
    return (
      <>
        <h3> Owned Stocks</h3>
        <OwnedStocksTable
          stocks={this.props.stocks}
          openBuyModal={this.openBuyModal}
          openSellModal={this.openSellModal}
        />
      </>
    )
  }

  render() {
    return(
      <div className="App">
        <div style={styles.container}>
          <div style={styles.segment}>
            {this.renderOwnedStocks()}
          </div>
          {this.renderModals()}
          <div style={styles.segment}>
            {this.renderStockSearch()}
          </div>
        </div>
      </div>
    );
  }
}

const styles = {
  container: {
    display: "flex",
    flexDirection: "column"
  },
  segment: {
    flexGrow: 1
  },
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

};
