//! Server API client (creator client).
//!
//! Used by creators to access their campaigns, members, posts, and webhooks.
//! These APIs require a creator `access_token` (Creator's Access Token from Patreon developer portal).
//!
//! ## Typical use cases
//! - Fetch campaigns on the server side
//! - List members/patrons
//! - Manage posts
//! - Handle webhooks
//!
//! ## Available APIs
//! - `/api/oauth2/v2/campaigns` - list all campaigns
//! - `/api/oauth2/v2/campaigns/{campaign_id}` - fetch campaign details
//! - `/api/oauth2/v2/campaigns/{campaign_id}/members` - list members
//! - `/api/oauth2/v2/campaigns/{campaign_id}/posts` - list posts
//! - `/api/oauth2/v2/members/{member_id}` - fetch a member
//! - `/api/oauth2/v2/posts/{post_id}` - fetch a post
//! - `/api/oauth2/v2/webhooks` - manage webhooks

use std::borrow::Cow;

use crate::models::*;
use crate::{API_BASE_URL, Error, Result};
use bon::bon;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Serialize;

/// Patreon creator (server) API client.
///
/// Used by creators to access their campaigns, members, posts, and webhooks.
///
/// # Example
///
/// ```rust,ignore
/// use patreon::PatreonCreatorClient;
///
/// // Using a creator access token
/// let client = PatreonCreatorClient::new("creator_access_token");
///
/// // List campaigns
/// let campaigns = client.campaigns().await?;
///
/// // List members for a campaign
/// let members = client.campaign_members("campaign_id").await?;
/// ```
#[derive(Debug, Clone)]
pub struct PatreonCreatorClient {
    access_token: String,
    http_client: reqwest::Client,
    base_url: String,
}

/// Parameters for creating a webhook.
#[derive(Debug, Clone, Serialize)]
pub struct CreateWebhookRequest {
    /// Webhook URL.
    pub uri: String,
    /// Campaign ID.
    pub campaign_id: String,
    /// Trigger list.
    pub triggers: Vec<String>,
}

