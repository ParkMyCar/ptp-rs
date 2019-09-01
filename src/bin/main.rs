use ptp_rs::api::{SearchFilter, API};
use ptp_rs::config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config();
    let mut api = API::new(config);

    api.login();

    let filters = SearchFilter {
        name: Some("aquaman".to_string()),
        year: Some("2018".to_string()),
    };
    let search_resp = api.search(&filters);

    let movies = search_resp.movies();
    let movie = movies.get(1).unwrap();
    println!("Title: {:?}, Year: {:?}", movie.title(), movie.year());
    let torrent = movie.torrents().iter().next().unwrap().clone();
    api.download_torrent(&torrent).expect("Could not save torrent file!");

    api.logout();

    Ok(())
}
