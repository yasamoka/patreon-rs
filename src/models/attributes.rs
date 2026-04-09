//! Patreon API resource attributes.
//!
//! Each resource type has a corresponding attributes struct.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============== User ==============

/// User attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserAttributes {
    /// Email address.
    pub email: Option<String>,

    /// Full name.
    pub full_name: Option<String>,

    /// First name.
    pub first_name: Option<String>,

    /// Last name.
    pub last_name: Option<String>,

    /// Vanity username.
    pub vanity: Option<String>,

    /// Bio/about text.
    pub about: Option<String>,

    /// Avatar image URL.
    pub image_url: Option<String>,

    /// Thumbnail URL.
    pub thumb_url: Option<String>,

    /// Patreon profile URL.
    pub url: Option<String>,

    /// Whether the user is a creator.
    pub is_creator: Option<bool>,

    /// Whether the email is verified.
    pub is_email_verified: Option<bool>,

    /// Account creation time.
    pub created: Option<DateTime<Utc>>,

    /// Whether pledges are hidden.
    pub hide_pledges: Option<bool>,

    /// Like count.
    pub like_count: Option<i32>,

    /// Social connections.
    pub social_connections: Option<serde_json::Value>,
}

// ============== Campaign ==============

/// Campaign attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CampaignAttributes {
    /// Campaign creation time.
    pub created_at: Option<DateTime<Utc>>,

    /// Creation name / what the creator makes.
    pub creation_name: Option<String>,

    /// Discord server ID.
    pub discord_server_id: Option<String>,

    /// Google Analytics ID
    pub google_analytics_id: Option<String>,

    /// Whether the campaign charges immediately.
    pub is_charged_immediately: Option<bool>,

    /// Whether the campaign charges monthly.
    pub is_monthly: Option<bool>,

    /// Whether the campaign is marked NSFW.
    pub is_nsfw: Option<bool>,

    /// Main image URL.
    pub image_url: Option<String>,

    /// Small main image URL.
    pub image_small_url: Option<String>,

    /// Cover photo URL.
    pub cover_photo_url: Option<String>,

    /// Cover photo URL sizes.
    pub cover_photo_url_sizes: Option<serde_json::Value>,

    /// Main video embed HTML.
    pub main_video_embed: Option<String>,

    /// Main video URL.
    pub main_video_url: Option<String>,

    /// Thanks video URL.
    pub thanks_video_url: Option<String>,

    /// Thanks message.
    pub thanks_msg: Option<String>,

    /// Thanks embed HTML.
    pub thanks_embed: Option<String>,

    /// One-liner.
    pub one_liner: Option<String>,

    /// Patron count.
    pub patron_count: Option<i32>,

    /// Paid member count.
    pub paid_member_count: Option<i32>,

    /// Pledge sum in cents.
    pub pledge_sum_cents: Option<i32>,

    /// Currency.
    pub pledge_sum_currency: Option<String>,

    /// Published at.
    pub published_at: Option<DateTime<Utc>>,

    /// Summary.
    pub summary: Option<String>,

    /// Campaign URL.
    pub url: Option<String>,

    /// Vanity.
    pub vanity: Option<String>,

    /// Pay-per name.
    pub pay_per_name: Option<String>,

    /// Whether the campaign is published.
    pub is_published: Option<bool>,

    /// Whether earnings are visible.
    pub show_earnings: Option<bool>,
}

// ============== Member ==============

/// Member attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MemberAttributes {
    /// Patron status.
    pub patron_status: Option<PatronStatus>,

    /// Whether this member is following.
    pub is_follower: Option<bool>,

    /// Full name.
    pub full_name: Option<String>,

    /// Email.
    pub email: Option<String>,

    /// Currently entitled amount (cents).
    pub currently_entitled_amount_cents: Option<i32>,

    /// Lifetime support (cents).
    pub lifetime_support_cents: Option<i32>,

    /// Last charge date.
    pub last_charge_date: Option<DateTime<Utc>>,

    /// Last charge status.
    pub last_charge_status: Option<ChargeStatus>,

    /// Next charge date.
    pub next_charge_date: Option<DateTime<Utc>>,

    /// Pledge relationship start.
    pub pledge_relationship_start: Option<DateTime<Utc>>,

    /// Note.
    pub note: Option<String>,

    /// Will pay amount (cents).
    pub will_pay_amount_cents: Option<i32>,

    /// Campaign currency.
    pub campaign_currency: Option<String>,

    /// Campaign lifetime support (cents).
    pub campaign_lifetime_support_cents: Option<i32>,

    /// Campaign pledge amount (cents).
    pub campaign_pledge_amount_cents: Option<i32>,
}

