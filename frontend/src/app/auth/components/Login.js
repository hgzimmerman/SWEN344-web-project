import React from 'react';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import FacebookLogin from 'react-facebook-login';
import TwitterLogin from 'react-twitter-auth';
import { fakeAuth } from '../../../config/auth.js'
import { Redirect } from 'react-router-dom';
import '../../../App.css';
export var fbData = '';

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

  componentDidMount() {
    var oauth_nonce = "";
    var possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    for(var i = 0; i < 32; i++) {
      oauth_nonce += possible.charAt(Math.floor(Math.random() * possible.length));
    }

    var status = "Log In";
    var include_entities = "true";
    var oauth_signature_method = "HMAC-SHA1";
    var oauth_timestamp = (new Date).getTime();
    var oauth_token = "1103913649654001669-h84acKV147pk2QP2WK44uJ2eGsmVet";
    var oauth_version = "1.0";
    var oauth_consumer_key = "Pq2sA4Lfbovd4SLQhSQ6UPEVg";

    var parameter_string = "include_entities=" + encodeURIComponent(include_entities)
      + "&oauth_consumer_key=" + encodeURIComponent(oauth_consumer_key) + "&oauth_nonce="
      + encodeURIComponent(oauth_nonce) + "&oauth_timestamp=" + encodeURIComponent(oauth_timestamp)
      + "&oauth_token=" + encodeURIComponent(oauth_token) + "&oauth_version="
      + encodeURIComponent(oauth_version) + "&status=" + encodeURIComponent(status);

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
      fakeAuth.authenticate(() => {
        this.setState({ redirectToReferrer: true });
      });
    }
    else {
      alert('Could not authenticate')
    }

  }

  onFailed(){
    alert('Could not authenticate')
  }

  onSuccess(){
    fakeAuth.authenticate(() => {
      this.setState({ redirectToReferrer: true });
    });
  }

  render() {
    const responseFacebook = (response) => {
      if (response.accessToken !== null && response.accessToken !== undefined){
        fbData = response;
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
            <div style={{marginTop: 20}}>
              <TwitterLogin
                loginUrl="http://localhost:8000/login"
                onFailure={this.onFailed}
                onSuccess={this.onSuccess}
                requestTokenUrl="http://localhost:8000/api/v1/auth/twitter/reverse"
              />
            </div>
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
