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

    pub fn get_profile_by_domain(&self, url: &Url) -> ProfileConfig {
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
            if item.domains.is_some() && item.do_i_like_this_domain(&search_domain) {
                ret = Some(item.clone());
                break;
            }

            if item.prefixed_uris.is_some() && item.do_i_like_this_url(&url.to_string()) {
                ret = Some(item.clone());
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
    pub prefixed_uris: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
}

impl ProfileConfig {
    pub(crate) fn do_i_like_this_domain(&self, domain: &str) -> bool {
        for domain_item in self.clone().domains.unwrap().into_iter() {
            let mut check_domain = domain_item.to_string();
            if domain_item.starts_with("*.") {
                check_domain = domain_item[2..].to_string()
            }

            if check_domain == domain || domain.ends_with(check_domain.as_str()) {
                return true;
            }
        }

        false
    }

    pub(crate) fn do_i_like_this_url(&self, url: &str) -> bool {
        for uri_prefix in self.clone().prefixed_uris.unwrap().into_iter() {
            let check_uri = uri_prefix.to_string();

            if url.starts_with(&check_uri) {
                return true;
            }
        }

        false
    }
}