/// Webhook request body (JSON:API format).
#[derive(Debug, Clone, Serialize)]
struct WebhookRequestBody {
    data: WebhookRequestData,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookRequestData {
    #[serde(rename = "type")]
    resource_type: String,
    attributes: WebhookRequestAttributes,
    relationships: WebhookRequestRelationships,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookRequestAttributes {
    uri: String,
    triggers: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookRequestRelationships {
    campaign: WebhookCampaignRelationship,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookCampaignRelationship {
    data: WebhookCampaignData,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookCampaignData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

#[bon]
impl PatreonCreatorClient {
    /// Creates a new creator client.
    ///
    /// # Parameters
    /// - `access_token`: creator access token
    ///   (available from <https://www.patreon.com/portal/registration/register-clients>)
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            http_client: reqwest::Client::new(),
            base_url: API_BASE_URL.to_string(),
        }
    }

    /// Uses a custom `reqwest::Client`.
    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = client;
        self
    }

    /// Uses a custom base URL (useful for tests).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Builds authorization headers.
    fn auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.access_token)).expect("Invalid token"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    /// Sends a GET request.
    async fn get<T: serde::de::DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http_client
            .get(&url)
            .headers(self.auth_headers())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: text,
            })
        }
    }

    /// Sends a POST request.
    async fn post_request<T: serde::de::DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers())
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: text,
            })
        }
    }

    /// Sends a PATCH request.
    async fn patch<T: serde::de::DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http_client
            .patch(&url)
            .headers(self.auth_headers())
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: text,
            })
        }
    }

    /// Sends a DELETE request.
    async fn delete(&self, endpoint: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http_client
            .delete(&url)
            .headers(self.auth_headers())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: text,
            })
        }
    }

    // ==================== Campaigns API ====================

    /// Lists campaigns with details.
    ///
    /// Returns detailed campaign information including creator info.
    /// # Parameters
    /// - `fields`: campaign fields
    /// - `includes`: top-level includes
    /// # Required scopes
    /// - `campaigns`
    #[builder]
    pub async fn campaigns(
        &self,
        fields: Option<CampaignFields>,
        includes: Option<CampaignIncludes>,
    ) -> Result<ListResponse<CampaignResource>> {
        self.get(format!("/campaigns{}", (fields, includes).query_params()).as_str())
            .await
    }

    /// Fetches a specific campaign.
    ///
    /// # Parameters
    /// - `id`: campaign ID
    /// - `fields`: campaign fields
    /// - `includes: top-level includes
    #[builder]
    pub async fn campaign(
        &self,
        #[builder(start_fn)] id: &str,
        fields: Option<CampaignFields>,
        includes: Option<CampaignIncludes>,
    ) -> Result<SingleResponse<CampaignResource>> {
        self.get(format!("/campaigns/{}{}", id, (fields, includes).query_params()).as_str())
            .await
    }

    // ==================== Members API ====================

    /// Lists campaign members including related resources with pagination.
    ///
    /// # Parameters
    /// - `campaign_id`: campaign ID
    /// - `member_fields: member fields
    /// - `includes`: top-level includes
    /// - `query`: query parameters
    /// # Required scopes
    /// - `campaigns.members`
    #[builder]
    pub async fn campaign_members(
        &self,
        #[builder(start_fn)] campaign_id: &str,
        fields: Option<MemberFields>,
        includes: Option<MemberIncludes>,
        query: Option<MembersQuery>,
    ) -> Result<ListResponse<MemberResource>> {
        self.get(
            format!(
                "/campaigns/{}/members{}",
                campaign_id,
                (fields, includes, query).query_params()
            )
            .as_str(),
        )
        .await
    }

    /// Fetches a specific member.
    ///
    /// # Parameters
    /// - `id`: member ID
    /// - `fields`: member fields
    /// - `includes`: top-level includes
    #[builder]
    pub async fn member(
        &self,
        #[builder(start_fn)] id: &str,
        fields: Option<MemberFields>,
        includes: Option<MemberIncludes>,
    ) -> Result<SingleResponse<MemberResource>> {
        self.get(format!("/members/{}{}", id, (fields, includes).query_params()).as_str())
            .await
    }

    // ==================== Posts API ====================

    /// Lists campaign posts with pagination.
    ///
    /// # Parameters
    /// - `campaign_id`: campaign ID
    /// - `fields`: post fields
    /// - `includes`: top-level includes
    /// - `query`: query parameters
    /// # Required scopes
    /// - `campaigns.posts`
    #[builder]
    pub async fn campaign_posts(
        &self,
        #[builder(start_fn)] campaign_id: &str,
        fields: Option<PostFields>,
        includes: Option<PostIncludes>,
        query: Option<PostsQuery>,
    ) -> Result<ListResponse<PostResource>> {
        self.get(
            format!(
                "/campaigns/{}/posts{}",
                campaign_id,
                (fields, includes, query).query_params()
            )
            .as_str(),
        )
        .await
    }

    /// Fetches a specific post.
    ///
    /// # Parameters
    /// - `id`: post ID
    /// - `fieldws`: post fields
    /// - `includes`: top-level includes
    #[builder]
    pub async fn post(
        &self,
        #[builder(start_fn)] id: &str,
        fields: Option<PostFields>,
        includes: Option<PostIncludes>,
    ) -> Result<SingleResponse<PostResource>> {
        self.get(format!("/posts/{}{}", id, (fields, includes).query_params()).as_str())
            .await
    }

    // ==================== Webhooks API ====================

    /// Lists all webhooks.
    ///
    /// # Required scopes
    /// - `w:campaigns.webhook`
    pub async fn webhooks(&self) -> Result<ListResponse<WebhookResource>> {
        self.get("/webhooks").await
    }

    /// Creates a webhook.
    ///
    /// # Parameters
    /// - `request`: webhook creation parameters
    ///
    /// # Trigger types
    /// - `members:create` - member created
    /// - `members:update` - member updated
    /// - `members:delete` - member deleted
    /// - `members:pledge:create` - pledge created
    /// - `members:pledge:update` - pledge updated
    /// - `members:pledge:delete` - pledge deleted
    /// - `posts:publish` - post published
    /// - `posts:update` - post updated
    /// - `posts:delete` - post deleted
    pub async fn create_webhook(
        &self,
        request: &CreateWebhookRequest,
    ) -> Result<SingleResponse<WebhookResource>> {
        let body = WebhookRequestBody {
            data: WebhookRequestData {
                resource_type: "webhook".to_string(),
                attributes: WebhookRequestAttributes {
                    uri: request.uri.clone(),
                    triggers: request.triggers.clone(),
                },
                relationships: WebhookRequestRelationships {
                    campaign: WebhookCampaignRelationship {
                        data: WebhookCampaignData {
                            resource_type: "campaign".to_string(),
                            id: request.campaign_id.clone(),
                        },
                    },
                },
            },
        };

        self.post_request("/webhooks", &body).await
    }

    /// Updates a webhook.
    ///
    /// # Parameters
    /// - `webhook_id`: webhook ID
    /// - `uri`: new webhook URL (optional)
    /// - `triggers`: new trigger list (optional)
    /// - `paused`: whether the webhook is paused (optional)
    pub async fn update_webhook(
        &self,
        webhook_id: &str,
        uri: Option<&str>,
        triggers: Option<&[&str]>,
        paused: Option<bool>,
    ) -> Result<SingleResponse<WebhookResource>> {
        #[derive(Serialize)]
        struct UpdateBody {
            data: UpdateData,
        }

        #[derive(Serialize)]
        struct UpdateData {
            #[serde(rename = "type")]
            resource_type: String,
            id: String,
            attributes: UpdateAttributes,
        }

        #[derive(Serialize)]
        struct UpdateAttributes {
            #[serde(skip_serializing_if = "Option::is_none")]
            uri: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            triggers: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            paused: Option<bool>,
        }

        let body = UpdateBody {
            data: UpdateData {
                resource_type: "webhook".to_string(),
                id: webhook_id.to_string(),
                attributes: UpdateAttributes {
                    uri: uri.map(String::from),
                    triggers: triggers.map(|t| t.iter().map(|s| s.to_string()).collect()),
                    paused,
                },
            },
        };

        self.patch(&format!("/webhooks/{}", webhook_id), &body)
            .await
    }

    /// Deletes a webhook.
    ///
    /// # Parameters
    /// - `webhook_id`: webhook ID
    pub async fn delete_webhook(&self, webhook_id: &str) -> Result<()> {
        self.delete(&format!("/webhooks/{}", webhook_id)).await
    }
}

