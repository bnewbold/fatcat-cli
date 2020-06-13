
use log::{self,info,debug};
use hyper::client::ResponseFuture;
use fatcat_openapi;
use fatcat_openapi::{ApiNoContext, ApiError, ContextWrapperExt};
use fatcat_openapi::client::Client;
use fatcat_openapi::models;
use swagger::{AuthData, ContextBuilder, EmptyContext, Push, XSpanIdString, auth};
use anyhow::{Result, anyhow, Context};
use crate::{ClientStatus,parse_macaroon_editor_id,Specifier, EntityType};
use tokio::runtime::current_thread::Runtime;


pub struct FatcatApiClient<'a> {
    pub api: fatcat_openapi::ContextWrapper<'a, Client<ResponseFuture>, swagger::make_context_ty!( ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString)>,
    pub rt: tokio::runtime::current_thread::Runtime,
    api_token: Option<String>,
    api_host: String,
    pub editor_id: Option<String>,
}

impl<'a> FatcatApiClient<'a> {

    pub fn new(client: &'a fatcat_openapi::client::Client<ResponseFuture>, api_host: String, api_token: Option<String>) -> Result<Self> {

        let auth_data = match api_token {
            Some(ref token) => Some(AuthData::Bearer(auth::Bearer{ token: token.clone() })),
            None => None,
        };
        //info!("{:?}", auth_data);
        let context: swagger::make_context_ty!(
            ContextBuilder,
            EmptyContext,
            Option<AuthData>,
            XSpanIdString
        ) = swagger::make_context!(
            ContextBuilder,
            EmptyContext,
            auth_data,
            XSpanIdString::default()
        );

        let wrapped_client: fatcat_openapi::ContextWrapper<Client<ResponseFuture>, swagger::make_context_ty!(
            ContextBuilder,
            EmptyContext,
            Option<AuthData>,
            XSpanIdString
        )> = client.with_context(context);
        let rt: Runtime = Runtime::new().expect("create tokio runtime");

        let editor_id = match api_token {
            Some(ref token) => Some(parse_macaroon_editor_id(token).context("parse API auth token")?),
            None => None,
        };
        
        Ok(FatcatApiClient {
            api: wrapped_client,
            rt,
            api_token: api_token,
            editor_id,
            api_host,
        })
    }

    pub fn status(&mut self) -> Result<ClientStatus> {
        let last_changelog = match self.rt.block_on(self.api.get_changelog(Some(1))) {
            Ok(fatcat_openapi::GetChangelogResponse::Success(entry_vec)) => Some(entry_vec[0].index),
            Ok(_) | Err(_) => None,
        };
        let has_api_token = self.api_token.is_some();
        let account: Option<models::Editor> = if has_api_token && last_changelog.is_some() {
            match self.rt.block_on(self.api.auth_check(None)).context("check auth token")? {
                fatcat_openapi::AuthCheckResponse::Success(_) => Ok(()),
                fatcat_openapi::AuthCheckResponse::Forbidden(err) => Err(anyhow!("Forbidden ({}): {}", err.error, err.message)),
                fatcat_openapi::AuthCheckResponse::NotAuthorized{body: err, ..} => Err(anyhow!("Bad Request ({}): {}", err.error, err.message)),
                resp => Err(anyhow!("{:?}", resp)).context(format!("auth check failed"))?,
            }.context("check auth token")?;
            match self.rt.block_on(self.api.get_editor(self.editor_id.as_ref().unwrap().to_string())).context("fetching editor account info")? {
                fatcat_openapi::GetEditorResponse::Found(editor) => Some(editor),
                fatcat_openapi::GetEditorResponse::NotFound(err) => Err(anyhow!("Not Found: {}", err.message))?,
                resp => Err(anyhow!("{:?}", resp)).context(format!("editor fetch failed"))?,
            }
        } else {
            None
        };
        Ok(ClientStatus {
            api_host: self.api_host.clone(),
            has_api_token,
            last_changelog,
            account,
        })
    }

