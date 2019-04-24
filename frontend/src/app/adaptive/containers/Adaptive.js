import React from 'react';
import {getJwtBearer} from "../../../config/auth";
import AdaptiveView from "../components/AdaptiveView";

export default class Adaptive extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      // Timeserries data regarding serving rates
      data: null,
      isLoading: true,
      error: null
    }
  }

  getAdServeRateData() {
    let headers = {
      "Authorization" : getJwtBearer()
    };
    const url = "/api/health/week";
    return fetch(url, {headers})
      .then((res) => {
        res.json().then(json => {
          if (res.ok) {
            console.log("loaded the health data");
            this.setState({
              data: json,
              isLoading: false
            });
          } else {
            this.setState({
              error: json,
              isLoading: false
            });
          }
        })

      });
  }

  componentDidMount() {
    console.log("mounted adaptive component");
    this.getAdServeRateData();
  }



  render(){
    return(
      <AdaptiveView
        data={this.state.data}
        isLoading={this.state.isLoading}
        error={this.state.error}
      />
    );

  }
}


