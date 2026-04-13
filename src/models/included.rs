use serde::{Deserialize, Serialize};

use crate::{
    AddressAttributes, BenefitAttributes, CampaignAttributes, GoalAttributes, IncludedResource,
    TierAttributes, UserAttributes,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum CampaignIncludedResource {
    Tier(IncludedResource<TierAttributes>),
    #[serde(rename = "user")]
    Creator(IncludedResource<UserAttributes>),
    Benefit(IncludedResource<BenefitAttributes>),
    Goal(IncludedResource<GoalAttributes>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum MemberIncludedResource {
    Address(IncludedResource<AddressAttributes>),
    Campaign(IncludedResource<CampaignAttributes>),
    #[serde(rename = "tier")]
    CurrentlyEntitledTier(IncludedResource<TierAttributes>),
    User(IncludedResource<UserAttributes>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PostIncludedResource {
    User(IncludedResource<UserAttributes>),
    Campaign(IncludedResource<CampaignAttributes>),
}
