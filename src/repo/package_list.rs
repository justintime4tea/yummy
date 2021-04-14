use async_stream::stream;
use futures::Stream;
use tracing::error;

use crate::Package;

#[derive(Clone, Debug, PartialEq)]
pub struct PackageList {
    path: String,
}

impl From<&str> for PackageList {
    fn from(tag: &str) -> Self {
        Self::from_tag(tag.into())
    }
}

impl PackageList {
    pub fn from_tag(path: String) -> Self {
        PackageList { path }
    }

    pub fn into_stream(self) -> impl Stream<Item = Package> {
        let tag = self.path;
        stream! {
          let packages = match crate::get_package_list_from_tag(&tag, "x86_64").await {
            Ok(packages) => packages,
            Err(e) => {
              error!("{:#}", e);
              vec![]
            }
          };

          for package in packages {
            yield package;
          }
        }
    }
}
