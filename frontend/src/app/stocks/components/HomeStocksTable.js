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
      visible: false,
      stock: null
    }

  }

  render(){
    return(
      <Paper style={styles.root} id={"StocksCard"}>
        <Table style={styles.table}>
          <TableHead>
            <TableRow>
              <TableCell align="right">Symbol</TableCell>
              <TableCell align="right">Price</TableCell>
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

              </TableRow>
            ))}
          </TableBody>
        </Table>
        {
          (this.props.stocks.length === 0)
            ? <span> No owned stocks </span>
            : <></>
        }

      </Paper>
    )
  }

}

const styles = {
  root: {
    flexGrow: 2,
    width: '100%',
    overflowX: 'auto',
    minWidth: 400,
    minHeight: 300,
    marginTop: 20
  },
  table: {
    minWidth: 400,
    minHeight: 300
  },
  button: {
    backgroundColor: '#1C0F13',
    color: 'white',
    height: 35,
    width: 10,
  },
};
