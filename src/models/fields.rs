pub(crate) trait Fields {
    fn fields(&self) -> String;
}

impl<T: FieldsImpl> Fields for T {
    fn fields(&self) -> String {
        format!(
            "fields[{}]={}",
            Self::name(),
            <T as FieldsImpl>::fields(&self)
                .filter_map(|(flag, field)| if flag { Some(field) } else { None })
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

trait FieldsImpl {
    fn name() -> &'static str;

    fn fields(&self) -> impl Iterator<Item = (bool, &'static str)>;
}

/// Field names for campaign resources.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct CampaignFields {
    pub created_at: bool,
    pub creation_name: bool,
    pub discord_server_id: bool,
    pub google_analytics_id: bool,
    pub image_url: bool,
    pub image_small_url: bool,
    pub is_charged_immediately: bool,
    pub is_monthly: bool,
    pub is_nsfw: bool,
    pub main_video_embed: bool,
    pub main_video_url: bool,
    pub one_liner: bool,
    pub patron_count: bool,
    pub pay_per_name: bool,
    pub published_at: bool,
    pub summary: bool,
    pub thanks_embed: bool,
    pub thanks_msg: bool,
    pub thanks_video_url: bool,
    pub url: bool,
    pub vanity: bool,
}

impl CampaignFields {
    pub fn all() -> Self {
        Self {
            created_at: true,
            creation_name: true,
            discord_server_id: true,
            google_analytics_id: true,
            image_url: true,
            image_small_url: true,
            is_charged_immediately: true,
            is_monthly: true,
            is_nsfw: true,
            main_video_embed: true,
            main_video_url: true,
            one_liner: true,
            patron_count: true,
            pay_per_name: true,
            published_at: true,
            summary: true,
            thanks_embed: true,
            thanks_msg: true,
            thanks_video_url: true,
            url: true,
            vanity: true,
        }
    }
}

impl FieldsImpl for CampaignFields {
    fn name() -> &'static str {
        "campaign"
    }

    fn fields(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self {
            created_at,
            creation_name,
            discord_server_id,
            google_analytics_id,
            image_url,
            image_small_url,
            is_charged_immediately,
            is_monthly,
            is_nsfw,
            main_video_embed,
            main_video_url,
            one_liner,
            patron_count,
            pay_per_name,
            published_at,
            summary,
            thanks_embed,
            thanks_msg,
            thanks_video_url,
            url,
            vanity,
        } = *self;
        [
            (created_at, "created_at"),
            (creation_name, "creation_name"),
            (discord_server_id, "discord_server_id"),
            (google_analytics_id, "google_analytics_id"),
            (image_url, "image_url"),
            (image_small_url, "image_small_url"),
            (is_charged_immediately, "is_charged_immediately"),
            (is_monthly, "is_monthly"),
            (is_nsfw, "is_nsfw"),
            (main_video_embed, "main_video_embed"),
            (main_video_url, "main_video_url"),
            (one_liner, "one_liner"),
            (patron_count, "patron_count"),
            (pay_per_name, "pay_per_name"),
            (published_at, "published_at"),
            (summary, "summary"),
            (thanks_embed, "thanks_embed"),
            (thanks_msg, "thanks_msg"),
            (thanks_video_url, "thanks_video_url"),
            (url, "url"),
            (vanity, "vanity"),
        ]
        .into_iter()
    }
}

/// Field names for member resources.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct MemberFields {
    pub campaign_lifetime_support_cents: bool,
    pub currently_entitled_amount_cents: bool,
    pub email: bool,
    pub full_name: bool,
    pub is_follower: bool,
    pub last_charge_date: bool,
    pub last_charge_status: bool,
    pub lifetime_support_cents: bool,
    pub next_charge_date: bool,
    pub note: bool,
    pub patron_status: bool,
    pub pledge_relationship_start: bool,
    pub will_pay_amount_cents: bool,
}

impl MemberFields {
    pub fn all() -> Self {
        Self {
            campaign_lifetime_support_cents: true,
            currently_entitled_amount_cents: true,
            email: true,
            full_name: true,
            is_follower: true,
            last_charge_date: true,
            last_charge_status: true,
            lifetime_support_cents: true,
            next_charge_date: true,
            note: true,
            patron_status: true,
            pledge_relationship_start: true,
            will_pay_amount_cents: true,
        }
    }
}

impl FieldsImpl for MemberFields {
    fn name() -> &'static str {
        "member"
    }

