use scraper::{Html, Selector};
use std::env;

#[tokio::main]
async fn main() {

    for argument in env::args().skip(1) {
        let mut urls = Vec::new();
        for n in 1..10 {
            urls.push(format!("https://letterboxd.com/{argument}/films/page/{n}"));
        }

        let mut handles = Vec::new();
        for url in urls {
            handles.push(tokio::spawn(async move {
                fetch_movies(&url).await
            }));
        }

        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await.unwrap())
        }

        let result = results.into_iter().flatten().flatten().collect::<Vec<_>>();
        println!("{result:?}")
    }
}

async fn fetch_movies(url: &str) -> Result<Vec<(String, String)>, reqwest::Error> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);
    let poster_selector = Selector::parse("li.poster-container").unwrap();
    let div_selector = Selector::parse("div").unwrap();
    let rating_selector = Selector::parse("span.rating").unwrap();

    let mut films = Vec::new();

    for film_node in document.select(&poster_selector) {
        if let Some(div_data) = film_node.select(&div_selector).next() {
            let name = div_data
                .value()
                .attr("data-film-slug")
                .unwrap_or("")
                .to_string();

            let rating = film_node
                .select(&rating_selector)
                .next()
                .and_then(|e| e.text().next())
                .unwrap_or("")
                .to_string();

            films.push((name, rating));
        }
    }

    Ok(films)
}
