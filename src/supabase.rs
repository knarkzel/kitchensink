use crate::*;

pub fn new() -> postgrest::Postgrest {
    postgrest::Postgrest::new(format!("{SUPABASE_ENDPOINT}/rest/v1"))
        .insert_header("apikey", SUPABASE_ANON_KEY)
}

pub async fn save(query: String) -> Result<()> {
    new()
        .from("settings")
        .upsert(query)
        .on_conflict("user_id")
        .execute()
        .await?;
    Ok(())
}

pub async fn settings(user_id: &str) -> Result<Settings> {
    let response = new()
        .from("settings")
        .select("feeds")
        .eq("user_id", user_id)
        .single()
        .execute()
        .await?;
    Ok(response.json::<Settings>().await?)
}
