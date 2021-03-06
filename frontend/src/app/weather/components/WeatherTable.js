import React from 'react';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import Loader from 'react-loader-spinner';

/**
 * A pure component that just displays weather info.
 */
export default class WeatherTable extends React.Component {
  // Parses precipitation for the day's weather.
  getPrecip() {
    console.log("Weather Table data: " + this.props.weather);
    let prec = '';
    let i;
    for (i = 0; i < this.props.weather.weather.length; i++) {
			if (i === this.props.weather.weather.length-1) {
				prec += this.props.weather.weather[i].main;
			}
			else {
				prec += (this.props.weather.weather[i].main + ', ');
			}
		}
		return prec;
	}

  render(){
    return (
      (this.props.weather !== null && this.props.weather !== undefined)
        ? <div id='weatherRoot'>
            <div style={{float:'center', margin: 10}} className="tables" id='weatherContainer'>
                <Paper style={styles.root}>
                  <h2 id='weatherZipCode'>{this.props.zipCode}</h2>
                  <Table style={styles.table} id='weatherTableTable'>
                    <TableHead id='weatherTableHeader'>
                      <TableRow>
                        <TableCell align="center">Current Temp</TableCell>
                        <TableCell align="center">High Temp</TableCell>
                        <TableCell align="center">Low Temp</TableCell>
                        <TableCell align="center">Precipitation</TableCell>
                        <TableCell align="center">Winds</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      <TableRow>
                        <TableCell id='weatherTemp'>
                          {this.props.weather.main.temp} Fahr
                        </TableCell>
                        <TableCell id='weatherTempMax'>
                          {this.props.weather.main.temp_max} Fahr
                        </TableCell>
                        <TableCell id='weatherTempMin'>
                          {this.props.weather.main.temp_min} Fahr
                        </TableCell>
                        <TableCell id='weatherPrecip'>
                          {this.getPrecip()}
                        </TableCell>
                        <TableCell id='weatherWind'>
                          {this.props.weather.wind.speed} MPH
                        </TableCell>
                      </TableRow>
                    </TableBody>
                  </Table>
                </Paper>
              </div>
            </div>
          : <div style={{marginTop: 50}}>
              <Loader
                 type="Oval"
                 color="#00BFFF"
                 height="100"
                 width="100"
              />
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
