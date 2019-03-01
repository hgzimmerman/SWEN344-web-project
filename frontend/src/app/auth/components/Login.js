import React from 'react';
import FacebookLogin from 'react-facebook-login';
import { Redirect } from 'react-router-dom';
import '../../../App.css';

export default class Login extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      username: null
    }

  }

  render() {
    const responseFacebook = (response) => {
      if (response.accessToken !== null && response.accessToken !== undefined){
        return (this.props.history.push("/"));
      }
      else {
        alert('Could not authenticate')
      }

    }

    return (
      <div className="App">
        <header className="App-header">
          <FacebookLogin
            appId="250744242473852"
            fields="name,email,picture"
            callback={responseFacebook}
          />
        </header>
      </div>
    );

  }

}
