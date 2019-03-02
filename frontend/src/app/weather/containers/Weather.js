import React from 'react';
import StocksView from '../components/WeatherView.js';

export default class Weather extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      weather: null,
      isLoading: true
    }
    this.getWeather = this.getWeather.bind(this);

  }

  componentDidMount(){
    this.getWeather();

  }
  
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

  getWeather(){
    const url = 'api.openweathermap.org/data/2.5/weather?zip=14623,us&APPID=2fe269f5d5f752aef9acf88ad1448cd'
    return fetch(url, { method: 'GET' })
      .then((res) => res.json())
        .then((res) => {
          this.setState({
            weather: res,
            isLoading: false
          });
          console.log(res)
        })

  }

  render(){
    return(
      (!this.state.isLoading)
      ? <WeatherView
          weather={this.state.weather}
        />
      : <p>Loading Weather...</p>


    );

  }

}
