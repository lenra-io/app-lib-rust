use crate::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

pub mod app_request;
pub mod internal_api;
pub mod lenra_components;
pub mod manifest;

pub trait ApiTrait {
    fn url(&self) -> String;
    fn token(&self) -> String;
}

#[derive(Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ApiParam {
    pub url: String,
    pub token: String,
}

#[derive(Debug, PartialEq)]
pub struct Api {
    pub data: DataApi,
}

impl Api {
    pub(crate) fn new(params: ApiParam) -> Api {
        Api {
            data: DataApi { api: params },
        }
    }
}

impl ApiTrait for Api {
    fn url(&self) -> String {
        self.data.api.url.clone()
    }

    fn token(&self) -> String {
        self.data.api.token.clone()
    }
}

trait DataApiTrait: ApiTrait {
    fn get_doc<T: Doc>(&self, coll: &str, id: &str) -> Result<T> {
        log::debug!("get_doc {}[{}]", coll, id);
        let request_url = format!(
            "{url}/app/colls/{coll}/docs/{id}",
            url = self.url(),
            id = id
        );

        ureq::get(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token()).as_str())
            .call()?
            .into_json()
            .map_err(|e| e.into())
    }

    fn create_doc<T: Doc>(&self, coll: &str, doc: T) -> Result<T> {
        log::debug!("create_doc {}", serde_json::to_string(&doc).unwrap());

        let request_url = format!("{url}/app/colls/{coll}/docs", url = self.url());

        ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token()).as_str())
            .send_json(doc)?
            .into_json()
            .map_err(|e| e.into())
    }

    fn update_doc<T: Doc>(&self, coll: &str, doc: T) -> Result<T> {
        log::debug!("update_doc {}", serde_json::to_string(&doc).unwrap());

        let request_url = format!(
            "{url}/app/colls/{coll}/docs/{id}",
            url = self.url(),
            id = doc.id().unwrap()
        );

        ureq::put(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token()).as_str())
            .send_json(doc)?
            .into_json()
            .map_err(|e| e.into())
    }

    fn delete_doc<T: Doc>(&self, coll: &str, doc: T) -> Result<()> {
        let request_url = format!(
            "{url}/app/colls/{coll}/docs/{id}",
            url = self.url(),
            id = doc.id().unwrap()
        );

        ureq::delete(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token()).as_str())
            .call()?;

        Ok(())
    }

    fn find<T: Doc, Q: Serialize>(&self, coll: &str, query: Q) -> Result<Vec<T>> {
        log::debug!("find {}", serde_json::to_string(&query).unwrap());
        let request_url = format!("{url}/app/colls/${coll}/docs/find", url = self.url());

        ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token()).as_str())
            .send_json(query)?
            .into_json()
            .map_err(|e| e.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct DataApi {
    api: ApiParam,
}

impl DataApi {
    pub fn start_transaction(&self) -> Result<Transaction> {
        log::debug!("start_transaction");

        let request_url = format!("{url}/app/transaction", url = self.url());

        ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token()).as_str())
            .send_json(json!({}))?
            .into_string()
            .map(|token| Transaction {
                api: self.api.clone(),
                token,
            })
            .map_err(|e| e.into())
    }
}

impl ApiTrait for DataApi {
    fn url(&self) -> String {
        self.api.url.clone()
    }

    fn token(&self) -> String {
        self.api.token.clone()
    }
}

impl DataApiTrait for DataApi {}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    api: ApiParam,
    token: String,
}

impl ApiTrait for Transaction {
    fn url(&self) -> String {
        self.api.url.clone()
    }

    fn token(&self) -> String {
        self.token.clone()
    }
}

impl DataApiTrait for Transaction {}

impl Transaction {
    pub fn commit(&self) -> Result<()> {
        log::debug!("transaction commit");

        let request_url = format!("{url}/app/transaction/commit", url = self.url());

        ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token()).as_str())
            .send_json(json!({}))?
            .into_string()
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub fn abort(&self) -> Result<()> {
        log::debug!("transaction commit");

        let request_url = format!("{url}/app/transaction/abort", url = self.url());

        ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token()).as_str())
            .send_json(json!({}))?
            .into_string()
            .map(|_| ())
            .map_err(|e| e.into())
    }
}

pub trait Doc: Sized + DeserializeOwned + Serialize + 'static + Clone {
    fn id(&self) -> Option<String>;
}

impl Doc for Value {
    fn id(&self) -> Option<String> {
        None
    }
}
