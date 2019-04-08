import React from 'react';
import Paper from '@material-ui/core/Paper';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import Loader from 'react-loader-spinner';

export default class WeatherView extends React.component {
  constructor(props){
    super(props);
    this.state = {
      weather: this.props.weather,
      zipCode: '',
      isLoading: this.props.isLoading
    }
    this.onSearchWeather = this.onSearchWeather.bind(this);
    this.getWeather = this.props.getWeather.bind(this);
  }

    onSearchWeather(e) {
      this.setState({
        zipCode: e.target.value,
        isLoading: true
      });
    }

    render() {
      return {
        <div className="App">
          <div style={styles.searchBar}>
            <TextField
              id="outlined-with-placeholder"
              label="Search"
              placeholder="ZIP Code..."
              margin="normal"
              variant="outlined"
              onChange={this.onSearchWeather}
              style={{width: '50%'}}
            />
          </div>

          <Button
            onClick={() => this.getWeather(this.state.zipCode)}
            variant="contained"
            style={styles.button}
          >
            Search
          </Button>
          <div style={{textAlign: 'center'}} >

          </div>
          <div>
            {
              (!this.state.isLoading)
                ? (this.state.error)
                  ? <p>
                      ZIP Code
                      <span style={styles.bold}> "{this.state.zipCode}" </span>
                      does not exist
                    </p>
                  : (this.state.weather !== undefined)
                    ? <div>
                        <WeatherTable
                          weather={this.state.weather}

                        />
                      </div>
                    : <div></div>
                : <div></div>

            }
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
  button: {
    backgroundColor: '#00A6DD',
    color: 'white',
    height: 50,
    width: 200
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
  bold: {
    fontWeight: 'bold'
  }
