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
      <div className="App">
      {
        this.state.visible && (
          <SellStockModal
            stock={this.state.stock}
            visible={this.state.visible}
            closeModal={() => this.closeModal()}
            sellStock={this.props.sellStock}
          />
        )
      }
        <div className="tables">
          <div style={{float:'left', margin: 10}}>
            <Paper style={styles.root}>
              <Table style={styles.table}>
                <TableHead>
                  <TableRow>
                    <TableCell align="right">Symbol</TableCell>
                    <TableCell align="right">Price</TableCell>
                    <TableCell align="right"></TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {this.props.stocks.map(stock => (
                    <TableRow key={stock.quote.symbol}>
                      <TableCell align="right">
                        {stock.quote.symbol}
                      </TableCell>
                      <TableCell align="right">
                        ${Math.round(stock.quote.open * 100) / 100}
                        {
                          (stock.quote.changePercent > 0)
                          ? <ArrowDropUp style={{color: '#45f442'}} />
                          : <ArrowDropDown style={{color: 'red'}} />
                        }
                        {stock.quote.changePercent.toFixed(3)}%
                      </TableCell>
                      <TableCell align="right">
                        <Button
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
