mod hn;
use std::fs::OpenOptions;
use std::io::Write;
fn main() {
    println!("Hacker News Trending Crawler...");

    let mut hn = hn::HackerNewsAPI::new();
    let db = hn::db::Database::new();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("../dist/hn.json")
        .unwrap();
    hn.get_top_stories(100).unwrap().iter().for_each(|story| {
        println!("story by {} with title '{}'", story.by, story.title);
        db.save_story(&story);
        let content = format!(
            "{{ \"title\": \"{}\", \"text\": \"{}\" }}\n",
            &story.title, &story.text
        );
        file.write_all(content.as_bytes()).unwrap();
    });

    println!("===========\nCreated: dist/hn.json");
}
