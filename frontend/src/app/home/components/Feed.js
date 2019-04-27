import React from 'react';
import {authenticatedFetchDe} from "../../../config/auth";
import TextField from "@material-ui/core/TextField";
import Button from "@material-ui/core/Button";
import Paper from "@material-ui/core/Paper";

export default class Adaptive extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      feed: [],
      newTweet: "",
      isLoading: true,
      error: null
    };
    this.postTweet = this.postTweet.bind(this);
    this.updateNewTweet = this.updateNewTweet.bind(this);
    this.displayFeed = this.displayFeed.bind(this);
  }

  componentDidMount() {
    const url = "/api/twitter_proxy/feed";
    return authenticatedFetchDe(url)
      .then(feed => {
        this.setState({
          feed,
          isLoading: false,
          error: null
        })
      })
      .catch(error => this.setState({
        error
      }));
  }

  postTweet() {
    console.log("posting tweet");
    const url = "/api/twitter_proxy/tweet";
    let body_obj = {
      text: this.state.newTweet
    };
    let body = JSON.stringify(body_obj);
    return authenticatedFetchDe(url, {method: "POST", body})
      .then(tweet => {
        if (this.state.feed != null) {
          console.log("successfully tweeted a thing");
          this.setState(prevState => ({
            newTweet: "",
            feed: [tweet, ...prevState.feed.slice()], // stick the new tweet at the beginning
            error: null
          }))
        }
      })
      .catch(error => this.setState({
        error
      }));
  }


  updateNewTweet(e) {
    this.setState({
      newTweet: e.target.value
    })
  }

  displayFeed() {
    let count = 0;
    let tweets = this.state.feed.map(tweet => {
      let id = 'tweet-' + count;
      let userTag = 'tag-' + count;
      let textTag = 'text-' + count;
      return (
        <Paper id={id}>
          <small id={userTag}>{tweet.user.name} - {tweet.created_at}</small>
          <p id={textTag}>{tweet.text}</p>
        </Paper>
      );
      count++;
    });

    return <div style={styles.tweet_container} id="feed">
      {tweets}
    </div>;
  }

  render() {
    return (
        <>
          {/*New Post*/}
          <div id="textContainer">
            <TextField
              id="outlined-with-placeholder"
              label="tweet"
              placeholder="Write your tweet"
              margin="normal"
              variant="outlined"
              onChange={this.updateNewTweet}
              style={{width: '70%'}}
            />
          </div>

          <Button
            id="postButton"
            onClick={() => this.postTweet()}
            variant="contained"
          >
            Tweet
          </Button>


          {/* Feed */}
          <div style={styles.scrolling_container} id="feedContainer">
          {
            (!this.state.isLoading)
              ? (this.state.error != null)
                ? <div id='errorDiv'> {this.state.error.message} </div>
                : (this.state.feed !== undefined && this.state.feed !== null)
                  ? this.displayFeed()
                  : <div id='emptyDiv'></div>
              : <div id='loadingDiv'>Loading...</div>
          }
          </div>
        </>
      );
  }

}

const styles = {
  tweet_container: {
    height: "100%",
    overflowY: "scroll"
  },
  scrolling_container: {
    overflow: "hidden"
  }
};
