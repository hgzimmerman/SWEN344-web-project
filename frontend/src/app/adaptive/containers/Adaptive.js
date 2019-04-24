import React from 'react';
import {getJwtBearer} from "../../../config/auth";

export default class Adaptive extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      // Timeserries data regarding serving rates
      data: '',
      isLoading: true,
      error: null
    }
  }

  getAdServeRateData() {
    let headers = {
      "Authorization" : getJwtBearer()
    };
    const url = "/api/health/week";
    fetch(url, {headers})
      .then((res) => {
        let json = res.json();
        if (res.ok) {
          this.setState({
            data: json
          });
        } else {
          this.setState({
            error: json
          });
        }
      });

  }
}


