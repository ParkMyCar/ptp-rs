use ptp_rs::config;
use ptp_rs::api::{API, SearchFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config();
    let mut api = API::new(config);
    
    api.login();

    let filters = SearchFilter {
        name: Some("star trek into darkness".to_string()),
        year: Some("2013".to_string()),
    };
    let search_resp = api.search(filters);
    println!("{:#?}", search_resp.movies().pop().unwrap().Title);

    for torrent in search_resp.movies().pop().unwrap().torrents() {
        println!("{}GB", torrent.gb());
    }

    api.logout();

    Ok(())
}