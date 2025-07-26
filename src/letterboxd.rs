use scraper::{Html, Selector};
use serde_json::{json, Value};

pub(crate) async fn fetch_movies(account: String) -> Result<Value, reqwest::Error> {
    let mut urls = Vec::new();
    for n in 1..10 {
        urls.push(format!("https://letterboxd.com/{account}/films/page/{n}"));
    }

    let mut handles = Vec::new();
    for url in urls {
        handles.push(tokio::spawn(async move {
            fetch_movies_for_url(&url).await
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap())
    }

    let flattened = results.into_iter().flatten().flatten();

    let result = json!(flattened.map(|(name, rating, poster_url, year)| {
        json!({
            "title": name,
            "rating": rating,
            "poster_url": poster_url,
            "year": year
        })
    }).collect::<Vec<_>>());

    Ok(result)
}

async fn fetch_movies_for_url(url: &str) -> Result<Vec<(String, u8, String, u16)>, reqwest::Error> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);
    let poster_selector = Selector::parse("li.poster-container").unwrap();
    let div_selector = Selector::parse("div").unwrap();
    let rating_selector = Selector::parse("span.rating").unwrap();
    let img_selector = Selector::parse("img").unwrap();

    let mut films = Vec::new();

    for film_node in document.select(&poster_selector) {
        if let Some(div_data) = film_node.select(&div_selector).next() {
            // Get movie title from image alt attribute (more reliable than data-film-slug)
            let title = film_node
                .select(&img_selector)
                .next()
                .and_then(|img| img.value().attr("alt"))
                .unwrap_or("")
                .to_string();

            // Get year from data attribute
            let year = div_data
                .value()
                .attr("data-film-release-year")
                .and_then(|y| y.parse::<u16>().ok())
                .unwrap_or(0);

            let rating = film_node
                .select(&rating_selector)
                .next()
                .and_then(|e| e.text().next())
                .unwrap_or("")
                .to_string();

            let poster_url = film_node
                .select(&img_selector)
                .next()
                .and_then(|img| img.value().attr("src"))
                .unwrap_or("")
                .to_string();

            films.push((title, rating_string_to_u8(rating), poster_url, year));
        }
    }

    Ok(films)
}

fn rating_string_to_u8(rating: String) -> u8 {
    match rating.as_str() {
        "★★★★★" => 10,
        "★★★★½" => 9,
        "★★★★" => 8,
        "★★★½" => 7,
        "★★★" => 6,
        "★★½" => 5,
        "★★" => 4,
        "★½" => 3,
        "★" => 2,
        "½" => 1,
        _ => 0
    }
}