trait Params {
    fn query_params(&self) -> Cow<'static, str>;
}

impl<F: Fields, I: Includes> Params for (Option<F>, Option<I>) {
    fn query_params(&self) -> Cow<'static, str> {
        let (fields, includes) = self;
        match (fields, includes) {
            (None, None) => Cow::Borrowed(""),
            (None, Some(includes)) => Cow::Owned(format!("?{}", includes.include())),
            (Some(fields), None) => Cow::Owned(format!("?{}", fields.fields())),
            (Some(fields), Some(includes)) => Cow::Owned(format!(
                "?{}&{}&{}",
                includes.include(),
                fields.fields(),
                includes.fields()
            )),
        }
    }
}

impl<F: Fields, I: Includes, Q: Query> Params for (Option<F>, Option<I>, Option<Q>) {
    fn query_params(&self) -> Cow<'static, str> {
        let (fields, includes, query) = self;
        match (fields, includes, query) {
            (None, None, None) => Cow::Borrowed(""),
            (fields, includes, query) => Cow::Owned(format!(
                "?{}",
                [
                    includes.as_ref().map(Includes::include),
                    fields.as_ref().map(Fields::fields),
                    includes.as_ref().map(Includes::fields),
                    query.as_ref().map(Query::query_params).flatten()
                ]
                .into_iter()
                .filter_map(|s| s)
                .collect::<Vec<_>>()
                .join("&")
            )),
        }
    }
}

/// Webhook trigger string constants.
pub mod webhook_triggers {
    pub const MEMBERS_CREATE: &str = "members:create";
    pub const MEMBERS_UPDATE: &str = "members:update";
    pub const MEMBERS_DELETE: &str = "members:delete";
    pub const MEMBERS_PLEDGE_CREATE: &str = "members:pledge:create";
    pub const MEMBERS_PLEDGE_UPDATE: &str = "members:pledge:update";
    pub const MEMBERS_PLEDGE_DELETE: &str = "members:pledge:delete";
    pub const POSTS_PUBLISH: &str = "posts:publish";
    pub const POSTS_UPDATE: &str = "posts:update";
    pub const POSTS_DELETE: &str = "posts:delete";
}

#[cfg(test)]
mod tests {
    use std::{env, sync::OnceLock};

    use dotenvy::dotenv;
    use tokio::sync::OnceCell;

    use super::*;

    #[test]
    fn test_new_creator_client() {
        let client = PatreonCreatorClient::new("test_token");
        assert_eq!(client.access_token, "test_token");
    }

