import React from 'react';
import Paper from '@material-ui/core/Paper';
import Loader from 'react-loader-spinner';
import HomeStocksTable from '../../stocks/components/HomeStocksTable.js';
import PostForm from '../containers/PostForm.js';
import '../../../App.css';

export default class HomeView extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      visible: false
    }
  }

  openModal(){
    this.setState({ visible: true });

  }

  closeModal(){
    this.setState({ visible: false });

  }

  render() {
    return (
      <div className="App">
        {
          (!this.props.isLoading)
          ? <div style={styles.container}>
              <Paper style={styles.feed}>
                <h2>Feed</h2>
                <PostForm/>
              </Paper>

              <div style={{padding: 30}}>
                <Paper style={styles.weather}>
                  <h2>RIT's Temperature</h2>
                  <p style={styles.temp}>
                    {this.props.weather.main.temp} F
                  </p>
                </Paper>

                <Paper style={styles.events}>
                  <h2>Events</h2>
                  <p style={styles.text}>No events scheduled for today</p>
                </Paper>
                <div>
                  <HomeStocksTable
                    stocks={this.props.stocks}
                    sellStock={this.props.sellStock}
                  />
                </div>
              </div>
              <Paper style={styles.ad}>
                    <img src="/api/advertisement" alt="advertisement"></img>
              </Paper> 

            </div>
          : <div style={{marginTop: 50}}>
              <Loader
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
