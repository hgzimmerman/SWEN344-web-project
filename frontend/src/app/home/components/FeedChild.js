import React, { Component } from 'react';
import Paper from '@material-ui/core/Paper';

export default class FeedChild extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      text: this.props.text,
      id: this.props.id,
      created_at: this.props.created_at,
      favorited: this.props.favorited,//
      favorite_count: this.props.favorite_count,
      user_name: this.props.user_name,
      user_id: this.props.user_id
    }
  }

  render() {
    if (this.state.user !== "") {
      return (
        <Paper>
          <small>{this.state.user_name} - {this.state.created_at}</small>
          <p>{this.state.text}</p>
        </Paper>
      );
    } else {
      return (
        <Paper>
          <small>{this.state.created_at}</small>
          <p>"{this.state.text}"</p>
          <small>Favorited {this.state.favorite_count} time(s)</small>
        </Paper>
      );
    }
  }
}
