import React from 'react';
import WeatherTable from '../components/WeatherTable.js';
import {authenticatedFetchDe} from "../../../config/auth";
import Button from "@material-ui/core/Button";
import TextField from "@material-ui/core/TextField";

export default class Weather extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      weather: null,
      isLoading: true,
      zipCode: '14623',
      newZipCode: '',
      error: null
    };
    this.getWeather = this.getWeather.bind(this);
    this.handleOnChangeNewZip = this.handleOnChangeNewZip.bind(this);
    this.setZipCode = this.setZipCode.bind(this);
    this.handleKeyPressNewZip = this.handleKeyPressNewZip.bind(this);
  }

  componentDidMount() {
    this.getZipCode().then(() => this.getWeather(this.state.zipCode));
  }

  /**
   * Gets the zip code from the backend.
   * Will default to the rochester zip code if the user hasn't set their zip code yet.
   */
  getZipCode() {
    const url = "/api/user/zip";
    return authenticatedFetchDe(url)
      .then(response => {
        const defaultZipCode = "14623";
        if (response !== null && response !== undefined) {
          this.setState({zipCode: response, error: null})
        } else {
          this.setState({zipCode: defaultZipCode})
        }
      });
  }

  getWeather(zipCode) {
    this.setState({
      isLoading: true
    });
    const url = `https://api.openweathermap.org/data/2.5/weather?zip=${zipCode},us&APPID=4c442afc1ade3bc91a9bb48fb1fd0e02&units=imperial`;
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          if (Object.entries(res).length === 0) {
            console.log("Error, could not get weather");
            this.setState({
              error: true,
              isLoading: false
            });
          }
          else {
            console.log("Weather retrieved " + res);
            console.log(res.weather);
            this.setState({
              weather: res,
              isLoading: false
            });
          }
        });
  }

  /**
   * Handle changing text for the new zip text field
   */
  handleOnChangeNewZip(event) {
      this.setState({
        newZipCode: event.target.value,
      });
  }
  /**
   * Handle the Enter handling for the new zip text field
   */
  handleKeyPressNewZip(event) {
     switch (event.key) {
        case 'Enter':
          console.log("enter pressed");
          this.setZipCode().then(() => this.getWeather(this.state.zipCode));
          break;
        default:
          break;
    }
  }

  /**
   * Sets the zip code on the backend
   * It uses the newZipCode state value to set the backend, and changes the zipcode state value in response.
   */
  setZipCode() {
    const url = "/api/user/zip";

    const extras = {method: "PUT", body: JSON.stringify(this.state.newZipCode)};
    return authenticatedFetchDe(url, extras)
      .then(response => {
        this.setState({zipCode: response, error: null})
      })
      .catch(error => {
        this.setState({error})
      })
  }

  render() {
    return(

      <div className="App">
        <h4 id='weatherTitle'> Current Zip Code: {this.state.zipCode} </h4>

        <div style={styles.searchBar} id='weatherSearch'>
          <TextField
            value={this.state.newZipCode}
            id="outlined-with-placeholder"
            label="Zip Code"
            placeholder="ZIP Code..."
            margin="normal"
            variant="outlined"
            onChange={this.handleOnChangeNewZip}
            onKeyPress={this.handleKeyPressNewZip}
            style={{width: '50%'}}
          />
        </div>
        <Button
          id='weatherSearch'
          onClick={() => this.setZipCode().then(() => this.getWeather(this.state.zipCode))}
          variant="contained"
          style={styles.button}
        >
          Change Zip Code
        </Button>
        <div style={{textAlign: 'center'}} >

        </div>
        <div id='weatherDisplay'>
          {
            (!this.state.isLoading)
              ? (this.state.error)
                ? <p id='errorMessage'>
                    Zip Code
                    <span style={styles.bold}> "{this.state.zipCode}" </span>
                    does not exist
                  </p>
                : (this.state.weather !== undefined)
                  ? <div>
                      <WeatherTable
                        weather={this.state.weather}
                      />
                    </div>
                  : <div><p id='weatherPrompt'>Enter a Zip Code for Weather</p></div>
                : <div><p id='weatherLoading'>Weather Loading...</p></div>
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
};
