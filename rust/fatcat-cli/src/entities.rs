
use std::str::FromStr;
use anyhow::{Result, anyhow};
use lazy_static::lazy_static;
use regex::Regex;
use fatcat_openapi::models;
use crate::Specifier;


#[derive(Debug, PartialEq, Clone)]
pub struct Mutation {
    field: String,
    value: Option<String>,
}

impl FromStr for Mutation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // first try simple entity prefixes
        lazy_static! {
            static ref MUTATE_ENTITY_RE: Regex = Regex::new(r"^([a-z_]+)=(.*)$").unwrap();
        }
        if let Some(captures) = MUTATE_ENTITY_RE.captures(s) {
            // XXX: Some() vs None for value
            return Ok(Mutation {
                field: captures[1].to_string(),
                value: match &captures[2] {
                    "" => None,
                    val => Some(val.to_string()),
                },
            });
        }
        Err(anyhow!("not a field mutation: {}", s))
    }
}

/*
 * Goal is to have traits around API entities. Things we'll want to do on concrete entities:
 *
 * - print, or pretty-print, as JSON or TOML
 * - get fcid (or, self-specifier)
 * - update (mutate or return copy) fields based on parameters
 * - update self to remote API
 *
 * Methods that might return trait objects:
 *
 * - get by specifier
 */

pub trait ApiEntityModel: ApiModelSer+ApiModelIdent+ApiModelMutate {
}

impl ApiEntityModel for models::ReleaseEntity {}
impl ApiEntityModel for models::ContainerEntity {}
impl ApiEntityModel for models::CreatorEntity {}
impl ApiEntityModel for models::WorkEntity {}
impl ApiEntityModel for models::FileEntity {}
impl ApiEntityModel for models::FilesetEntity {}
impl ApiEntityModel for models::WebcaptureEntity {}
impl ApiEntityModel for models::Editor{}
impl ApiEntityModel for models::Editgroup{}
impl ApiEntityModel for models::ChangelogEntry{}

pub trait ApiModelSer {
    fn to_json_string(&self) -> Result<String>;
    fn to_toml_string(&self) -> Result<String>;
}

impl<T: serde::Serialize> ApiModelSer for T {

    fn to_json_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    fn to_toml_string(&self) -> Result<String> {
        Ok(toml::Value::try_from(self)?.to_string())
    }
}

pub trait ApiModelIdent {
    fn specifier(&self) -> Specifier;
}

macro_rules! generic_entity_specifier {
    ($specifier_type:ident) => {
        fn specifier(&self) -> Specifier {
            if let Some(fcid) = &self.ident { Specifier::$specifier_type(fcid.to_string()) } else { panic!("expected full entity") }
        }
    }
}

impl ApiModelIdent for models::ReleaseEntity { generic_entity_specifier!(Release); }
impl ApiModelIdent for models::ContainerEntity { generic_entity_specifier!(Container); }
impl ApiModelIdent for models::CreatorEntity { generic_entity_specifier!(Creator); }
impl ApiModelIdent for models::WorkEntity { generic_entity_specifier!(Work); }
impl ApiModelIdent for models::FileEntity { generic_entity_specifier!(File); }
impl ApiModelIdent for models::FilesetEntity { generic_entity_specifier!(FileSet); }
impl ApiModelIdent for models::WebcaptureEntity { generic_entity_specifier!(WebCapture); }

impl ApiModelIdent for models::ChangelogEntry{
    fn specifier(&self) -> Specifier {
        Specifier::Changelog(self.index)
    }
}

impl ApiModelIdent for models::Editgroup {
    fn specifier(&self) -> Specifier {
        if let Some(fcid) = &self.editgroup_id { Specifier::Editgroup(fcid.to_string()) } else { panic!("expected full entity") }
    }
}

impl ApiModelIdent for models::Editor {
    fn specifier(&self) -> Specifier {
        if let Some(fcid) = &self.editor_id { Specifier::Editor(fcid.to_string()) } else { panic!("expected full entity") }
    }
}

pub trait ApiModelMutate {
    fn mutate(&mut self, mutations: Vec<Mutation>) -> Result<()>;
}

