extern crate selenium_rs;

use selenium_rs::webdriver::{Selector, Browser, WebDriver};
use selenium_rs::element::Element;

fn main() {
    let mut driver = WebDriver::new(Browser::Firefox);
    driver.start_session();
    driver.navigate("http://www.google.com");

    let search_form =  driver.query_element(Selector::CSS, "#searchform").unwrap();
    assert!(search_form.get_css_value("min-width").unwrap() == "980px");
}