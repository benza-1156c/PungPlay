use rusty_ytdl::search::{YouTube, SearchOptions, SearchType};

#[tokio::main]
async fn main() {
    let yt = YouTube::new().unwrap();
    let opts = SearchOptions {
        limit: 2,
        search_type: SearchType::Channel,
        ..Default::default()
    };
    let res = yt.search("กาย หงิด", Some(&opts)).await.unwrap();
    for r in res {
        println!("{:#?}", r);
    }
}
