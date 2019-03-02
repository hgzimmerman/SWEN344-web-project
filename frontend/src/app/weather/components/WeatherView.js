import React from 'react';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

export default class WeatherView extends React.Component {
  constructor(props){
    super(props);
		this.state={
			weather: this.props.weather
		}

  }

	getPrecip() {
		var prec = '';
		var i;
		for (i = 0; i < this.state.weather.weather.length; i++) {
			if (i === this.state.weather.weather.length-1) {
				prec += this.state.weather.weather[i].main;
			}
			else {
				prec += (this.state.weather.weather[i].main + ', ');
			}
		}
		console.log("prec: " + prec);
		return prec;
	}

  render(){
		const prec = this.getPrecip();
		//	{
		//		"coord":{"lon":-122.09,"lat":37.39},
		//		"sys":{"type":3,"id":168940,"message":0.0297,"country":"US","sunrise":1427723751,"sunset":1427768967},
		//		"weather":[{"id":800,"main":"Clear","description":"Sky is Clear","icon":"01n"}],
		//		"base":"stations",
		//		"main":{"temp":285.68,"humidity":74,"pressure":1016.8,"temp_min":284.82,"temp_max":286.48},
		//		"wind":{"speed":0.96,"deg":285.001},
		//		"clouds":{"all":0},
		//		"dt":1427700245,
		//		"id":0,
		//		"name":"Mountain View",
		//		"cod":200
		//	}

    return(
      <div className="App">
        <Paper levation={1} >
        <div style={{float:'left', margin: 10}}>
          <Paper style={styles.root}>
            <Table style={styles.table}>
        			<TableHead>
								<TableRow>
									<TableCell>
										<b>
											RIT Weather
										</b>
									</TableCell>
								</TableRow>
							</TableHead>
              <TableHead>
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
									<TableCell>
										{this.state.weather.main.temp} Fahr
									</TableCell>
									<TableCell>
										{this.state.weather.main.temp_max} Fahr
									</TableCell>
									<TableCell>
										{this.state.weather.main.temp_min} Fahr
									</TableCell>
									<TableCell>
										{prec}
									</TableCell>
									<TableCell>
										{this.state.weather.wind.speed} MPH
									</TableCell>
								</TableRow>
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
