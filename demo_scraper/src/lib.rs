use reqwest::Error;
use std::process::Command;
use std::{fmt::Display, thread::sleep};
use thirtyfour::{
  common::capabilities::firefox::LogLevel,
  common::capabilities::chrome,
  common::capabilities::chrome::ChromeCapabilities,
  error::WebDriverError,
  extensions::query::{ElementQueryable, ElementWaitable},
  By, DesiredCapabilities, WebDriver, ChromiumLikeCapabilities
};

// import std base64
use base64::prelude::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use thirtyfour::extensions::cdp::ChromeDevTools;

trait ToSnakeCase {
  fn to_snake_case(&self) -> String;
}

impl ToSnakeCase for str {
  fn to_snake_case(&self) -> String {
    let mut result = String::with_capacity(self.len());
    for (i, c) in self.chars().enumerate() {
      if c.is_uppercase() {
        if i > 0 {
          result.push('_');
        }
        // Directly convert uppercase ASCII characters to lowercase
        if c.is_ascii() {
          result.push(((c as u8) + 32) as char);
        } else {
          // Fallback for non-ASCII characters
          result.extend(c.to_lowercase());
        }
      } else {
        result.push(c);
      }
    }
    result
  }
}

#[derive(
  Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Page {
  url: String,
  title: String,
  source: String,
  screenshot: Option<Vec<u8>>,
}

impl Display for Page {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // serialize the page to a JSON string
    let json = serde_json::to_string(self).unwrap();
    write!(f, "{}", json)
  }
}

pub trait Base64Encoded {
  fn to_base64(&self) -> String;
  fn from_base64(b64_str: &str) -> Result<Self, base64::DecodeError>
  where
    Self: Sized;
}

impl Base64Encoded for Vec<u8> {
  fn to_base64(&self) -> String {
    BASE64_STANDARD.encode(&self)
  }
  fn from_base64(b64_str: &str) -> Result<Self, base64::DecodeError> {
    BASE64_STANDARD.decode(b64_str)
  }
}

#[derive(Debug, Clone)]
pub struct Browser {
  driver: WebDriver,
  //body: Option<thirtyfour::WebElement>,
  //cookies: Vec<thirtyfour::Cookie>,
}

impl Browser {
  // Asynchronously create a new instance of the browser
  pub async fn new(port: i16) -> Result<Self, WebDriverError> {
    //let mut caps = DesiredCapabilities::chrome();
    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;
    caps.set_log_level(LogLevel::Debug)?;
    //If Using chrome:
    //caps.add_arg("--verbose");
    //caps.add_arg("--headless");

    let driver_result =
      WebDriver::new(format!("http://localhost:{:?}", port), caps).await;
    match driver_result {
      Ok(driver) => Ok(Self { driver }),
      Err(e) => Err(e),
    }
  }

  pub async fn screenshot(&self) -> Result<Vec<u8>, WebDriverError> {
    self.driver.screenshot_as_png().await
  }

  // Navigate to a URL
  pub async fn navigate(
    &self,
    url: &str,
  ) -> Result<Page, Box<dyn std::error::Error>> {
    self.driver.get(url).await?;
    // wait for the page to load
    let body = self.driver.query(By::Tag("body")).first().await?;
    body
      .wait_until()
      .has_attribute("data-runtime-theme", "default")
      .await?;

    let title = self.driver.title().await?;

    let session_cookie =
      self.driver.get_named_cookie("session_id_edsby").await?;

    println!("Getting session cookie...");

    if session_cookie.value.len() <= 0 {
      println!("No session cookie found");
    } else {
      println!("SESSION COOKIE: {:?}", session_cookie);
    }

    let source = self.driver.source().await?;
    let screenshot = self.driver.screenshot_as_png().await?;

    Ok(Page {
      url: url.to_string(),
      title,
      source,
      screenshot: Some(screenshot),
    })
  }
  pub async fn close(self) -> Result<(), WebDriverError> {
    self.driver.quit().await
  }
}

#[tokio::main]
pub async fn demo_scraper() {
  let mut webdriver_start = Command::new("geckodriver");
  webdriver_start.spawn().expect("0");
  let start_headless_res = Browser::new(4444).await;
  let browser = match start_headless_res {
    Ok(browser) => browser,
    Err(e) => {
      println!("Error: {:?}", e);
      return;
    }
  };
  let url = "https://recompile.me";

  //let page = browser.navigate(url).await.unwrap();
  let page = browser.navigate(url).await;
  //let page_title = browser.driver.title();
  //let page_source = browser.driver.source();
  //let page_status = browser.driver.status();

  //let title = page.title.clone().to_snake_case();
  //println!("Page: {}", title);
  /*
  // save screenshot to a file
  std::fs::write(
    format!("{:?}.png", title.clone()),
    page.clone().screenshot.unwrap(),
  )
  .unwrap();

  // serialize the page to a JSON string
  std::fs::write(format!("{:?}.json", title.clone()), page.to_string())
    .unwrap();

  std::fs::write(format!("{:?}.html", title.clone()), page.source).unwrap();

   */

  //sleep(std::time::Duration::from_secs(5));

  let close_res = browser.close().await;
  match close_res {
    Ok(_) => println!("Browser closed"),
    Err(e) => println!("Error: {:?}", e),
  }
}