impl ApiModelMutate for models::ReleaseEntity {
    fn mutate(&mut self, mutations: Vec<Mutation>) -> Result<()> {
        for m in mutations {
            match (m.field.as_str(), m.value) {
                ("title", val) => { self.title = val; },
                ("subtitle", val) => { self.subtitle = val; },
                ("container_id", val) => { self.container_id = val; },
                ("work_id", val) => { self.work_id = val; },
                ("release_type", val) => { self.release_type = val; },
                ("release_stage", val) => { self.release_stage = val; },
                ("withdrawn_status", val) => { self.withdrawn_status = val; },
                ("license_slug", val) => { self.license_slug= val; },
                ("volume", val) => { self.volume = val; },
                ("issue", val) => { self.issue = val; },
                ("number", val) => { self.number = val; },
                ("publisher", val) => { self.publisher = val; },
                ("language", val) => { self.language = val; },
                (field, _) => unimplemented!("setting field {} on a release", field),
            }
        }
        Ok(())
    }
}

impl ApiModelMutate for models::ContainerEntity {
    fn mutate(&mut self, mutations: Vec<Mutation>) -> Result<()> {
        for m in mutations {
            match (m.field.as_str(), m.value) {
                ("name", val) => { self.name = val; },
                ("container_type", val) => { self.container_type = val; },
                ("publisher", val) => { self.publisher = val; },
                ("issnl", val) => { self.issnl = val; },
                (field, _) => unimplemented!("setting field {} on a container", field),
            }
        }
        Ok(())
    }
}

impl ApiModelMutate for models::CreatorEntity {
    fn mutate(&mut self, mutations: Vec<Mutation>) -> Result<()> {
        for m in mutations {
            match (m.field.as_str(), m.value) {
                ("display_name", val) => { self.display_name = val; },
                ("given_name", val) => { self.given_name = val; },
                ("surname", val) => { self.surname = val; },
                (field, _) => unimplemented!("setting field {} on a creator", field),
            }
        }
        Ok(())
    }
}

impl ApiModelMutate for models::WorkEntity {
    fn mutate(&mut self, _mutations: Vec<Mutation>) -> Result<()> {
        unimplemented!("mutations")
    }
}

impl ApiModelMutate for models::FileEntity {
    fn mutate(&mut self, mutations: Vec<Mutation>) -> Result<()> {
        for m in mutations {
            match (m.field.as_str(), m.value) {
                ("size", Some(val)) => { self.size = Some(i64::from_str(&val)?); },
                ("size", None) => { self.size = None; },
                ("md5", val) => { self.md5 = val; },
                ("sha1", val) => { self.sha1 = val; },
                ("sha256", val) => { self.sha256 = val; },
                ("mimetype", val) => { self.mimetype = val; },
                (field, _) => unimplemented!("setting field {} on a file", field),
            }
        }
        Ok(())
    }
}

impl ApiModelMutate for models::FilesetEntity {
    fn mutate(&mut self, _mutations: Vec<Mutation>) -> Result<()> {
        unimplemented!("mutations")
    }
}

impl ApiModelMutate for models::WebcaptureEntity {
    fn mutate(&mut self, _mutations: Vec<Mutation>) -> Result<()> {
        unimplemented!("mutations")
    }
}

impl ApiModelMutate for models::Editor {
    fn mutate(&mut self, mutations: Vec<Mutation>) -> Result<()> {
        for m in mutations {
            match (m.field.as_str(), m.value) {
                ("username", Some(val)) => { self.username = val; },
                (field, _) => unimplemented!("setting field {} on an editor", field),
            }
        }
        Ok(())
    }
}

impl ApiModelMutate for models::Editgroup {
    fn mutate(&mut self, mutations: Vec<Mutation>) -> Result<()> {
        for m in mutations {
            match (m.field.as_str(), m.value) {
                ("description", val) => { self.description = val; },
                (field, _) => unimplemented!("setting field {} on an editgroup", field),
            }
        }
        Ok(())
    }
}

impl ApiModelMutate for models::ChangelogEntry {
    fn mutate(&mut self, _mutations: Vec<Mutation>) -> Result<()> {
        unimplemented!("mutations")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutation_from_str() -> () {
        assert!(Mutation::from_str("release_asdf").is_err());
        assert_eq!(Mutation::from_str("title=blah").unwrap(),
            Mutation { field: "title".to_string(), value: Some("blah".to_string()) });
        assert_eq!(Mutation::from_str("title=").unwrap(),
            Mutation { field: "title".to_string(), value: None });
        assert_eq!(Mutation::from_str("title=string with spaces and stuff").unwrap(),
            Mutation { field: "title".to_string(), value: Some("string with spaces and stuff".to_string()) });
    }

}
