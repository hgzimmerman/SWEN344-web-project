extern crate selenium_rs;

use selenium_rs::{
    element::Element,
    webdriver::{Browser, Selector, WebDriver},
};

fn main() {
    let mut driver = WebDriver::new(Browser::Chrome);

    driver.start_session();
    driver.navigate("http://vm344c.se.rit.edu");

    let stocks_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[1]/header/div/a[2]/h6",
        )
        .unwrap();
    stocks_button.click();

    let stock_text = driver
        .query_element(Selector::XPath, "//*[@id=\"outlined-with-placeholder\"]")
        .unwrap();
    stock_text.click();
    stock_text.type_text("AAPL");

    let stock_search = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[2]/button/span[1]",
        )
        .unwrap();
    stock_search.click();

    let calendar_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[1]/header/div/a[3]/h6",
        )
        .unwrap();
    calendar_button.click();

    let month_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[2]/div/div[1]/button[3]",
        )
        .unwrap();
    month_button.click();

    let may_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[2]/div/div[2]/div/button[8]/abbr",
        )
        .unwrap();
    may_button.click();

    let seventeen_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[2]/div/div[2]/div/div/div[2]/button[19]/abbr",
        )
        .unwrap();
    seventeen_button.click();

    let weather_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[1]/header/div/a[4]/h6",
        )
        .unwrap();
    weather_button.click();
}
