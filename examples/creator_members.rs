use patreon::{
    AddressFields, MemberFields, MemberIncludes, MembersQuery, PatreonCreatorClient, TierFields,
    UserFields,
};

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn opt_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PatreonCreatorClient::new(env("CREATOR_ACCESS_TOKEN"));

    let campaigns = client.campaigns(None, None).await?;
    let campaign_id = opt_env("CAMPAIGN_ID")
        .or_else(|| campaigns.data.first().map(|c| c.id.clone()))
        .expect("CAMPAIGN_ID is required (or ensure the token has at least one campaign)");

    let members = client
        .campaign_members(&campaign_id, None, None, None)
        .await?;
    println!("campaign_members.count: {}", members.data.len());

    let members_with_query = client
        .campaign_members(
            &campaign_id,
            None,
            None,
            Some(MembersQuery {
                cursor: None,
                page_size: Some(10),
            }),
        )
        .await?;
    println!(
        "campaign_members_with_query.count: {}",
        members_with_query.data.len()
    );

    let members_with_details = client
        .campaign_members(
            &campaign_id,
            Some(MemberFields::all()),
            Some(MemberIncludes {
                address: Some(AddressFields::all()),
                currently_entitled_tiers: Some(TierFields::all()),
                user: Some(UserFields::all()),
                ..Default::default()
            }),
            None,
        )
        .await?;
    println!(
        "campaign_members_with_details.count: {}",
        members_with_details.data.len()
    );

    let members_with_details_and_query = client
        .campaign_members(
            &campaign_id,
            Some(MemberFields::all()),
            Some(MemberIncludes {
                address: Some(AddressFields::all()),
                currently_entitled_tiers: Some(TierFields::all()),
                user: Some(UserFields::all()),
                ..Default::default()
            }),
            Some(MembersQuery {
                cursor: None,
                page_size: Some(10),
            }),
        )
        .await?;
    println!(
        "campaign_members_with_details_and_query.count: {}",
        members_with_details_and_query.data.len()
    );

    let member_id = opt_env("MEMBER_ID")
        .or_else(|| members.data.first().map(|m| m.id.clone()))
        .expect("MEMBER_ID is required (or ensure the campaign has at least one member)");

    let member = client.member(&member_id, None, None).await?;
    println!("member.id: {}", member.data.id);

    let member_with_details = client
        .member(
            &member_id,
            Some(MemberFields::all()),
            Some(MemberIncludes::all()),
        )
        .await?;
    println!("member_with_details.id: {}", member_with_details.data.id);

    Ok(())
}
