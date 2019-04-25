import React from 'react';
import FeedChild from "./FeedChild";
import {authenticatedFetchDe} from "../../../config/auth";

export default class Adaptive extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      feed: null,
      isLoading: true,
      error: null
    }
  }

  componentDidMount() {
    const url = "/api/twitter_proxy/feed";
    return authenticatedFetchDe(url)
      .then(feed => this.setState({
        feed,
        isLoading: false,
        error: null
      }))
      .catch(error => this.setState({
        error
      }));
  }


  render() {
    return (
        (!this.state.isLoading)
          ? (this.state.error)
            ? <div> {this.state.error.message} </div>
            : (this.state.feed !== undefined && this.state.feed !== null)
                ? displayFeed(this.state.feed)
                : <div></div>
            : <div>Loading...</div>
      );
  }

}


function displayFeed(feed) {
  var tweets = [];
  for (var i = 0; i < feed.length; i++) {
    tweets.push(<FeedChild
      text={feed[i].text}
      id={feed[i].id}
      created_at={feed[i].created_at}
      favorited={feed[i].favorited}
      favorite_count={feed[i].favorite_count}
      user={feed[i].user}
    />)
  }
  return <div>{tweets}</div>;
}