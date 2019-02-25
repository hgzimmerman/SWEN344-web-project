import React from 'react';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

const weather = {
	"temperature_fahreinheit": "12",
	"temperature_celsius": "-2"
}

export default class WeatherView extends React.Component {
  constructor(props){
    super(props);

  }
  render(){

    return(
      <div className="App">
        <Paper levation={1} >
        <div style={{float:'left', margin: 10}}>
          <Paper style={styles.root}>
            this.props.weather.temperature_fahreinheit
            <Table style={styles.table}>
        
              <TableHead>
                <TableRow>
                  <TableCell align="center">RIT Temperature (Fahrenheit)</TableCell>
                  <TableCell align="center">RIT Temperature (Celsius)</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                <TableCell>{this.props.weather.temperature_fahreinheit}</TableCell>
				<TableCell>{this.props.weather.temperature_celsius}</TableCell>
              </TableBody>
            </Table>
          </Paper>
          </div>
        </Paper>
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
