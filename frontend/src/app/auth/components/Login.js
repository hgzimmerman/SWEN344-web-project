import React from 'react';
<<<<<<< HEAD
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

=======
import '../../../App.css';

export default class Login extends React.Component {
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <p>
            Login Page
          </p>

        </header>
      </div>
    );
    
>>>>>>> 8260ea595b66949fe44859d17a7afb5ae07d1fa8
  }

}