    pub fn update_editgroup_submit(&mut self, editgroup_id: String, submit: bool) -> Result<models::Editgroup> {
        let result = self.rt.block_on(
            self.api.get_editgroup(editgroup_id.clone())
        ).context("fetch editgroups")?;
        let eg = match result {
            fatcat_openapi::GetEditgroupResponse::Found(eg) => eg,
            other => Err(anyhow!("{:?}", other))
                .context(format!("failed to fetch editgroup {}", editgroup_id))?,
        };
        let result = self.rt.block_on(
            self.api.update_editgroup(editgroup_id.clone(), eg, Some(submit))
        ).context("submit editgroup")?;
        match result {
            fatcat_openapi::UpdateEditgroupResponse::UpdatedEditgroup(eg) => Ok(eg),
            other => Err(anyhow!("{:?}", other))
                .context(format!("failed to submit editgroup {}", editgroup_id))?,
        }
    }

    pub fn delete_entity(&mut self, specifier: Specifier, editgroup_id: String) -> Result<models::EntityEdit> {
        use Specifier::*;
        let specifier = specifier.into_entity_specifier(self)?;
        match specifier.clone() {
            Release(fcid) => match self.rt.block_on(self.api.delete_release(editgroup_id, fcid))? {
                fatcat_openapi::DeleteReleaseResponse::DeletedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            Work(fcid) => match self.rt.block_on(self.api.delete_work(editgroup_id, fcid))? {
                fatcat_openapi::DeleteWorkResponse::DeletedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            Container(fcid) => match self.rt.block_on(self.api.delete_container(editgroup_id, fcid))? {
                fatcat_openapi::DeleteContainerResponse::DeletedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            Creator(fcid) => match self.rt.block_on(self.api.delete_creator(editgroup_id, fcid))? {
                fatcat_openapi::DeleteCreatorResponse::DeletedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            File(fcid) => match self.rt.block_on(self.api.delete_file(editgroup_id, fcid))? {
                fatcat_openapi::DeleteFileResponse::DeletedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            FileSet(fcid) => match self.rt.block_on(self.api.delete_fileset(editgroup_id, fcid))? {
                fatcat_openapi::DeleteFilesetResponse::DeletedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            WebCapture(fcid) => match self.rt.block_on(self.api.delete_webcapture(editgroup_id, fcid))? {
                fatcat_openapi::DeleteWebcaptureResponse::DeletedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            Editgroup(..) | Editor(..) => unimplemented!("deletion for this entity type"),
            Changelog(..) => Err(anyhow!("mutating this entity type doesn't make sense"))?,
            EditorUsername(..) | ReleaseLookup(..) | ContainerLookup(..) | FileLookup(..) | CreatorLookup(..) =>
                Err(anyhow!("into_entity_specifier() didn't work?"))?,
        }.context(format!("failed to delete {:?}", specifier))
    }

    pub fn create_entity_from_json(&mut self, entity_type: EntityType, json_str: &str, editgroup_id: String) -> Result<models::EntityEdit> {
        match entity_type {
            EntityType::Release => {
                match self.rt.block_on(self.api.create_release(editgroup_id, serde_json::from_str(&json_str)?))? {
                    fatcat_openapi::CreateReleaseResponse::CreatedEntity(ee) => Ok(ee),
                    other => Err(anyhow!("{:?}", other)),
                }
            },
            EntityType::Work => {
                match self.rt.block_on(self.api.create_work(editgroup_id, serde_json::from_str(&json_str)?))? {
                    fatcat_openapi::CreateWorkResponse::CreatedEntity(ee) => Ok(ee),
                    other => Err(anyhow!("{:?}", other)),
                }
            },
            EntityType::Creator => {
                match self.rt.block_on(self.api.create_creator(editgroup_id, serde_json::from_str(&json_str)?))? {
                    fatcat_openapi::CreateCreatorResponse::CreatedEntity(ee) => Ok(ee),
                    other => Err(anyhow!("{:?}", other)),
                }
            },
            EntityType::Container => {
                match self.rt.block_on(self.api.create_container(editgroup_id, serde_json::from_str(&json_str)?))? {
                    fatcat_openapi::CreateContainerResponse::CreatedEntity(ee) => Ok(ee),
                    other => Err(anyhow!("{:?}", other)),
                }
            },
            EntityType::File => {
                match self.rt.block_on(self.api.create_file(editgroup_id, serde_json::from_str(&json_str)?))? {
                    fatcat_openapi::CreateFileResponse::CreatedEntity(ee) => Ok(ee),
                    other => Err(anyhow!("{:?}", other)),
                }
            },
            EntityType::FileSet => {
                match self.rt.block_on(self.api.create_fileset(editgroup_id, serde_json::from_str(&json_str)?))? {
                    fatcat_openapi::CreateFilesetResponse::CreatedEntity(ee) => Ok(ee),
                    other => Err(anyhow!("{:?}", other)),
                }
            },
            EntityType::WebCapture => {
                match self.rt.block_on(self.api.create_webcapture(editgroup_id, serde_json::from_str(&json_str)?))? {
                    fatcat_openapi::CreateWebcaptureResponse::CreatedEntity(ee) => Ok(ee),
                    other => Err(anyhow!("{:?}", other)),
                }
            },
        }.context(format!("parsing and creating {:?} entity", entity_type))
    }

    pub fn update_entity_from_json(&mut self, specifier: Specifier, json_str: &str, editgroup_id: String) -> Result<models::EntityEdit> {
        use Specifier::*;
        let specifier = specifier.into_entity_specifier(self)?;
        match specifier.clone() {
            Release(fcid) => match self.rt.block_on(self.api.update_release(editgroup_id, fcid, serde_json::from_str(&json_str)?))? {
                fatcat_openapi::UpdateReleaseResponse::UpdatedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            Work(fcid) => match self.rt.block_on(self.api.update_work(editgroup_id, fcid, serde_json::from_str(&json_str)?))? {
                fatcat_openapi::UpdateWorkResponse::UpdatedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            Container(fcid) => match self.rt.block_on(self.api.update_container(editgroup_id, fcid, serde_json::from_str(&json_str)?))? {
                fatcat_openapi::UpdateContainerResponse::UpdatedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            Creator(fcid) => match self.rt.block_on(self.api.update_creator(editgroup_id, fcid, serde_json::from_str(&json_str)?))? {
                fatcat_openapi::UpdateCreatorResponse::UpdatedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            File(fcid) => match self.rt.block_on(self.api.update_file(editgroup_id, fcid, serde_json::from_str(&json_str)?))? {
                fatcat_openapi::UpdateFileResponse::UpdatedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            FileSet(fcid) => match self.rt.block_on(self.api.update_fileset(editgroup_id, fcid, serde_json::from_str(&json_str)?))? {
                fatcat_openapi::UpdateFilesetResponse::UpdatedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            WebCapture(fcid) => match self.rt.block_on(self.api.update_webcapture(editgroup_id, fcid, serde_json::from_str(&json_str)?))? {
                fatcat_openapi::UpdateWebcaptureResponse::UpdatedEntity(ee) => Ok(ee),
                other => Err(anyhow!("{:?}", other)),
            },
            Editgroup(..) | Editor(..) => unimplemented!("updates for this entity type"),
            Changelog(..) => Err(anyhow!("deleting this entity type doesn't make sense"))?,
            EditorUsername(..) | ReleaseLookup(..) | ContainerLookup(..) | FileLookup(..) | CreatorLookup(..) =>
                Err(anyhow!("into_entity_specifier() didn't work?"))?,
        }.context(format!("failed to update {:?}", specifier))
    }
}
