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
    this.setState({ buyStockModalVisible: true });
  }

  closeModal(){
    this.setState({ buyStockModalVisible: false });
  }

  componentDidMount() {
    this.setState({isLoading: false})
  }

  render() {
    const adUrl = "/api/advertisement";
    return (
      <div className="App">
        {
          (!this.props.isLoading)
          ? <div style={styles.container}>
              <div style={styles.section}>
                <Paper style={styles.feed} id={"TwitterCard"}>
                  <h2>Twitter Feed</h2>
                  <Feed/>
                </Paper>
              </div>

              <div style={styles.section}>
                {renderEvents(this.props.events)}
                <HomeStocksTable
                  stocks={this.props.stocks}
                />
              </div>

              <div style={styles.section}>
                <Paper style={styles.ad} id={"AdCard"}>
                  <img src={adUrl} alt="advertisement"/>
                </Paper>
                <Paper style={styles.weather}>
                  <h2>Temperature for {this.props.zipCode}</h2>
                  <p style={styles.temp}>
                    {
                      (this.props.weather !== null
                        && this.props.weather !== undefined
                        && this.props.weather.main !== null
                        && this.props.weather.main !== undefined)
                        ? <>{this.props.weather.main.temp} F</>
                        : <></>
                    }
                  </p>
                </Paper>
              </div>


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

function renderEvents(events) {
  return (
  <Paper style={styles.events} id={"EventsCard"}>
    <h2>Events</h2>
    {/*TODO Actually show the event instead of stringifying them*/}
    {
      (events.length > 0)
        ? <>{JSON.stringify(events)}</>
        : <p style={styles.text}>No events scheduled for today</p>
    }
  </Paper>
  )
}

const styles = {
  container: {
    display: 'flex',
    flexWrap: 'wrap',
    justifyContent: 'center',
  },
  section: {
    display: 'flex',
    flexDirection: 'column',
    paddingLeft: "15px",
    paddingRight: "15px",
    marginTop: "25px",
    minWidth: 440
  },
  stocks: {
    width: '40%',
    height: 300,
    textAlign: 'center',
    color: 'black'
  },
  feed: {
    display: "flex",
    flexDirection: "column",
    width: "600px",
    height: "700px",
    textAlign: 'center',
    color: 'black'
  },
  weather: {
    marginTop: 20,
    minHeight: 150,
  },
  temp: {
    color: '#00A6DD',
    fontWeight: '400',
    fontSize: 40
  },
  ad: {
    height: "400px",
    minWidth: "300px"

  },
  events: {
    flexGrow: 3,
    height: 250
  },
  text: {
    color: '#7c7c7c'
  },

};
