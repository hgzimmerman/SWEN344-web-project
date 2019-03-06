import React from 'react';
import {accessToken} from '../../auth/components/Login.js';

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

  handleSubmit(event) {
    // ToDo: The needful     POST https://graph.facebook.com/546349135390552/feed?message=Hello Fans!&access_token=your-access-token
    const url = `https://graph.facebook.com/546349135390552/feed?message=${event.target.value}&access_token=${accessToken}`;
    return fetch(url, { method: 'POST' })
      .then((res) => res.json())
        .then((res) => {
          if (res.id !== null) {
            alert("Post successfully made!");
          } else {
            alert("Failed to post");
          }
        }
      )
  }

  render() {
    return (
      <form onSubmit={this.handleSubmit}>
        <label>
          Create Post
          <br/>
          <textArea value={this.state.value} onChange={this.handleChange} width="95%"/>
        </label>
        <br/>
        <input type="submit" value="Submit" />
      </form>
    );
  }
}
