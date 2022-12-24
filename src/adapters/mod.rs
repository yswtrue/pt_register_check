use anyhow::{Ok, Result};
pub mod hdsky;
pub mod pttime;
use crate::utils;
use async_trait::async_trait;
#[async_trait]
pub trait Scraper {
    fn name(&self) -> &str;
    fn registry_url(&self) -> &str;
    fn dis_match_text(&self) -> &str;
    async fn check(&self) -> Result<bool> {
        let resp = reqwest::get(self.registry_url()).await?.text().await?;
        if !resp.contains(self.dis_match_text()) {
            utils::send_email(format!("{}已开放注册", self.name()), resp).await?;
            return Ok(true);
        }
        Ok(false)
    }
}
