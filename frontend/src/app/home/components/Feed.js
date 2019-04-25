import React from 'react';
import FeedChild from "./FeedChild";
import {authenticatedFetchDe} from "../../../config/auth";
import TextField from "@material-ui/core/TextField";
import Button from "@material-ui/core/Button";

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

  static transformTweet(tweet) {
    return {
      text: tweet.text,
      id: tweet.id,
      created_at: tweet.created_at,
      favorited: tweet.favorited,
      favorite_count: tweet.favorite_count,
      user_name: tweet.user.name,
      user_id: tweet.user.id
    }
  }

  componentDidMount() {
    const url = "/api/twitter_proxy/feed";
    return authenticatedFetchDe(url)
      .then(responseFeed => {
        let feed = responseFeed.map(tweet => Adaptive.transformTweet(tweet));
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
      .then(tweet => Adaptive.transformTweet(tweet))
      .then(tweet => {
        if (this.state.feed != null) {
          console.log("successfully tweeted a thing");
          this.setState(prevState => ({
            feed: [...prevState.feed.slice(), tweet] // stick the new tweet at the beginning
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
    let tweets = this.state.feed.map(tweet => {
      return <FeedChild
        text={tweet.text}
        id={tweet.id}
        created_at={tweet.created_at}
        favorited={tweet.favorited}
        favorite_count={tweet.favorite_count}
        user={tweet.user}
      />
    });

    return <div style={styles.tweet_container}>
      {tweets}
    </div>;
  }

  render() {
    return (
        <>
          {/*New Post*/}
          <div >
            <TextField
              id="outlined-with-placeholder"
              label="tweet"
              placeholder="Write your tweet"
              margin="normal"
              variant="outlined"
              onChange={this.updateNewTweet}
              style={{width: '50%'}}
            />
          </div>

          <Button
            onClick={() => this.postTweet()}
            variant="contained"
          >
            Tweet
          </Button>


          {/* Feed */}
          <div style={styles.scrolling_container}>
          {
            (!this.state.isLoading)
              ? (this.state.error != null)
                ? <div> {this.state.error.message} </div>
                : (this.state.feed !== undefined && this.state.feed !== null)
                  ? this.displayFeed()
                  : <div></div>
              : <div>Loading...</div>
          }
          </div>
        </>
      );
  }

}

const styles = {
  tweet_container: {
    // display: 'flex',
    // alignItems: 'center',
    // justifyContent: 'center',
    // flexDirection: "column",
    height: "100%",
    overflowY: "scroll"
  },
  scrolling_container: {
    overflow: "hidden"
  }
};
