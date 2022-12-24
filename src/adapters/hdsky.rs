use super::Scraper;
pub struct HDSky {}
impl Scraper for HDSky {
    fn registry_url(&self) -> &str {
        return "https://hdsky.me/signup.php";
    }

    fn dis_match_text(&self) -> &str {
        return "自由注册当前关闭，只允许邀请注册。";
    }

    fn name(&self) -> &str {
        return "HDSky";
    }
}
