use crate::api::API_STRING;
use log::info;
use warp::{
    self,
    filters::BoxedFilter,
    fs::File,
    path::Peek,
    reply::Reply,
    Filter,
};

const ASSETS_DIRECTORY: &str = "../../frontend/app/static/"; // TODO Point this at the build directory for the frontend.

pub struct FileConfig {
    static_dir_path: String,
    /// This is mostly to support testing.
    /// If set to Some, then the string in there will be used as the index,
    /// otherwise the app will assume that the static_dir_path/index.html is used for the index
    index_file_path: Option<String>
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            static_dir_path: ASSETS_DIRECTORY.to_string(),
            index_file_path: None
        }
    }
}

impl FileConfig {
    fn index(&self) -> String {
        if let Some(index) = &self.index_file_path {
            index.clone()
        } else {
            format!("{}index.html", self.static_dir_path)
        }
    }
}


/// Expose filters that work with static files
pub fn static_files_handler(file_config: FileConfig) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Static Files handler");

    let files = index(file_config.static_dir_path.clone())
        .or(index_static_file_redirect(file_config.index()));

    warp::any().and(files).with(warp::log("static_files")).boxed()
}

/// If the path does not start with /api, return the index.html, so the app will bootstrap itself
/// regardless of whatever the frontend-specific path is.
fn index_static_file_redirect(index_file_path: String) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path::peek())
        .and(warp::fs::file(index_file_path))
        .and_then(|segments: Peek, file: File| {
            // Reject the request if the path starts with /api/
            if let Some(first_segment) = segments.segments().next() {
                if first_segment == API_STRING {
                    //                    return Error::NotFound.reject()
                    return Err(warp::reject::not_found()); // TODO maybe keep this in the Error Type
                }
            }
            Ok(file)
        })
        .boxed()
}

fn index(dir_path: String) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::fs::dir(dir_path))
        .and(warp::path::end())
        .boxed()
}

#[test]
fn index_test() {
    // request the main file from this crate.
    let x = warp::test::request()
            .path("/src/main.rs")
            .reply(&index("./".to_string()));
    assert_eq!(x.status(), 200);
}

#[test]
fn static_files_404() {
    assert!(warp::test::request()
        .path("/api")
        .filter(&static_files_handler(FileConfig::default()))
        .is_err())
}

#[test]
fn static_files_redirect_to_index() {
    let config = FileConfig {
        index_file_path: Some("./src/main.rs".to_string()),
        ..Default::default()
    };

    assert!(warp::test::request()
        .path("/yeet")
        .filter(&static_files_handler(config))
        .is_ok())
}

#[test]
fn static_invalid_api_path_still_404s() {
    use crate::error::Error;
    let err = warp::test::request()
        .path("/api/yeet") // Matches nothing in the API space
        .filter(&static_files_handler(FileConfig::default()));

    let err: warp::Rejection = match err {
        Ok(_) => panic!("Error was expected, found valid Reply"),
        Err(e) => e,
    };
    assert!(err.is_not_found())
    //    let err = *err.into_cause::<Error>().expect("Should be a cause.");
    //    assert_eq!(err, Error::NotFound)
}