use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProviderOutput {
    pub content: String,
}