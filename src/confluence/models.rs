use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    pub status: String,
    pub title: String,
    pub space_id: Option<String>,
    pub parent_id: Option<String>,
    pub author_id: Option<String>,
    pub created_at: Option<String>,
    pub version: Option<Version>,
    pub body: Option<PageBody>,
    #[serde(rename = "_links")]
    pub links: Option<PageLinks>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageLinks {
    #[serde(rename = "webui")]
    pub web_ui: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    pub number: u32,
    pub message: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body {
    pub representation: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageBody {
    pub storage: Option<Body>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePageRequest {
    pub space_id: String,
    pub status: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub body: Body,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePageRequest {
    pub id: String,
    pub status: String,
    pub title: String,
    pub body: Body,
    pub version: VersionUpdate,
}

#[derive(Debug, Serialize)]
pub struct VersionUpdate {
    pub number: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl CreatePageRequest {
    pub fn new(space_id: String, title: String, body_value: String) -> Self {
        Self {
            space_id,
            status: "current".to_string(),
            title,
            parent_id: None,
            body: Body {
                representation: "storage".to_string(),
                value: body_value,
            },
        }
    }

    pub fn parent_id(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }
}

impl UpdatePageRequest {
    pub fn new(id: String, title: String, body_value: String, version_number: u32) -> Self {
        Self {
            id,
            status: "current".to_string(),
            title,
            body: Body {
                representation: "storage".to_string(),
                value: body_value,
            },
            version: VersionUpdate {
                number: version_number + 1,
                message: None,
            },
        }
    }

    pub fn version_message(mut self, message: String) -> Self {
        self.version.message = Some(message);
        self
    }
}
