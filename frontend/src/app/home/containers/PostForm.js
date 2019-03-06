import React from 'react';
import Button from '@material-ui/core/Button';
import { fbData } from '../../auth/components/Login.js';

export default class PostForm extends React.Component {
  constructor(props) {
    super(props);
    this.state = {value: ''};

    this.handleChange = this.handleChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  handleChange(event) {
    this.setState({value: event.target.value});
  }

  handleSubmit() {
    if (fbData !== '') {
      console.log(fbData)
      const url = `https://graph.facebook.com/${fbData.id}/feed?message=${this.state.value}&access_token=${fbData.accessToken}`;
      return fetch(url, { method: 'POST' })
        .then((res) => res.json())
          .then((res) => {
            console.log(res);
            if (res.id !== null) {
              alert("Post successfully made!");
            }
            else {
              alert("Failed to post");
            }
          })
    }
    else {
      alert("Login to Facebook to post");
    }

  }

  render() {
    return (
      <form onSubmit={this.handleSubmit}>
        <label>
          Create Post
          <br/>
          <textarea value={this.state.value} onChange={this.handleChange}/>
        </label>
        <br/>
        <Button
          onClick={() => this.handleSubmit()}
          variant="contained"
          style={styles.button}
        >
          Publish
        </Button>
      </form>
    );
  }
}

const styles = {
  button: {
    backgroundColor: '#00A6DD',
    color: 'white',
    height: 50,
    width: 200
  },
}
