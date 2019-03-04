import React from 'react';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import ArrowDropUp from '@material-ui/icons/ArrowDropUp';
import ArrowDropDown from '@material-ui/icons/ArrowDropDown';

export default class StocksView extends React.Component {
  render(){
    return(
      <div className="App">
        <div className="tables">
          <div style={{float:'left', margin: 10}}>
            <Paper style={styles.root}>
              <Table style={styles.table}>
                <TableHead>
                  <TableRow>
                    <TableCell align="right">Symbol</TableCell>
                    <TableCell align="right">Price</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {this.props.stocks.map(row => (
                    <TableRow key={row.id}>
                      <TableCell align="right">
                        {row.quote.symbol}
                      </TableCell>
                      <TableCell align="right">
                        ${Math.round(row.quote.open * 100) / 100}
                        {
                          (row.quote.changePercent > 0)
                          ? <ArrowDropUp style={{color: '#45f442'}} />
                          : <ArrowDropDown style={{color: 'red'}} />
                        }
                        {row.quote.changePercent.toFixed(3)}%
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
    minWidth: 340,
  },
}
