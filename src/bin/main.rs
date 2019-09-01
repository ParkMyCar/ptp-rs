use ptp_rs::config;
use ptp_rs::api::{API, SearchFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config();
    let mut api = API::new(config);
    
    api.login();

    let filters = SearchFilter {
        name: Some("aquaman".to_string()),
        year: None/*Some("2013".to_string())*/,
    };
    let search_resp = api.search(&filters);

    for movie in search_resp.movies() {
        println!("Title: {:?}, Year: {:?}", movie.title(), movie.year());
        let torrent = movie.torrents().iter().next().unwrap().clone();
        api.download_torrent(&torrent);
    }

    api.logout();

    Ok(())
}