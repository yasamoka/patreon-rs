use patreon::{PatreonCreatorClient, PostFields, PostIncludes, PostsQuery};

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn opt_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PatreonCreatorClient::new(env("CREATOR_ACCESS_TOKEN"));

    let campaigns = client.campaigns().call().await?;
    let campaign_id = opt_env("CAMPAIGN_ID")
        .or_else(|| campaigns.data.first().map(|c| c.id.clone()))
        .expect("CAMPAIGN_ID is required (or ensure the token has at least one campaign)");

    let posts = client.campaign_posts(&campaign_id).call().await?;
    println!("campaign_posts.count: {}", posts.data.len());

    let posts_with_query = client
        .campaign_posts(&campaign_id)
        .query(PostsQuery::builder().page_size(10).build())
        .call()
        .await?;
    println!(
        "campaign_posts_with_query.count: {}",
        posts_with_query.data.len()
    );

    let posts_with_details = client
        .campaign_posts(&campaign_id)
        .fields(PostFields::all())
        .includes(PostIncludes::all())
        .call()
        .await?;
    println!(
        "campaign_posts_with_details.count: {}",
        posts_with_details.data.len()
    );

    let post_id = opt_env("POST_ID")
        .or_else(|| posts.data.first().map(|p| p.id.clone()))
        .expect("POST_ID is required (or ensure the campaign has at least one post)");

    let post = client.post(&post_id).call().await?;
    println!("post.id: {}", post.data.id);

    let post_with_details = client
        .post(&post_id)
        .fields(PostFields::all())
        .includes(PostIncludes::all())
        .call()
        .await?;
    println!("post_with_details.id: {}", post_with_details.data.id);

    Ok(())
}
