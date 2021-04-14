use anyhow::Result;
use futures::{future::BoxFuture, FutureExt};
use tokio::fs::DirEntry;
use tracing::error;

mod dto;
mod repo;

pub use dto::*;
pub use repo::*;

pub async fn get_package_list_from_tag(tag: &str, arch: &str) -> Result<Vec<Package>> {
    let mut packages: Vec<Package> = vec![];
    let path = format!("{}/{}", "/var/cache/yum", arch);
    let mut dirs = tokio::fs::read_dir(path).await?;
    while let Some(entry) = dirs.next_entry().await? {
        if entry.path().is_dir() {
            let mut dirs = tokio::fs::read_dir(entry.path()).await?;
            while let Some(entry) = dirs.next_entry().await? {
                if entry.path().is_dir() && entry.path().ends_with(&tag) {
                    match get_package_metadata_from_dir(entry).await {
                        Ok(package_entry) => {
                            for package in package_entry {
                                packages.push(package);
                            }
                            break;
                        }
                        Err(e) => {
                            error!("{:?}", e);
                            break;
                        }
                    };
                }
            }
        }
    }

    Ok(packages)
}

pub async fn get_package_list_from_cache() -> Result<Vec<Package>> {
    let mut packages: Vec<Package> = vec![];
    let mut dirs = tokio::fs::read_dir("/var/cache/yum").await?;

    while let Some(entry) = dirs.next_entry().await? {
        if entry.path().is_dir() {
            match get_package_metadata_from_dir(entry).await {
                Ok(package_entry) => {
                    for package in package_entry {
                        packages.push(package);
                    }
                }
                Err(e) => error!("{:?}", e),
            };
        }
    }

    Ok(packages)
}

pub fn get_package_metadata_from_dir(dir_entry: DirEntry) -> BoxFuture<'static, Result<Vec<Package>>> {
    async move {
        let mut packages: Vec<Package> = vec![];
        if dir_entry.path().is_file() && dir_entry.path().ends_with("primary.xml") {
            let path = dir_entry.path().canonicalize()?;
            let path = path.as_os_str();

            let reader = quick_xml::Reader::from_file(path)?;
            let packages_from_file: Vec<Package> = match quick_xml::de::from_reader(reader.into_underlying_reader()) {
                Ok(repo) => {
                    let repo: Repo = repo;
                    repo.packages
                }
                Err(e) => {
                    error!("{:?}", e);
                    vec![]
                }
            };

            for package in packages_from_file {
                packages.push(package);
            }
        } else if dir_entry.path().is_dir() {
            let mut dirs = tokio::fs::read_dir(dir_entry.path().canonicalize()?.as_os_str()).await?;
            while let Some(dir_entry) = dirs.next_entry().await? {
                match get_package_metadata_from_dir(dir_entry).await {
                    Ok(package_entries) => {
                        for package in package_entries {
                            packages.push(package);
                        }
                    }
                    Err(e) => error!("{:?}", e),
                }
            }
        }

        Ok(packages)
    }
    .boxed()
}
