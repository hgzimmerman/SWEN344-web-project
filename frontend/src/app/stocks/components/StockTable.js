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
      <div className="App" id='stockTableRoot'>
        <div style={styles.panel} className="tables" id='stockTablePanel'>
          <Paper style={styles.root} id='stockTablePaper'>
            <h2 id='stockTableHeader'>{this.props.stock.symbol}</h2>
            <Table style={styles.table} id='stockTableTable'>
              <TableHead id='stockTableHeadRow'>
                <TableRow>
                  <TableCell>Company</TableCell>
                  <TableCell align="right">Symbol</TableCell>
                  <TableCell align="right">Current price</TableCell>
                  <TableCell align="right">Day's high</TableCell>
                  <TableCell align="right">Day's low</TableCell>
                  <TableCell align="right">Yearly high</TableCell>
                  <TableCell align="right">Yearly low</TableCell>
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
                  <TableCell align="right">
                    {this.props.stock.high}
                  </TableCell>
                  <TableCell align="right">
                    {this.props.stock.low}
                  </TableCell>
                  <TableCell align="right">
                    {this.props.stock.week52High}
                  </TableCell>
                  <TableCell align="right">
                    {this.props.stock.week52Low}
                  </TableCell>
                </TableRow>
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

}
