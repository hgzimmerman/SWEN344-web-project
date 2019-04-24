import React, { Component } from 'react';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import '../../../App.css';

export default class PostView extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      post: this.props.post
    }
    this.onFeedPost = this.onFeedPost.bind(this);
    this.postFeed = this.props.postFeed.bind(this);
  }

  onFeedPost(e) {
    this.setState({
      post: e.target.value,
      isLoading: true
    })
  }

  render() {
    return (
      <div>
      <div>
        <TextField
          id="outlined-with-placeholder"
          label="Search"
          placeholder="Write your tweet"
          margin="normal"
          variant="outlined"
          onChange={this.onFeedPost}
          style={{width: '50%'}}
        />
      </div>

      <Button
        onClick={() => this.postFeed(this.state.stockName)}
        variant="contained"
      >
        Search
      </Button>
      </div>
    );
  }
}
