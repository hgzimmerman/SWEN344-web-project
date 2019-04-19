use crate::state::State;
use warp::{Reply};
use crate::{error::Error, server_auth::twitter_token_filter, util::json_body_filter};
use apply::Apply;
use egg_mode::{
    tweet::{DraftTweet, Timeline, Tweet},
    Response, Token,
};
use futures::future::Future;
use log::info;
use serde::{Deserialize, Serialize};
use warp::{get2, path, post2, Filter};
use warp::Rejection;

/// Request used to create a tweet
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TweetRequest {
    pub text: String,
}

/// Response for a tweet
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TweetResponse {
    pub text: String,
    pub id: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub favorited: Option<bool>,
    pub favorite_count: i32,
    pub user: Option<TwitterUser>,
}

impl From<Tweet> for TweetResponse {
    fn from(t: Tweet) -> Self {
        TweetResponse {
            text: t.text,
            id: t.id,
            created_at: t.created_at,
            favorited: t.favorited,
            favorite_count: t.favorite_count,
            user: t.user.map(|u| {
                let u = *u;
                TwitterUser {
                    name: u.name,
                    id: u.id,
                }
            }),
        }
    }
}

/// Twitter user
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TwitterUser {
    pub name: String,
    pub id: u64,
}

/// Proxy for twitter related things.
pub fn twitter_proxy_api(state: &State) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    info!("Attaching twitter proxy");

    let post_tweet = path!("tweet")
        .and(post2())
        .and(json_body_filter(3))
        .and(twitter_token_filter(state))
        .and_then(|request: TweetRequest, twitter_token: Token| {
            DraftTweet::new(request.text)
                .send(&twitter_token)
                .map_err(|e| {
                    Error::dependent_connection_failed_context(format!("Tweet failed to send: '{}'", e))
                        .reject()
                })
        })
        .map(|tweet: Response<Tweet>| {
            TweetResponse::from(tweet.response).apply(|x| warp::reply::json(&x))
        });

    let get_feed = path!("feed")
        .and(get2())
        .and(twitter_token_filter(state))
        .and_then(|twitter_token: Token| {
            egg_mode::tweet::home_timeline(&twitter_token)
                .with_page_size(50)
                .start()
                .map_err(|e| {
                    Error::dependent_connection_failed_context(format!("Could not get twitter feed: '{}'", e))
                        .reject()
                })
        })
        .untuple_one()
        .map(
            |_timeline: Timeline, feed_responses: Response<Vec<Tweet>>| {
                feed_responses
                    .response
                    .into_iter()
                    .map(TweetResponse::from)
                    .collect::<Vec<_>>()
                    .apply(|x| warp::reply::json(&x))
            },
        );

    path!("twitter_proxy").and(post_tweet.or(get_feed))
}
