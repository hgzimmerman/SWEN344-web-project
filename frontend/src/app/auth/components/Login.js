import React from 'react';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import FacebookLogin from 'react-facebook-login';
import { Redirect } from 'react-router-dom';
import '../../../App.css';

export default class Login extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      username: null,
      password: null
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
      return (this.props.history.push("/"));
    }
    else {
      alert('Could not authenticate')
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
