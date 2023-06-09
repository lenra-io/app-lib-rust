use crate::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug, PartialEq, Default)]
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

#[derive(Debug, PartialEq)]
pub struct DataApi {
    api: ApiParam,
}

impl DataApi {
    pub fn get_doc<T: Doc>(&self, coll: &str, id: &str) -> Result<T> {
        log::debug!("get_doc {}[{}]", coll, id);
        let request_url = format!(
            "{url}/app/colls/{coll}/docs/{id}",
            url = self.api.url,
            id = id
        );

        ureq::get(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token).as_str(),
            )
            .call()?
            .into_json()
            .map_err(|e| e.into())
    }

    pub fn create_doc<T: Doc>(&self, coll: &str, doc: T) -> Result<T> {
        log::debug!("create_doc {}", serde_json::to_string(&doc).unwrap());

        let request_url = format!("{url}/app/colls/{coll}/docs", url = self.api.url);

        ureq::post(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token).as_str(),
            )
            .send_json(doc)?
            .into_json()
            .map_err(|e| e.into())
    }

    pub fn update_doc<T: Doc>(&self, coll: &str, doc: T) -> Result<T> {
        log::debug!("update_doc {}", serde_json::to_string(&doc).unwrap());

        let request_url = format!(
            "{url}/app/colls/{coll}/docs/{id}",
            url = self.api.url,
            id = doc.id().unwrap()
        );

        ureq::put(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token).as_str(),
            )
            .send_json(doc)?
            .into_json()
            .map_err(|e| e.into())
    }

    pub fn delete_doc<T: Doc>(&self, coll: &str, doc: T) -> Result<()> {
        let request_url = format!(
            "{url}/app/colls/{coll}/docs/{id}",
            url = self.api.url,
            id = doc.id().unwrap()
        );

        ureq::delete(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token).as_str(),
            )
            .call()?;

        Ok(())
    }

    pub fn find<T: Doc, Q: Serialize>(&self, coll: &str, query: Q) -> Result<Vec<T>> {
        log::debug!("find {}", serde_json::to_string(&query).unwrap());
        let request_url = format!("{url}/app/colls/${coll}/docs/find", url = self.api.url);

        ureq::post(request_url.as_str())
            .set(
                "Authorization",
                format!("Bearer {}", self.api.token).as_str(),
            )
            .send_json(query)?
            .into_json()
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