/// Patron status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PatronStatus {
    /// Active patron.
    ActivePatron,
    /// Declined patron.
    DeclinedPatron,
    /// Former patron.
    FormerPatron,
    /// Other/unknown status.
    #[serde(other)]
    Unknown,
}

impl Default for PatronStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Charge status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChargeStatus {
    /// Paid.
    Paid,
    /// Declined.
    Declined,
    /// Deleted.
    Deleted,
    /// Pending.
    Pending,
    /// Refunded.
    Refunded,
    /// Fraud.
    Fraud,
    /// Other/unknown.
    #[serde(other)]
    Unknown,
}

impl Default for ChargeStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

// ============== Tier ==============

/// Tier attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TierAttributes {
    /// Tier amount (cents).
    pub amount_cents: Option<i32>,

    /// Created at.
    pub created_at: Option<DateTime<Utc>>,

    /// Description.
    pub description: Option<String>,

    /// Discord role IDs.
    pub discord_role_ids: Option<Vec<String>>,

    /// Edited at.
    pub edited_at: Option<DateTime<Utc>>,

    /// Image URL.
    pub image_url: Option<String>,

    /// Patron count.
    pub patron_count: Option<i32>,

    /// Post count.
    pub post_count: Option<i32>,

    /// Whether published.
    pub published: Option<bool>,

    /// Published at.
    pub published_at: Option<DateTime<Utc>>,

    /// Title.
    pub title: Option<String>,

    /// Unpublished at.
    pub unpublished_at: Option<DateTime<Utc>>,

    /// Tier URL.
    pub url: Option<String>,

    /// User limit.
    pub user_limit: Option<i32>,

    /// Remaining capacity.
    pub remaining: Option<i32>,
}

// ============== Post ==============

/// Post attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostAttributes {
    /// Title.
    pub title: Option<String>,

    /// Content (HTML).
    pub content: Option<String>,

    /// Whether public.
    pub is_public: Option<bool>,

    /// Whether paid.
    pub is_paid: Option<bool>,

    /// Published at.
    pub published_at: Option<DateTime<Utc>>,

    /// Edited at.
    pub edited_at: Option<DateTime<Utc>>,

    /// Created at.
    pub created_at: Option<DateTime<Utc>>,

    /// Embed data.
    pub embed: Option<serde_json::Value>,

    /// Embed URL.
    pub embed_url: Option<String>,

    /// App ID.
    pub app_id: Option<i64>,

    /// App status.
    pub app_status: Option<String>,

    /// Image.
    pub image: Option<serde_json::Value>,

    /// Whether this is a teaser.
    pub is_teaser: Option<bool>,

    /// Teaser text.
    pub teaser_text: Option<String>,

    /// Like count.
    pub like_count: Option<i32>,

    /// Comment count.
    pub comment_count: Option<i32>,

    /// Post URL.
    pub url: Option<String>,

    /// Post type.
    pub post_type: Option<String>,

    /// Post file.
    pub post_file: Option<serde_json::Value>,

    /// Post metadata.
    pub post_metadata: Option<serde_json::Value>,

    /// Minimum cents pledged to view.
    pub min_cents_pledged_to_view: Option<i32>,

    /// Thumbnail URL.
    pub thumbnail_url: Option<String>,

    /// Thumbnail.
    pub thumbnail: Option<serde_json::Value>,
}

// ============== Benefit ==============

/// Benefit attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BenefitAttributes {
    /// Title.
    pub title: Option<String>,

    /// Description.
    pub description: Option<String>,

    /// Benefit type.
    pub benefit_type: Option<String>,

    /// Rule type.
    pub rule_type: Option<String>,

    /// Created at.
    pub created_at: Option<DateTime<Utc>>,

    /// Whether published.
    pub is_published: Option<bool>,

    /// Whether deleted.
    pub is_deleted: Option<bool>,

    /// Whether deliverable.
    pub is_deliverable: Option<bool>,

    /// Deliverables due today count.
    pub deliverables_due_today_count: Option<i32>,

    /// Delivered deliverables count.
    pub delivered_deliverables_count: Option<i32>,

    /// Not delivered deliverables count.
    pub not_delivered_deliverables_count: Option<i32>,

    /// Next deliverable due date.
    pub next_deliverable_due_date: Option<DateTime<Utc>>,

    /// Tiers count.
    pub tiers_count: Option<i32>,

    /// App external ID.
    pub app_external_id: Option<String>,

    /// App metadata.
    pub app_meta: Option<serde_json::Value>,
}

