import React from 'react';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

export default class StockTable extends React.Component {
  render(){
    return(
      <div className="App">
        <div style={styles.panel} className="tables">
          <Paper elevation={1} >
          <div>
            <Paper style={styles.root}>
              <h2>{this.props.stock.symbol}</h2>
              <Table style={styles.table}>
                <TableHead>
                  <TableRow>
                    <TableCell>Company</TableCell>
                    <TableCell align="right">Symbol</TableCell>
                    <TableCell align="right">Price</TableCell>
                    <TableCell align="right">Owned</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  <TableRow>
                    <TableCell component="th" scope="row">
                      {this.props.stock.companyName}
                    </TableCell>
                    <TableCell align="right">
                      {this.props.stock.symbol}
                    </TableCell>
                    <TableCell align="right">
                      {this.props.stock.latestPrice}
                    </TableCell>
                    <TableCell align="right">{0}</TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </Paper>
            </div>

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

}
