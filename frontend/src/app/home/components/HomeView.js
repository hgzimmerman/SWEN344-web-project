import React from 'react';
import Paper from '@material-ui/core/Paper';
import Loader from 'react-loader-spinner';
import HomeStocksTable from '../../stocks/components/HomeStocksTable.js';
import '../../../App.css';
import Feed from "./Feed";

export default class HomeView extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      visible: false,
      isLoading: true
    }
  }


  openModal(){
    this.setState({ visible: true });

  }

  closeModal(){
    this.setState({ visible: false });
  }

  componentDidMount() {
    this.setState({isLoading: false})
  }

  render() {
    return (
      <div className="App" id='homeRoot'>
        {
          (!this.props.isLoading)
          ? <div style={styles.container} id='homeContainer'>
              <Paper style={styles.feed} id='homeFeedContainer'>
                <h2 id='feedTitle'>Twitter Feed</h2>
                <Feed id='homeFeed'/>
              </Paper>

              <div style={{padding: 30}} id='homeWeatherContainer'>
                <Paper style={styles.weather} id='homeWeather'>
                  <h2 id='weatherTitle'>Temperature for {this.props.zipCode}</h2>
                  <p style={styles.temp} id='weatherTemp'>
                    {this.props.weather.main.temp} F
                  </p>
                </Paper>

                <Paper style={styles.events} id='homeEvents'>
                  <h2 id='eventsTitle'>Events</h2>
                  <p style={styles.text} id='noEvents'>No events scheduled for today</p>
                </Paper>
                <div>
                  <HomeStocksTable
                    id='homeStocksTable'
                    stocks={this.props.stocks}
                    sellStock={this.props.sellStock}
                  />
                </div>
              </div>

              <Paper style={styles.ad} id='adBanner'>
                    <img src="/api/advertisement" alt="advertisement" id='adImg'/>
              </Paper>


            </div>
          : <div style={{marginTop: 50}} id='loaderContainer'>
              <Loader
                 id='homeLoader'
                 type="Oval"
                 color="#00BFFF"
                 height="100"
                 width="100"
              />
            </div>
        }
      </div>
    );

  }

}

const styles = {
  container: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    padding: 10
  },
  stocks: {
    width: '40%',
    height: 300,
    marginTop: 20,
    textAlign: 'center',
    color: 'black'
  },
  feed: {
    display: "flex",
    flexDirection: "column",
    width: '40%',
    height: 700,
    marginTop: 20,
    textAlign: 'center',
    color: 'black'
  },
  weather: {
    height: 150,
    marginTop: 20,
  },
  temp: {
    color: '#00A6DD',
    fontWeight: '400',
    fontSize: 40
  },
    ad: {
    height: 200
  },
  events: {
    height: 230
  },
  text: {
    color: '#7c7c7c'
  },

}
