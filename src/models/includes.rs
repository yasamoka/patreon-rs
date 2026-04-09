use crate::{AddressFields, CampaignFields};

use super::fields::{BenefitFields, Fields, GoalFields, TierFields, UserFields};

pub(crate) trait Includes {
    fn include(&self) -> String;

    fn fields(&self) -> String;
}

impl<T: IncludesImpl> Includes for T {
    fn include(&self) -> String {
        format!(
            "include={}",
            self.includes()
                .filter_map(|(fields, include)| if fields { Some(include) } else { None })
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    fn fields(&self) -> String {
        <Self as IncludesImpl>::fields(self)
            .filter_map(|s| s)
            .collect::<Vec<_>>()
            .join("&")
    }
}

trait IncludesImpl {
    fn includes(&self) -> impl Iterator<Item = (bool, &'static str)>;

    fn fields(&self) -> impl Iterator<Item = Option<String>>;
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct CampaignIncludes {
    pub tiers: Option<TierFields>,
    pub creator: Option<UserFields>,
    pub benefits: Option<BenefitFields>,
    pub goals: Option<GoalFields>,
}

impl CampaignIncludes {
    pub fn all() -> Self {
        Self {
            tiers: Some(TierFields::all()),
            creator: Some(UserFields::all()),
            benefits: Some(BenefitFields::all()),
            goals: Some(GoalFields::all()),
        }
    }
}

impl IncludesImpl for CampaignIncludes {
    fn includes(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self {
            tiers,
            creator,
            benefits,
            goals,
        } = *self;
        [
            (tiers.is_some(), "tiers"),
            (creator.is_some(), "creator"),
            (benefits.is_some(), "tiers.benefits"),
            (goals.is_some(), "goals"),
        ]
        .into_iter()
    }

    fn fields(&self) -> impl Iterator<Item = Option<String>> {
        let Self {
            tiers,
            creator,
            benefits,
            goals,
        } = *self;
        [
            tiers.map(|f| f.fields()),
            creator.map(|f| f.fields()),
            benefits.map(|f| f.fields()),
            goals.map(|f| f.fields()),
        ]
        .into_iter()
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct MemberIncludes {
    pub address: Option<AddressFields>,
    pub campaign: Option<CampaignFields>,
    pub currently_entitled_tiers: Option<TierFields>,
    pub user: Option<UserFields>,
}

impl MemberIncludes {
    pub fn all() -> Self {
        Self {
            address: Some(AddressFields::all()),
            campaign: Some(CampaignFields::all()),
            currently_entitled_tiers: Some(TierFields::all()),
            user: Some(UserFields::all()),
        }
    }
}

impl IncludesImpl for MemberIncludes {
    fn includes(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self {
            address,
            campaign,
            currently_entitled_tiers,
            user,
        } = *self;
        [
            (address.is_some(), "address"),
            (campaign.is_some(), "campaign"),
            (
                currently_entitled_tiers.is_some(),
                "currently_entitled_tiers",
            ),
            (user.is_some(), "user"),
        ]
        .into_iter()
    }

    fn fields(&self) -> impl Iterator<Item = Option<String>> {
        let Self {
            address,
            campaign,
            currently_entitled_tiers,
            user,
        } = *self;
        [
            address.map(|f| f.fields()),
            campaign.map(|f| f.fields()),
            currently_entitled_tiers.map(|f| f.fields()),
            user.map(|f| f.fields()),
        ]
        .into_iter()
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct PostIncludes {
    pub user: Option<UserFields>,
    pub campaign: Option<CampaignFields>,
}

impl PostIncludes {
    pub fn all() -> Self {
        Self {
            user: Some(UserFields::all()),
            campaign: Some(CampaignFields::all()),
        }
    }
}

impl IncludesImpl for PostIncludes {
    fn includes(&self) -> impl Iterator<Item = (bool, &'static str)> {
        let Self { user, campaign } = *self;
        [(user.is_some(), "user"), (campaign.is_some(), "campaign")].into_iter()
    }

    fn fields(&self) -> impl Iterator<Item = Option<String>> {
        let Self { user, campaign } = *self;
        [user.map(|f| f.fields()), campaign.map(|f| f.fields())].into_iter()
    }
}