    fn fields(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self {
            campaign_lifetime_support_cents,
            currently_entitled_amount_cents,
            email,
            full_name,
            is_follower,
            last_charge_date,
            last_charge_status,
            lifetime_support_cents,
            next_charge_date,
            note,
            patron_status,
            pledge_relationship_start,
            will_pay_amount_cents,
        } = *self;
        [
            (
                campaign_lifetime_support_cents,
                "campaign_lifetime_support_cents",
            ),
            (
                currently_entitled_amount_cents,
                "currently_entitled_amount_cents",
            ),
            (email, "email"),
            (full_name, "full_name"),
            (is_follower, "is_follower"),
            (last_charge_date, "last_charge_date"),
            (last_charge_status, "last_charge_status"),
            (lifetime_support_cents, "lifetime_support_cents"),
            (next_charge_date, "next_charge_date"),
            (note, "note"),
            (patron_status, "patron_status"),
            (pledge_relationship_start, "pledge_relationship_start"),
            (will_pay_amount_cents, "will_pay_amount_cents"),
        ]
        .into_iter()
    }
}

/// Field names for user resources.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct UserFields {
    email: bool,
    full_name: bool,
    image_url: bool,
    url: bool,
    vanity: bool,
}

impl UserFields {
    pub fn all() -> Self {
        Self {
            email: true,
            full_name: true,
            image_url: true,
            url: true,
            vanity: true,
        }
    }
}

impl FieldsImpl for UserFields {
    fn name() -> &'static str {
        "user"
    }

    fn fields(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self {
            email,
            full_name,
            image_url,
            url,
            vanity,
        } = *self;
        [
            (email, "email"),
            (full_name, "full_name"),
            (image_url, "image_url"),
            (url, "url"),
            (vanity, "vanity"),
        ]
        .into_iter()
    }
}

/// Field names for tier resources.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct TierFields {
    amount_cents: bool,
    title: bool,
    url: bool,
}

impl TierFields {
    pub fn all() -> Self {
        Self {
            amount_cents: true,
            title: true,
            url: true,
        }
    }
}

impl FieldsImpl for TierFields {
    fn name() -> &'static str {
        "tier"
    }

    fn fields(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self {
            amount_cents,
            title,
            url,
        } = *self;
        [
            (amount_cents, "amount_cents"),
            (title, "title"),
            (url, "url"),
        ]
        .into_iter()
    }
}

/// Field names for address resources.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct AddressFields {
    addressee: bool,
    city: bool,
    country: bool,
    line_1: bool,
    line_2: bool,
    phone_number: bool,
    postal_code: bool,
    state: bool,
}

impl AddressFields {
    pub fn all() -> Self {
        Self {
            addressee: true,
            city: true,
            country: true,
            line_1: true,
            line_2: true,
            phone_number: true,
            postal_code: true,
            state: true,
        }
    }
}

impl FieldsImpl for AddressFields {
    fn name() -> &'static str {
        "address"
    }

    fn fields(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self {
            addressee,
            city,
            country,
            line_1,
            line_2,
            phone_number,
            postal_code,
            state,
        } = *self;
        [
            (addressee, "addressee"),
            (city, "city"),
            (country, "country"),
            (line_1, "line_1"),
            (line_2, "line_2"),
            (phone_number, "phone_number"),
            (postal_code, "postal_code"),
            (state, "state"),
        ]
        .into_iter()
    }
}

