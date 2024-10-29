use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default: String,
    pub profile: Vec<ProfileConfig>,
}

impl Config {
    pub fn get_default_profile(&self) -> ProfileConfig {
        let blabla: Vec<ProfileConfig> = self.profile.clone().into_iter().filter(|profilez| profilez.name == self.default).collect();
        blabla.first().unwrap().clone()
    }

    pub fn get_profile_by_domain(&self, url: Url) -> ProfileConfig {
        if url.domain().is_none() {
            return self.get_default_profile();
        }

        let mut search_domain = url.domain().unwrap().to_string();
        if search_domain == "localhost" && url.port().is_some() {
            let port = url.port().unwrap().to_string();
            search_domain = format!("localhost:{port}")
        }
        let mut ret: Option<ProfileConfig> = None;
        for item in self.profile.clone().into_iter() {
            if item.domains.is_none() {
                continue;
            }

            for domain_item in item.clone().domains.unwrap().into_iter() {
                let mut check_domain = domain_item.to_string();
                if domain_item.starts_with("*.") {
                    check_domain = domain_item[2..].to_string()
                }

                if check_domain == search_domain || search_domain.ends_with(check_domain.as_str()) {
                    ret = Some(item);
                    break;
                }
            }

            if ret.is_some() {
                break;
            }
        }

        ret.unwrap_or_else(|| self.get_default_profile())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProfileConfig {
    pub name: String,
    pub exec: String,
    pub domains: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
}