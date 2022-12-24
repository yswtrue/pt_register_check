use super::Scraper;
pub struct PTTime {}
impl Scraper for PTTime {
    fn registry_url(&self) -> &str {
        return "https://www.pttime.org/signup.php";
    }

    fn dis_match_text(&self) -> &str {
        return "自由注册当前关闭，开启时间不定，可关注各PT社区！";
    }
    fn name(&self) -> &str {
        return "PTTime";
    }
}
