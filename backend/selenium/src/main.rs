extern crate selenium_rs;

use selenium_rs::webdriver::{Selector, Browser, WebDriver};
use selenium_rs::element::Element;
use std::{thread, time};

#[test]
fn main() {
    //Start the webdriver
    let mut driver = WebDriver::new(Browser::Chrome);

    driver.start_session();
    driver.navigate("http://vm344c.se.rit.edu");

    //Wait for page to load
    thread::sleep(time::Duration::from_millis(1000));

    //TEST for Login
    let login_button = driver.query_element(Selector::CSS, "button[value=\"Login\"]").unwrap();
    assert_eq!(driver.get_current_url().unwrap(), String::from("https://vm344c.se.rit.edu/login"));
    login_button.click();

    let username_field = driver.query_element(Selector::CSS, "input[id=\"username_or_email\"]").unwrap();
    username_field.type_text("meatdownstairs@gmail.com");

    let password_field = driver.query_element(Selector::CSS, "input[id=\"password\"]").unwrap();
    password_field.type_text("iHave5meats");

    let submit_button = driver.query_element(Selector::CSS, "input[id=\"allow\"]").unwrap();
    submit_button.click();
    assert_eq!(driver.get_current_url().unwrap(), String::from("https://vm344c.se.rit.edu/"));

    //TEST for Stocks
    let stock_button = driver.query_element(Selector::CSS, "a[href=\"/stocks\"]").unwrap();
    stock_button.click();
    assert_eq!(driver.get_current_url().unwrap(), String::from("https://vm344c.se.rit.edu/stocks"));

    let stock_text = driver.query_element(Selector::CSS, "input[id=\"outlined-with-placeholder\"]").unwrap();
    stock_text.type_text("AAPL");

    let stock_search_button = driver.query_element(Selector::CSS, "button[id=\"stocksViewSearchBtn\"]").unwrap();
    stock_search_button.click();

    //Wait for button to appear
    thread::sleep(time::Duration::from_millis(1000));

    let purchase_button = driver.query_element(Selector::CSS, "button[style=\"background-color: rgb(28, 15, 19); color: white; height: 50px; width: 200px; margin-bottom: 20px;\"]").unwrap();
    purchase_button.click();

    //Wait for popup to appear
    thread::sleep(time::Duration::from_millis(1000));

    let purchase_text = driver.query_element(Selector::CSS, "input[placeholder=\"Buy shares…\"]").unwrap();
    purchase_text.type_text("1");

    let confirm_purchase_button = driver.query_element(Selector::CSS, "button[style=\"background-color: rgb(0, 166, 221); color: white; height: 50px; width: 200px; margin-top: 30px;\"]").unwrap();
    confirm_purchase_button.click();

    //TEST for Calendar
    let calendar_button = driver.query_element(Selector::CSS, "a[href=\"/calendar\"]").unwrap();
    calendar_button.click();
    assert_eq!(driver.get_current_url().unwrap(), String::from("https://vm344c.se.rit.edu/calendar"));

    let future_button = driver.query_element(Selector::CSS, "button[class=\"fc-next-button fc-button fc-state-default fc-corner-left fc-corner-right\"").unwrap();
    future_button.click();

    let past_button = driver.query_element(Selector::CSS, "button[class=\"fc-prev-button fc-button fc-state-default fc-corner-left fc-corner-right\"").unwrap();
    past_button.click();
    past_button.click();

    let week_button = driver.query_element(Selector::CSS, "button[class=\"fc-basicWeek-button fc-button fc-state-default\"]").unwrap();
    week_button.click();

    future_button.click();
    past_button.click();

    let day_button = driver.query_element(Selector::CSS, "button[class=\"fc-basicDay-button fc-button fc-state-default fc-corner-right\"]").unwrap();
    day_button.click();

    future_button.click();
    past_button.click();

    let today_button = driver.query_element(Selector::CSS, "button[class=\"fc-today-button fc-button fc-state-default fc-corner-left fc-corner-right\"]").unwrap();
    today_button.click();

    //TEST for Weather
    // TODO: Uncomment these lines when weather is fixed
    // let weather_button = driver.query_element(Selector::CSS, "a[href=\"/weather\"]").unwrap();
    // weather_button.click();
    // assert_eq!(driver.get_current_url().unwrap(), String::from("https://vm344c.se.rit.edu/weather"));

    //TEST for Adaptive Component
    let adaptive_button = driver.query_element(Selector::CSS, "a[href=\"/adaptive\"]").unwrap();
    adaptive_button.click();
    assert_eq!(driver.get_current_url().unwrap(), String::from("https://vm344c.se.rit.edu/adaptive"));

    //TEST for homepage
    let homepage_button = driver.query_element(Selector::CSS, "a[href=\"/\"]").unwrap();
    homepage_button.click();
    assert_eq!(driver.get_current_url().unwrap(), String::from("https://vm344c.se.rit.edu/"));

    //TEST for logout
    let profile_button = driver.query_element(Selector::CSS, "button[tabindex=\"0\"]").unwrap();
    profile_button.click();

    //Wait for logout button to appear
    thread::sleep(time::Duration::from_millis(500));

    let logout_button = driver.query_element(Selector::CSS, "li[tabindex=\"-1\"]").unwrap();
    logout_button.click();
    assert_eq!(driver.get_current_url().unwrap(), String::from("https://vm344c.se.rit.edu/login"));
}