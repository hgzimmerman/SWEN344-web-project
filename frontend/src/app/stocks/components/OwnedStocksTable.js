import React from 'react';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

export default class OwnedStockSTable extends React.Component {
  
  renderRow(aggregateStock) {
    return (
      <TableRow>
        <TableCell component="th" scope="row">
          {aggregateStock.iexStock.quote.companyName}
        </TableCell>
        <TableCell align="right">
          {aggregateStock.iexStock.quote.symbol}
        </TableCell>
        <TableCell align="right">
          {aggregateStock.backendStock.transactions.reduce((acc, cur) => {return acc + cur.quantity;}, 0) /*Sum the transactions*/}
        </TableCell>
        <TableCell align="right">
          {aggregateStock.performance}
        </TableCell>
        <TableCell align="right">
          {aggregateStock.iexStock.quote.latestPrice}
        </TableCell>
        <TableCell align="right">
          {aggregateStock.iexStock.quote.high}
        </TableCell>
        <TableCell align="right">
          {aggregateStock.iexStock.quote.low}
        </TableCell>
        <TableCell align="right">
          {aggregateStock.iexStock.quote.week52High}
        </TableCell>
        <TableCell align="right">
          {aggregateStock.iexStock.quote.week52Low}
        </TableCell>
        <TableCell align="right">
          Buy
        </TableCell>
        <TableCell align="right">
          Sell
        </TableCell>
      </TableRow>
    )
  }
  
  render(){
    return(
      <div className="App" id='StocksTable'>
        <div style={styles.panel} className="tables">
          <Paper style={styles.root}>
            <Table style={styles.table}>
              <TableHead>
                <TableRow>
                  <TableCell>Company</TableCell>
                  <TableCell align="right">Symbol</TableCell>
                  <TableCell align="right">Quantity Owned</TableCell>
                  <TableCell align="right">Net Gain/Loss</TableCell>
                  <TableCell align="right">Current Price</TableCell>
                  <TableCell align="right">Day's High</TableCell>
                  <TableCell align="right">Day's Low</TableCell>
                  <TableCell align="right">Yearly High</TableCell>
                  <TableCell align="right">Yearly Low</TableCell>
                  <TableCell align="right">Buy</TableCell>
                  <TableCell align="right">Sell</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {
                  (this.props.stocks !== null && this.props.stocks !== undefined )
                    ? this.props.stocks.map(aggregateStock => {
                      return this.renderRow(aggregateStock)
                    })
                    : <></>
                }
              </TableBody>
            </Table>
          </Paper>
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
    minWidth: 700,
  },
  panel: {
    padding: 30
  }
};