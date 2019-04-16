use selenium_rs::{
    element::Element,
    webdriver::{Browser, Selector, WebDriver},
};

#[allow(dead_code)]
fn test_stock_page() {
    let mut driver = WebDriver::new(Browser::Chrome);

    driver.start_session().unwrap();
    driver.navigate("http://vm344c.se.rit.edu").unwrap();

    let stocks_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[1]/header/div/a[2]/h6",
        )
        .unwrap();
    stocks_button.click().unwrap();

    let stock_text = driver
        .query_element(Selector::XPath, "//*[@id=\"outlined-with-placeholder\"]")
        .unwrap();
    stock_text.click().unwrap();
    stock_text.type_text("AAPL").unwrap();

    let stock_search = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[2]/button/span[1]",
        )
        .unwrap();
    stock_search.click().unwrap();

    let calendar_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[1]/header/div/a[3]/h6",
        )
        .unwrap();
    calendar_button.click().unwrap();

    let month_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[2]/div/div[1]/button[3]",
        )
        .unwrap();
    month_button.click().unwrap();

    let may_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[2]/div/div[2]/div/button[8]/abbr",
        )
        .unwrap();
    may_button.click().unwrap();

    let seventeen_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[2]/div/div[2]/div/div/div[2]/button[19]/abbr",
        )
        .unwrap();
    seventeen_button.click().unwrap();

    let weather_button = driver
        .query_element(
            Selector::XPath,
            "//*[@id=\"root\"]/main/div[1]/header/div/a[4]/h6",
        )
        .unwrap();
    weather_button.click().unwrap();
}
