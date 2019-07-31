extern crate reqwest;
use serde::{Deserialize, Serialize};

pub struct HackerNewsAPI {
    api_root_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Story {
    pub by: String,
    pub id: i32,
    time: i64,
    pub title: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    descendants: i32,
    #[serde(default)]
    pub kids: Vec<i32>,
    #[serde(default)]
    parts: Vec<i32>,
    #[serde(default)]
    score: i32,
}

impl HackerNewsAPI {
    pub fn new() -> HackerNewsAPI {
        HackerNewsAPI {
            api_root_url: String::from("https://hacker-news.firebaseio.com/v0"),
        }
    }
    pub fn get_top_stories(&mut self, top_n: usize) -> Result<Vec<Story>, Box<std::error::Error>> {
        let stories_ids = self.get_top_stories_ids();

        let res = stories_ids
            .unwrap()
            .iter()
            .take(top_n)
            .map(|id| {
                let request_uri = format!("{}/item/{}.json", self.api_root_url, id);
                let text: String = reqwest::get(&request_uri).unwrap().text().unwrap();
                let result: Story = serde_json::from_str(&text).unwrap();
                result
            })
            .collect();

        Ok(res)
    }

    fn get_top_stories_ids(&mut self) -> Result<Vec<i32>, Box<std::error::Error>> {
        let request_uri = format!("{}/topstories.json", self.api_root_url);
        let body: String = reqwest::get(&request_uri)?.text()?;
        let results: Vec<i32> = serde_json::from_str(&body)?;
        Ok(results)
    }
}

pub mod db {
    extern crate mongodb;
    use crate::hn::db::mongodb::db::ThreadedDatabase;
    use mongodb::{bson, doc, Bson};
    use mongodb::{Client, ThreadedClient};
    extern crate chrono;
    use chrono::prelude::*;
    use chrono::Utc;

    pub struct Database {
        client: Client,
    }

    impl Database {
        pub fn new() -> Database {
            let client = Client::connect("dvm5074.cloudapp.net", 27017)
                .expect("Failed to initialize standalone client.");

            let db = client.db("hacker-news");
            db.auth("crawler", "5rXW8dllBT0qutXL0qdkM0VDIEfXzCXd1xxlNszitU5177DBs8oGSVQr4pwG7IlU0UuI2RxeXH64Htehk9n7Vw==").unwrap();
            Database { client: client }
        }

        pub fn print_records(&self) {
            let coll = self.client.db("hacker-news").collection("top_stories");
            let cursor = coll.find(None, None).unwrap();
            for result in cursor {
                if let Ok(item) = result {
                    println!("DB Item: {:?}", item);
                    if let Some(&Bson::String(ref title)) = item.get("title") {
                        println!("title: {}", title);
                    }
                }
            }
        }

        pub fn save_story(&self, story: &super::Story) {
            let utc: DateTime<Utc> = Utc::now();
            let doc = doc! {
                "id": &story.id.to_string(),
                "by": &story.by,
                "title": &story.title,
                "text": &story.text,
                "lastModified": utc.to_string()
            };
            let coll = self.client.db("hacker-news").collection("top_stories");
            coll.insert_one(doc, None).unwrap();
            println!("{:?}", &story.text);
        }
    }

}
