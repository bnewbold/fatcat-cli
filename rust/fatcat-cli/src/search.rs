
use serde_json::json;
use std::time::Duration;
use anyhow::{Result, Context, anyhow};
use log::{self,info,debug};
use crate::EntityType;


pub struct SearchResults {
    pub entity_type: EntityType,
    pub limit: Option<u64>,
    pub count: u64,
    pub took_ms: u64,
    offset: u64,
    batch: Vec<serde_json::Value>,
    scroll_id: Option<String>,
    scroll_url: String,
    http_client: reqwest::Client,
}

impl Iterator for SearchResults {
    type Item = Result<serde_json::Value>;

    fn next(&mut self) -> Option<Result<serde_json::Value>> {
        // if we already hit limit, bail early
        if let Some(l) = self.limit {
            if self.offset >= l {
                return None
            }
        }
        // if current batch is empty, and we are scrolling, refill the current batch
        if self.batch.len() == 0 && self.scroll_id.is_some() {
            let response = self.http_client.get(&self.scroll_url)
                .header("Content-Type", "application/json")
                .body(json!({
                    "scroll": "2m",
                    "scroll_id": self.scroll_id.clone().unwrap(),
                }).to_string())
                .send();
            let mut response = match response {
                Err(e) => return Some(Err(e.into())),
                Ok(v) => v,
            };
            if !response.status().is_success() {
                return Some(Err(anyhow!("search error, status={}", response.status())));
            };
            let body: serde_json::Value = match response.json() {
                Err(e) => return Some(Err(e.into())),
                Ok(v) => v,
            };
            self.scroll_id = Some(body["_scroll_id"].as_str().unwrap().to_string());
            self.batch = body["hits"]["hits"].as_array().unwrap().to_vec();
        }

        // return next hit from the most recent batch
        if self.batch.len() > 0 {
            self.offset += 1;
            let val = self.batch.pop().unwrap();
            let source = val["_source"].clone();
            return Some(Ok(source))
        }

        // if batch is empty and couldn't be refilled, terminate
        // TODO: should we raise error if ended early?
        return None
    }
}

pub fn crude_search(api_host: &str, entity_type: EntityType, limit: Option<u64>, terms: Vec<String>) -> Result<SearchResults> {

    let index = match entity_type {
        EntityType::Release => "fatcat_release",
        EntityType::File => "fatcat_file",
        EntityType::Container => "fatcat_container",
        _ => Err(anyhow!("No search index for entity type: {:?}", entity_type))?,
    };
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .danger_accept_invalid_certs(true)
        .build()
        .expect("ERROR :: Could not build reqwest client");

    let query: String = if terms.len() == 0 {
        "*".to_string()
    } else {
        terms.join(" ")
    };
    info!("Search query string: {}", query);
    let request_url = format!("{}/{}/_search", api_host, index);
    let scroll_url = format!("{}/_search/scroll", api_host);

    // sort by _doc for (potentially) very large result sets
    let (scroll_mode, sort_mode, size) = match limit {
        None => (true, "_doc", 100),
        Some(l) if l > 100 => (true, "_doc", 100),
        Some(l) => (false, "_score", l),

    };

    let query_body = json!({
            "query": {
                "boosting": {
                    "positive": {
                        "bool": {
                            "must": {
                                "query_string": {
                                    "query": query,
                                    "default_operator": "AND",
                                    "analyze_wildcard": true,
                                    "allow_leading_wildcard": false,
                                    "lenient": true,
                                    "fields": [
                                        "title^2",
                                        "biblio",
                                    ],
                                },
                            },
                            "should": {
                                "term": { "in_ia": true },
                            },
                        },
                    },
                    "negative": {
                        "bool": {
                            "should": [
                                {"bool": { "must_not" : { "exists": { "field": "title" }}}},
                                {"bool": { "must_not" : { "exists": { "field": "year" }}}},
                                {"bool": { "must_not" : { "exists": { "field": "type" }}}},
                                {"bool": { "must_not" : { "exists": { "field": "stage" }}}},
                            ],
                        },
                    },
                    "negative_boost": 0.5,
                },
            },
            "size": size,
            "sort": [ sort_mode ],
        }).to_string();

    let mut request = http_client.get(&request_url)
        .header("Content-Type", "application/json")
        .body(query_body);

    if scroll_mode {
        request = request.query(&[("scroll", "2m")]);
    }

    let mut response = request.send()?;

    if !response.status().is_success() {
        Err(anyhow!("search error, status={}", response.status()))?;
    }
    //println!("{:?}", response);
    let body: serde_json::Value = response.json()?;

    let scroll_id = match scroll_mode {
        false => None,
        true => Some(body["_scroll_id"].as_str().unwrap().to_string()),
    };

    Ok(SearchResults {
        entity_type,
        limit,
        count: body["hits"]["total"].as_u64().unwrap(),
        took_ms: body["took"].as_u64().unwrap(),
        offset: 0,
        batch: body["hits"]["hits"].as_array().unwrap().to_vec(),
        scroll_id,
        scroll_url,
        http_client,
    })
}
