import React from 'react';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

const rows = {
  Items: [
    {
      "id": "1",
      "name": "Apple",
      "symbol": "AAPL",
      "price": "170.92",
      "owned": "2",
      "change": "+0.5%"
    },
    {
      "id": "2",
      "name": "Neopets",
      "symbol": "NEO",
      "price": "1970.92",
      "owned": "99",
      "change": "+99.5%"
    }
  ]
}

export default class StocksView extends React.Component {
  constructor(props){
    super(props);

  }


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
                  {rows.Items.map(row => (
                    <TableRow key={row.id}>
                      <TableCell align="right">{row.symbol}</TableCell>
                      <TableCell align="right">{row.price}</TableCell>
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
