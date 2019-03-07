import React from 'react';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import FacebookLogin from 'react-facebook-login';
import { fakeAuth } from '../../../config/auth.js'
import { Redirect } from 'react-router-dom';
import '../../../App.css';
export var fbData = '';
export var loggedIn = false;

export default class Login extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      username: null,
      password: null,
      redirectToReferrer: false
    }
    this.onChangeUsername = this.onChangeUsername.bind(this);
    this.onChangePassword = this.onChangePassword.bind(this);

  }

  onChangeUsername(e){
    this.setState({
      username: e.target.value,
    });

  }

  onChangePassword(e){
    this.setState({
      password: e.target.value,
    });

  }

  authenticate(){
    if (this.state.username === 'test' && this.state.password === 'test'){
      loggedIn = true;
      fakeAuth.authenticate(() => {
        this.setState({ redirectToReferrer: true });
      });
    }
    else {
      alert('Could not authenticate')
    }

  }

  render() {
    const responseFacebook = (response) => {
      if (response.accessToken !== null && response.accessToken !== undefined){
        fbData = response;
        loggedIn = true;
        fakeAuth.authenticate(() => {
          this.setState({ redirectToReferrer: true });
        });
      }
      else {
        alert('Could not authenticate')
      }

    }
    let { from } = this.props.location.state || { from: { pathname: "/" } };
    let { redirectToReferrer } = this.state;

    if (redirectToReferrer) return <Redirect to={from} />;

    return (
      <div className="App">
        <div style={styles.container}>
          <TextField
            id="outlined-with-placeholder"
            label="Username"
            placeholder="Username"
            margin="normal"
            variant="outlined"
            onChange={this.onChangeUsername}
            style={{width: '50%'}}
          />
          <TextField
            id="outlined-with-placeholder"
            label="Password"
            placeholder="Password"
            type="password"
            margin="normal"
            variant="outlined"
            onChange={this.onChangePassword}
            style={{width: '50%'}}
          />
          <div style={styles.container}>
            <Button
              onClick={() => this.authenticate()}
              variant="contained"
              style={styles.button}
            >
              Login
            </Button>
            <FacebookLogin
              appId="250744242473852"
              fields="name,email,picture"
              callback={responseFacebook}
            />
          </div>
        </div>
      </div>
    );

  }

}

const styles = {
  container: {
    padding: 15,
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
  },
  button: {
    backgroundColor: '#00A6DD',
    color: 'white',
    marginBottom: 20,
    height: 50,
    width: 200
  }
}