    fn client() -> &'static PatreonCreatorClient {
        static CLIENT: OnceLock<PatreonCreatorClient> = OnceLock::new();
        CLIENT.get_or_init(|| {
            dotenv().ok();
            let access_token = env::var("PATREON_CREATOR_ACCESS_TOKEN").unwrap();
            PatreonCreatorClient::new(access_token)
        })
    }

    async fn campaign() -> &'static CampaignResource {
        static CAMPAIGN: OnceCell<CampaignResource> = OnceCell::const_new();
        CAMPAIGN
            .get_or_init(async || {
                let client = client();
                let campaigns = client.campaigns().call().await.unwrap();
                campaigns.data.into_iter().next().unwrap()
            })
            .await
    }

    async fn member() -> &'static MemberResource {
        static MEMBER: OnceCell<MemberResource> = OnceCell::const_new();
        MEMBER
            .get_or_init(async || {
                let client = client();
                let campaign = campaign().await;
                let members = client.campaign_members(&campaign.id).call().await.unwrap();
                members.data.into_iter().next().unwrap()
            })
            .await
    }

    async fn post() -> &'static PostResource {
        static POST: OnceCell<PostResource> = OnceCell::const_new();
        POST.get_or_init(async || {
            let client = client();
            let campaign = campaign().await;
            let posts = client.campaign_posts(&campaign.id).call().await.unwrap();
            posts.data.into_iter().next().unwrap()
        })
        .await
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaigns() {
        let client = client();
        client.campaigns().call().await.unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaigns_with_details() {
        let client = client();
        client
            .campaigns()
            .fields(CampaignFields::all())
            .includes(CampaignIncludes {
                creator: Some(UserFields::all()),
                ..Default::default()
            })
            .call()
            .await
            .unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaign() {
        campaign().await;
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaign_with_tiers_and_benefits() {
        let client = client();
        let campaign = campaign().await;
        client
            .campaign(&campaign.id)
            .fields(CampaignFields::all())
            .includes(CampaignIncludes::all())
            .call()
            .await
            .unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaign_members() {
        let client = client();
        let campaign = campaign().await;
        client.campaign_members(&campaign.id).call().await.unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaign_members_with_query() {
        let client = client();
        let campaign = campaign().await;
        client
            .campaign_members(&campaign.id)
            .query(MembersQuery {
                cursor: None,
                page_size: None,
            })
            .call()
            .await
            .unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaign_members_with_details() {
        let client = client();
        let campaign = campaign().await;
        client
            .campaign_members(&campaign.id)
            .fields(MemberFields::all())
            .includes(MemberIncludes {
                address: Some(AddressFields::all()),
                currently_entitled_tiers: Some(TierFields::all()),
                user: Some(UserFields::all()),
                ..Default::default()
            })
            .call()
            .await
            .unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaign_members_with_details_and_query() {
        let client = client();
        let campaign = campaign().await;
        client
            .campaign_members(&campaign.id)
            .fields(MemberFields::all())
            .includes(MemberIncludes {
                address: Some(AddressFields::all()),
                currently_entitled_tiers: Some(TierFields::all()),
                user: Some(UserFields::all()),
                ..Default::default()
            })
            .query(MembersQuery {
                cursor: None,
                page_size: None,
            })
            .call()
            .await
            .unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_member() {
        member().await;
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_member_with_details() {
        let client = client();
        let member = member().await;
        client
            .member(&member.id)
            .fields(MemberFields::all())
            .includes(MemberIncludes::all())
            .call()
            .await
            .unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaign_posts() {
        let client = client();
        let campaign = campaign().await;
        client.campaign_posts(&campaign.id).call().await.unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaign_posts_with_query() {
        let client = client();
        let campaign = campaign().await;
        client
            .campaign_posts(&campaign.id)
            .query(PostsQuery {
                cursor: None,
                page_size: None,
            })
            .call()
            .await
            .unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_campaign_posts_with_details() {
        let client = client();
        let campaign = campaign().await;
        client
            .campaign_posts(&campaign.id)
            .fields(PostFields::all())
            .includes(PostIncludes::all())
            .call()
            .await
            .unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_post() {
        post().await;
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_post_with_details() {
        let client = client();
        let post = post().await;
        client
            .post(&post.id)
            .fields(PostFields::all())
            .includes(PostIncludes::all())
            .call()
            .await
            .unwrap();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_webhooks() {
        let client = client();
        client.webhooks().await.unwrap();
    }
}
