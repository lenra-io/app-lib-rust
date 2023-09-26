use crate::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

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
    pub(self) params: ApiParam,
    pub data: DataApi,
}

impl Api {
    pub(crate) fn new(api: ApiParam) -> Api {
        let params: ApiParam = api.clone();
        Api {
            params: params.clone(),
            data: DataApi {
                params: params.clone(),
            },
        }
    }
}

impl ApiTrait for Api {
    fn url(&self) -> String {
        self.params.url.clone()
    }

    fn token(&self) -> String {
        self.params.token.clone()
    }
}

pub trait CollectionGetter: ApiTrait + Sized {
    fn coll(&'static self, coll: &str) -> Collection {
        Collection {
            coll: coll.to_string(),
            api: self,
        }
    }
}

pub struct Collection {
    coll: String,
    api: &'static dyn ApiTrait,
}

impl Collection {
    pub fn get_doc<T: Doc>(&self, id: &str) -> Result<T> {
        log::debug!("get_doc {}[{}]", self.coll, id);
        let request_url = format!(
            "{url}/app-api/v1/data/colls/{coll}/docs/{id}",
            url = self.api.url(),
            id = id,
            coll = self.coll
        );

        ureq::get(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token()).as_str(),
            )
            .call()?
            .into_json()
            .map_err(|e| e.into())
    }

    pub fn create_doc<T: Doc>(&self, doc: T) -> Result<T> {
        log::debug!("create_doc {}", serde_json::to_string(&doc).unwrap());

        let request_url = format!(
            "{url}/app-api/v1/data/colls/{coll}/docs",
            url = self.api.url(),
            coll = self.coll
        );

        ureq::post(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token()).as_str(),
            )
            .send_json(doc)?
            .into_json()
            .map_err(|e| e.into())
    }

    pub fn update_doc<T: Doc>(&self, doc: T) -> Result<T> {
        log::debug!("update_doc {}", serde_json::to_string(&doc).unwrap());

        let request_url = format!(
            "{url}/app-api/v1/data/colls/{coll}/docs/{id}",
            url = self.api.url(),
            id = doc.id().unwrap(),
            coll = self.coll
        );

        ureq::put(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token()).as_str(),
            )
            .send_json(doc)?
            .into_json()
            .map_err(|e| e.into())
    }

    pub fn delete_doc<T: Doc>(&self, doc: T) -> Result<()> {
        let request_url = format!(
            "{url}/app-api/v1/data/colls/{coll}/docs/{id}",
            url = self.api.url(),
            id = doc.id().unwrap(),
            coll = self.coll
        );

        ureq::delete(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token()).as_str(),
            )
            .call()?;

        Ok(())
    }

    pub fn find<T: Doc, Q: Serialize, P: Serialize>(
        &self,
        query: Q,
        projection: Option<P>,
    ) -> Result<Vec<T>> {
        log::debug!("find {}", serde_json::to_string(&query).unwrap());
        let request_url = format!(
            "{url}/app-api/v1/data/colls/{coll}/find",
            url = self.api.url(),
            coll = self.coll
        );

        ureq::post(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token()).as_str(),
            )
            .send_json(json!({ "query": query, "projection": projection }))?
            .into_json()
            .map_err(|e| e.into())
    }

    pub fn update_many<T: Doc, Q: Serialize, U: Serialize>(
        &self,
        filter: Q,
        update: U,
    ) -> Result<Vec<T>> {
        log::debug!("updateMany {}, {}", serde_json::to_string(&filter).unwrap(), serde_json::to_string(&update).unwrap());
        let request_url = format!(
            "{url}/app-api/v1/data/colls/{coll}/updateMany",
            url = self.api.url(),
            coll = self.coll
        );

        ureq::post(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token()).as_str(),
            )
            .send_json(json!({"filter": filter, "update": update}))?
            .into_json()
            .map_err(|e| e.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct DataApi {
    pub(self) params: ApiParam,
}

impl DataApi {
    pub fn start_transaction(self) -> Result<Transaction> {
        log::debug!("start_transaction");

        let request_url = format!("{url}/app/transaction", url = self.url());

        ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token()).as_str())
            .send_json(json!({}))?
            .into_string()
            .map(|token| Transaction {
                params: self.params.clone(),
                token,
            })
            .map_err(|e| e.into())
    }
}

impl ApiTrait for DataApi {
    fn url(&self) -> String {
        self.params.url.clone()
    }

    fn token(&self) -> String {
        self.params.token.clone()
    }
}

impl CollectionGetter for DataApi {}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub(self) params: ApiParam,
    token: String,
}

impl ApiTrait for Transaction {
    fn url(&self) -> String {
        self.params.url.clone()
    }

    fn token(&self) -> String {
        self.token.clone()
    }
}

impl CollectionGetter for Transaction {}

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
