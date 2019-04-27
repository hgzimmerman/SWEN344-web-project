import React from 'react';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import ArrowDropUp from '@material-ui/icons/ArrowDropUp';
import ArrowDropDown from '@material-ui/icons/ArrowDropDown';
import SellStockModal from '../components/SellStockModal.js';
import Button from '@material-ui/core/Button';

export default class HomeStocksTable extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      visible: false
    }

  }

  openModal(stock){
    this.setState({
      visible: true,
      stock: stock
    });

  }

  closeModal(){
    this.setState({ visible: false });

  }

  render(){
    return(
      <div className="App" id='sellStockRoot'>
      {
        this.state.visible && (
          <SellStockModal
            id='sellStockModal'
            stock={this.state.stock}
            visible={this.state.visible}
            closeModal={() => this.closeModal()}
            sellStock={this.props.sellStock}
          />
        )
      }
        <div className="tables" id='homeStockTableRoot'>
          <div style={{float:'left', margin: 10}} id='stockTableContainer'>
            <Paper style={styles.root} id='stockPaperRoot'>
              <Table style={styles.table} id='stockTable'>
                <TableHead id='stockTableHead'>
                  <TableRow>
                    <TableCell align="right" id='stockSymbolCell'>Symbol</TableCell>
                    <TableCell align="right" id='stockPriceCell'>Price</TableCell>
                    <TableCell align="right" id='emptyStockCell'></TableCell>
                  </TableRow>
                </TableHead>
                <TableBody id='stockTableBody'>
                  {this.props.stocks.map(stock => (
                    <TableRow key={stock.quote.symbol}>
                      <TableCell align="right" id=`${stock.quote.symbol}-symbol`>
                        {stock.quote.symbol}
                      </TableCell>
                      <TableCell align="right" id=`${stock.quote.symbol}-price`>
                        ${Math.round(stock.quote.open * 100) / 100}
                        {
                          (stock.quote.changePercent > 0)
                          ? <ArrowDropUp style={{color: '#45f442'}} />
                          : <ArrowDropDown style={{color: 'red'}} />
                        }
                        {stock.quote.changePercent.toFixed(3)}%
                      </TableCell>
                      <TableCell align="right" id=`${stock.quote.symbol}-buy`>
                        <Button
                          id=`${stock.quote.symbol}-buyBtn`
                          onClick={() => this.openModal(stock.quote.symbol)}
                          variant="contained"
                          style={styles.button}
                        >
                          Sell
                        </Button>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </Paper>
          </div>
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
  table: {
    minWidth: 400,
  },
  button: {
    backgroundColor: '#1C0F13',
    color: 'white',
    height: 35,
    width: 10,
  },
}
