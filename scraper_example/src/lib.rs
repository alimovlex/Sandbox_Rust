// custom struct to store the scraping data
struct Country {
    name: String,
    capital: String,
    population: String,
    area: String,
}

// define a custom data structure to store the scraped data
struct Product {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}

pub fn scrap_example() -> Result<(), Box<dyn std::error::Error>> {
    // connect to the target page
    let response = reqwest::blocking::get("https://www.scrapethissite.com/pages/simple/")?;
    // extract the raw html and print it
    let html = response.text()?;
    // parse the HTML document
    println!("{}", html);
    let document = scraper::Html::parse_document(&html);
    // where to store the scraped data
    let mut countries: Vec<Country> = Vec::new();

    // select the country info box HTML elements
    let html_country_info_box_selector = scraper::Selector::parse(".country")?;
    let html_country_info_box_elements = document.select(&html_country_info_box_selector);

    // iterate over the country HTML elements and scrape them all
    for html_country_info_box_element in html_country_info_box_elements {
        // scraping logic for a single country info box HTML element
        let country_name_selector = scraper::Selector::parse(".country-name")?;
        let name = html_country_info_box_element
            .select(&country_name_selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_owned())
            .ok_or("Country name not found")?;

        let country_capital_selector = scraper::Selector::parse(".country-capital")?;
        let capital = html_country_info_box_element
            .select(&country_capital_selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_owned())
            .ok_or("Country capital not found")?;

        let country_population_selector = scraper::Selector::parse(".country-population")?;
        let population = html_country_info_box_element
            .select(&country_population_selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_owned())
            .ok_or("Country population not found")?;

        let country_area_selector = scraper::Selector::parse(".country-area")?;
        let area = html_country_info_box_element
            .select(&country_area_selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_owned())
            .ok_or("Country area not found")?;

        // create a new Country object and add it to the vector
        let country = Country {
            name,
            capital,
            population,
            area,
        };
        countries.push(country);
    }

    // initialize the output CSV file
    let mut writer = csv::Writer::from_path("countries.csv")?;
    // write the CSV header
    writer.write_record(&["name", "capital", "population", "area"])?;
    // populate the file with each country
    for country in countries {
        writer.write_record(&[
            country.name,
            country.capital,
            country.population,
            country.area,
        ])?;
    }
    Ok(())
}

pub fn scrap_sample() {
    // initialize the vector that will store the scraped data
    let mut products: Vec<Product> = Vec::new();
    // download the target HTML document
    let response = reqwest::blocking::get("https://www.scrapingcourse.com/ecommerce/");
    // get the HTML content from the request response
    let html_content = response.unwrap().text().unwrap();
    println!("{}", html_content);
    // parse the HTML document
    let document = scraper::Html::parse_document(&html_content);

    // define the CSS selector to get all product
    // on the page
    let html_product_selector = scraper::Selector::parse("li.product").unwrap();
    // apply the CSS selector to get all products
    let html_products = document.select(&html_product_selector);

    // iterate over each HTML product to extract data
    // from it
    for html_product in html_products {
        // scraping logic to retrieve the info
        // of interest
        let url = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let image = html_product
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);
        let name = html_product
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
        let price = html_product
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());

        // instanciate a new product
        // with the scraped data and add it to the list
        let product = Product {
            url,
            image,
            name,
            price,
        };

        products.push(product);
    }
    // create the CSV output file
    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    // append the header to the CSV
    writer
        .write_record(&["url", "image", "name", "price"])
        .unwrap();

    // populate the output file
    for product in products {
        let url = product.url.unwrap();
        let image = product.image.unwrap();
        let name = product.name.unwrap();
        let price = product.price.unwrap();
        writer.write_record(&[url, image, name, price]).unwrap();
    }

    // free up the writer resources
    writer.flush().unwrap();
}