// ============== Address ==============

/// Address attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AddressAttributes {
    /// Addressee.
    pub addressee: Option<String>,

    /// City.
    pub city: Option<String>,

    /// Country.
    pub country: Option<String>,

    /// Created at.
    pub created_at: Option<DateTime<Utc>>,

    /// Line 1.
    pub line_1: Option<String>,

    /// Line 2.
    pub line_2: Option<String>,

    /// Phone number.
    pub phone_number: Option<String>,

    /// Postal code.
    pub postal_code: Option<String>,

    /// State/region.
    pub state: Option<String>,

    /// Whether confirmed.
    pub confirmed: Option<bool>,

    /// Confirmed at.
    pub confirmed_at: Option<DateTime<Utc>>,
}

// ============== Goal ==============

/// Goal attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GoalAttributes {
    /// Amount (cents).
    pub amount_cents: Option<i32>,

    /// Completed percentage.
    pub completed_percentage: Option<i32>,

    /// Created at.
    pub created_at: Option<DateTime<Utc>>,

    /// Description.
    pub description: Option<String>,

    /// Reached at.
    pub reached_at: Option<DateTime<Utc>>,

    /// Title.
    pub title: Option<String>,
}

// ============== Media ==============

/// Media attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MediaAttributes {
    /// Created at.
    pub created_at: Option<DateTime<Utc>>,

    /// Download URL.
    pub download_url: Option<String>,

    /// File name.
    pub file_name: Option<String>,

    /// Image URLs.
    pub image_urls: Option<serde_json::Value>,

    /// Metadata.
    pub metadata: Option<serde_json::Value>,

    /// MIME type.
    pub mimetype: Option<String>,

    /// Owner ID.
    pub owner_id: Option<String>,

    /// Owner relationship.
    pub owner_relationship: Option<String>,

    /// Owner type.
    pub owner_type: Option<String>,

    /// Size in bytes.
    pub size_bytes: Option<i64>,

    /// State.
    pub state: Option<String>,

    /// Upload expires at.
    pub upload_expires_at: Option<DateTime<Utc>>,

    /// Upload parameters.
    pub upload_parameters: Option<serde_json::Value>,

    /// Upload URL.
    pub upload_url: Option<String>,
}

// ============== Webhook ==============

/// Webhook attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WebhookAttributes {
    /// Last attempted at.
    pub last_attempted_at: Option<DateTime<Utc>>,

    /// Consecutive failure count.
    pub num_consecutive_times_failed: Option<i32>,

    /// Whether paused.
    pub paused: Option<bool>,

    /// Secret.
    pub secret: Option<String>,

    /// Trigger list.
    pub triggers: Option<Vec<WebhookTrigger>>,

    /// Webhook URL.
    pub uri: Option<String>,
}

/// Webhook trigger type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebhookTrigger {
    /// Member created.
    #[serde(rename = "members:create")]
    MembersCreate,

    /// Member updated.
    #[serde(rename = "members:update")]
    MembersUpdate,

    /// Member deleted.
    #[serde(rename = "members:delete")]
    MembersDelete,

    /// Pledge created.
    #[serde(rename = "members:pledge:create")]
    MembersPledgeCreate,

    /// Pledge updated.
    #[serde(rename = "members:pledge:update")]
    MembersPledgeUpdate,

    /// Pledge deleted.
    #[serde(rename = "members:pledge:delete")]
    MembersPledgeDelete,

    /// Post published.
    #[serde(rename = "posts:publish")]
    PostsPublish,

    /// Post updated.
    #[serde(rename = "posts:update")]
    PostsUpdate,

    /// Post deleted.
    #[serde(rename = "posts:delete")]
    PostsDelete,

    /// Other/unknown trigger.
    #[serde(other)]
    Unknown,
}

impl Default for WebhookTrigger {
    fn default() -> Self {
        Self::Unknown
    }
}
