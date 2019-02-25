import React from 'react';
import StockTable from './StockTable.js';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import SearchIcon from '@material-ui/icons/Search';
import InputBase from '@material-ui/core/InputBase';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import '../../../App.css';

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
    this.state = {
      stock: this.props.stock,
      stockInfo: this.props.stock,
      stockName: '',
      isLoading: this.props.isLoading
    }
    this.onSearchStock = this.onSearchStock.bind(this);
    this.getStock = this.props.getStock.bind(this);
  }


  onSearchStock(e){
    this.setState({
      stockName: e.target.value,
      isLoading: true
    })
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

          <div>
            {
              (!this.state.isLoading)
                ? (this.state.stock !== undefined)
                  ? <StockTable
                      stock={this.state.stock[this.state.stockName].quote}
                    />
                  : <div></div>
                : <div></div>
            }
          </div>
      </div>
    );
  }

}

// {
//   (!this.props.isLoading)
//   ? <div style={styles.panel} className="tables">
//
//     <Paper levation={1} >
//     <div style={{float:'left', margin: 10}}>
//       <Paper style={styles.root}>
//         <h2>Owned</h2>
//         <Table style={styles.table}>
//
//           <TableHead>
//             <TableRow>
//               <TableCell>Company</TableCell>
//               <TableCell align="right">Symbol</TableCell>
//               <TableCell align="right">Price</TableCell>
//               <TableCell align="right">Owned</TableCell>
//             </TableRow>
//           </TableHead>
//           <TableBody>
//             {
//               this.props.stocks.map((stock) => (
//                 <TableRow key={stock.id}>
//                   <TableCell component="th" scope="row">
//                     {stock.companyName}
//                   </TableCell>
//                   <TableCell align="right">{stock.symbol}</TableCell>
//                   <TableCell align="right">{stock.latestPrice}</TableCell>
//                   <TableCell align="right">{0}</TableCell>
//                 </TableRow>
//               ))
//             }
//           </TableBody>
//         </Table>
//       </Paper>
//       </div>
//   : <div className="App">
//       <p>Loading Stocks...</p>
//       <Loader
//          type="Oval"
//          color="#00BFFF"
//          height="100"
//          width="100"
//       />
//     </div>
// }

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
  searchIcon: {
    width: '50%',
    height: '100%',
    position: 'absolute',
    pointerEvents: 'none',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
  }
}
