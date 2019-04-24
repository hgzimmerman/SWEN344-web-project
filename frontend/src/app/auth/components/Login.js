import React from 'react';
import Button from '@material-ui/core/Button';
import '../../../App.css';
export var fbData = '';

export default class Login extends React.Component {
  constructor(props){
    super(props);
    this.state = {
      link: null
    };
  }

  /**
   * When the component mounts, it will fetch the login link it needs to use from the api.
   */
  componentDidMount() {
    fetch('/api/auth/link')
        .then(response => {
          let json = response.json();
          if (response.ok) {
              return json;
          } else {
              return json.then(err => {throw err;});
          }
        })
        .catch(err => {
          alert(err)
        })
        .then( data => {
          this.setState({link: data.authentication_url})
        })
  }

  /**
   * Goes to the login link.
   */
  goToLink() {
    window.location.href = this.state.link;
  }

  render() {
    return (
      <div className="App">
        <div style={styles.container}>
          <Button
              onClick={() => this.goToLink()}
              value="Login"
              variant="contained"
              style={styles.button}
          >
            Login using Twitter
          </Button>
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
};
