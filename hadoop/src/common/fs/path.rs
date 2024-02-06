use iref::{IriRef, IriRefBuf};

/// The directory separator, a slash.
pub const SEPARATOR: &str = "/";

// TODO: review and refactor `Path`
#[derive(Debug, Clone)]
pub struct Path {
    uri: IriRefBuf,
}

impl Path {
    /// Create a new Path based on the child path resolved against the parent path.
    pub fn from_parent(parent: &Self, child: &Self) -> anyhow::Result<Self> {
        // TODO: review and refactor
        let parent_uri = parent
            .uri
            .as_iri()
            .ok_or(anyhow::Error::msg("Parent path is not absolute"))?;
        let uri = child.uri.resolved(parent_uri).into();
        Ok(Self { uri })
    }

    pub fn to_uri(&self) -> &IriRef {
        self.uri.as_iri_ref()
    }

    /// Returns true if the path component (i.e. directory) of this URI is
    /// absolute.
    pub fn is_uri_path_absolute(&self) -> bool {
        self.uri.path().as_str().starts_with(SEPARATOR)
    }
}

impl From<IriRefBuf> for Path {
    /// Construct a path from a URI
    fn from(uri: IriRefBuf) -> Self {
        // TODO: normalize uri
        Self { uri }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iref::IriBuf;
    use std::str::FromStr;

    #[test]
    fn test_from_parent() {
        let parent = Path::from(IriRefBuf::from_str("hdfs://namenode/user/alex/").unwrap());
        let child = Path::from(IriRefBuf::from_str("database/hive/test.db").unwrap());
        let path = Path::from_parent(&parent, &child).unwrap();
        assert_eq!(
            path.uri.as_str(),
            "hdfs://namenode/user/alex/database/hive/test.db"
        );
    }

    #[test]
    fn test_is_uri_path_absolute() {
        let iri = IriRefBuf::new("/dev/../hello".to_owned()).unwrap();
        if let Some(scheme) = iri.scheme() {
            let i = iri.resolved(IriBuf::from_scheme(scheme.to_owned()).as_iri());
            println!("i: {:#?}", i);
        } else {
            let i = iri.resolved(IriRefBuf::from_str("hdfs://").unwrap().as_iri().unwrap());
            println!("i: {:#?}", i);
        }
        println!("iri: {:#?}", iri);
        // let path = Path {
        //     uri: IriBuf::from_string("dev/hello".to_string()).unwrap(),
        // };
        // let p = path.uri.as_iri();
        // println!("path: {:#?}", path);
    }
}