/// Field names for post resources.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct PostFields {
    app_id: bool,
    app_status: bool,
    content: bool,
    embed_data: bool,
    embed_url: bool,
    is_paid: bool,
    is_public: bool,
    published_at: bool,
    title: bool,
    url: bool,
}

impl PostFields {
    pub fn all() -> Self {
        Self {
            app_id: true,
            app_status: true,
            content: true,
            embed_data: true,
            embed_url: true,
            is_paid: true,
            is_public: true,
            published_at: true,
            title: true,
            url: true,
        }
    }
}

impl FieldsImpl for PostFields {
    fn name() -> &'static str {
        "post"
    }

    fn fields(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self {
            app_id,
            app_status,
            content,
            embed_data,
            embed_url,
            is_paid,
            is_public,
            published_at,
            title,
            url,
        } = *self;
        [
            (app_id, "app_id"),
            (app_status, "app_status"),
            (content, "content"),
            (embed_data, "embed_data"),
            (embed_url, "embed_url"),
            (is_paid, "is_paid"),
            (is_public, "is_public"),
            (published_at, "published_at"),
            (title, "title"),
            (url, "url"),
        ]
        .into_iter()
    }
}

/// Field names for benefit resources.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct BenefitFields {
    benefit_type: bool,
    created_at: bool,
    deliverables_due_today_count: bool,
    delivered_deliverables_count: bool,
    description: bool,
    is_deleted: bool,
    is_published: bool,
    next_deliverable_due_date: bool,
    not_delivered_deliverables_count: bool,
    rule_type: bool,
    tiers_count: bool,
    title: bool,
}

impl BenefitFields {
    pub fn all() -> Self {
        Self {
            benefit_type: true,
            created_at: true,
            deliverables_due_today_count: true,
            delivered_deliverables_count: true,
            description: true,
            is_deleted: true,
            is_published: true,
            next_deliverable_due_date: true,
            not_delivered_deliverables_count: true,
            rule_type: true,
            tiers_count: true,
            title: true,
        }
    }
}

impl FieldsImpl for BenefitFields {
    fn name() -> &'static str {
        "benefit"
    }

    fn fields(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self {
            benefit_type,
            created_at,
            deliverables_due_today_count,
            delivered_deliverables_count,
            description,
            is_deleted,
            is_published,
            next_deliverable_due_date,
            not_delivered_deliverables_count,
            rule_type,
            tiers_count,
            title,
        } = *self;
        [
            (benefit_type, "benefit_type"),
            (created_at, "created_at"),
            (deliverables_due_today_count, "deliverables_due_today_count"),
            (delivered_deliverables_count, "delivered_deliverables_count"),
            (description, "description"),
            (is_deleted, "is_deleted"),
            (is_published, "is_published"),
            (next_deliverable_due_date, "next_deliverable_due_date"),
            (
                not_delivered_deliverables_count,
                "not_delivered_deliverables_count",
            ),
            (rule_type, "rule_type"),
            (tiers_count, "tiers_count"),
            (title, "title"),
        ]
        .into_iter()
    }
}

/// Field names for goal resources.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct GoalFields {
    amount_cents: bool,
    completed_percentage: bool,
    created_at: bool,
    description: bool,
    reached_at: bool,
    title: bool,
}

impl GoalFields {
    pub fn all() -> Self {
        Self {
            amount_cents: true,
            completed_percentage: true,
            created_at: true,
            description: true,
            reached_at: true,
            title: true,
        }
    }
}

impl FieldsImpl for GoalFields {
    fn name() -> &'static str {
        "goal"
    }

    fn fields(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self {
            amount_cents,
            completed_percentage,
            created_at,
            description,
            reached_at,
            title,
        } = *self;
        [
            (amount_cents, "amount_cents"),
            (completed_percentage, "completed_percentage"),
            (created_at, "created_at"),
            (description, "description"),
            (reached_at, "reached_at"),
            (title, "title"),
        ]
        .into_iter()
    }
}
