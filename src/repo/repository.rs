use async_stream::stream;
use async_walkdir::WalkDir;
use futures::{Stream, StreamExt};
use tracing::{debug, error};

use crate::{Package, Repo};

#[derive(Clone, Debug, PartialEq)]
pub struct Repository {
    pub tag: String,
    pub name: Option<String>,
    pub enabled: Option<u64>,
    pub mirror_list: Option<String>,
    pub base_url: Option<String>,
    pub ssl_ca_cert: Option<String>,
    pub ssl_client_cert: Option<String>,
    pub ssl_verify: Option<u64>,
    pub metadata_expire: Option<u64>,
    pub enable_metadata: Option<u64>,
    pub gpg_check: Option<u64>,
    pub gpg_key: Option<String>,
}

impl Repository {
    pub fn is_enabled(&self) -> bool {
        self.enabled.is_none() || (self.enabled.is_some() && self.enabled.unwrap() == 1)
    }

    pub fn packages(self) -> impl Stream<Item = Package> {
        let path = "/var/cache/yum";
        let mut dir_entries = WalkDir::new(path);
        stream! {
            'repos: loop {
                debug!("repos loop");
                match dir_entries.next().await {
                    Some(Ok(entry)) => {
                        debug!("next entry {:?}", entry.path());
                        if entry.path().is_dir() && entry.path().ends_with(self.tag.clone()) {
                            debug!("Found matching tag!");
                            let path: String = entry.path().to_string_lossy().to_string();
                            let mut dir_walker = WalkDir::new(path);

                            // NOTE: Now iterating through THIS repos packages, must use " break 'repo; "
                            loop {
                                match dir_walker.next().await {
                                    Some(Ok(entry)) => {
                                        if entry.path().ends_with("primary.xml") {
                                            match quick_xml::Reader::from_file(entry.path()) {
                                                Ok(reader) => {
                                                    match quick_xml::de::from_reader(reader.into_underlying_reader()) {
                                                        Ok(repo) => {
                                                            let repo: Repo = repo;

                                                            for package in repo.packages {
                                                                yield package;
                                                            }
                                                            break 'repos;
                                                        }
                                                        Err(e) => {
                                                            error!("{:?}", e);
                                                            break 'repos;
                                                        }
                                                    }
                                                },
                                                Err(e) => {
                                                    error!("{:#}", e);
                                                    break 'repos;
                                                }
                                            }
                                        }
                                    },
                                    Some(Err(e)) => {
                                        error!("{:#}", e);
                                        break 'repos;
                                    }
                                    None => break 'repos,
                                };
                            }
                        }
                    },
                    Some(Err(e)) => {
                        error!("{:#}", e);
                        break;
                    }
                    None => break,
                };
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RepositoryList;

impl RepositoryList {
    pub fn default() -> impl Stream<Item = Repository> {
        Self::from_path("/etc/yum.repos.d")
    }

    pub fn from_path(path: &str) -> impl Stream<Item = Repository> {
        let mut dir_walker = WalkDir::new(path);

        // ? QUESTION: Investigate stream combinator pattern and use AndThen/and_then ?
        stream! {
            loop {
                match dir_walker.next().await {
                    Some(Ok(entry)) => {
                        let absolute_path = entry.path().to_string_lossy().to_string();
                        if entry.path().is_file() && absolute_path.contains(".repo") {
                            let mut config = configparser::ini::Ini::new_cs();
                            if let Some(path) = entry.path().to_str() {
                                let config_map = config.load(&path).unwrap();

                                for section in config_map.keys() {
                                    yield Repository {
                                        tag: section.clone(),
                                        name: config.get(&section, "name").take(),
                                        enabled: config.getuint(&section, "enabled").unwrap_or(None),
                                        mirror_list: config.get(&section, "mirrorlist").take(),
                                        base_url: config.get(&section, "baseurl").take(),
                                        ssl_ca_cert: config.get(&section, "sslcacert").take(),
                                        ssl_client_cert: config.get(&section, "sslclientcert").take(),
                                        ssl_verify: config.getuint(&section, "sslverify").unwrap_or(None),
                                        metadata_expire: config.getuint(&section, "metadataexpire").unwrap_or(None),
                                        enable_metadata: config.getuint(&section, "enablemetadata").unwrap_or(None),
                                        gpg_check: config.getuint(&section, "gpgcheck").unwrap_or(None),
                                        gpg_key: config.get(&section, "gpgkey").take(),
                                    };
                                }
                            }
                        }
                    },
                    Some(Err(e)) => {
                        error!("{:#}", e);
                        break;
                    }
                    None => break,
                }
            }
        }
    }
}
