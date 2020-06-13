#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]

use futures::Stream;
use std::io::Error;

#[deprecated(note = "Import futures directly")]
pub use futures::Future;
#[deprecated(note = "Import swagger-rs directly")]
pub use swagger::{ApiError, ContextWrapper};

pub const BASE_PATH: &'static str = "/v0";
pub const API_VERSION: &'static str = "0.3.1";

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AcceptEditgroupResponse {
    /// Merged Successfully
    MergedSuccessfully(models::Success),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Edit Conflict
    EditConflict(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AuthCheckResponse {
    /// Success
    Success(models::Success),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum AuthOidcResponse {
    /// Found
    Found(models::AuthOidcResult),
    /// Created
    Created(models::AuthOidcResult),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Conflict
    Conflict(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateAuthTokenResponse {
    /// Success
    Success(models::AuthTokenResult),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateContainerResponse {
    /// Created Entity
    CreatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateContainerAutoBatchResponse {
    /// Created Editgroup
    CreatedEditgroup(models::Editgroup),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateCreatorResponse {
    /// Created Entity
    CreatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateCreatorAutoBatchResponse {
    /// Created Editgroup
    CreatedEditgroup(models::Editgroup),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateEditgroupResponse {
    /// Successfully Created
    SuccessfullyCreated(models::Editgroup),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateEditgroupAnnotationResponse {
    /// Created
    Created(models::EditgroupAnnotation),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateFileResponse {
    /// Created Entity
    CreatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateFileAutoBatchResponse {
    /// Created Editgroup
    CreatedEditgroup(models::Editgroup),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateFilesetResponse {
    /// Created Entity
    CreatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateFilesetAutoBatchResponse {
    /// Created Editgroup
    CreatedEditgroup(models::Editgroup),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateReleaseResponse {
    /// Created Entity
    CreatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateReleaseAutoBatchResponse {
    /// Created Editgroup
    CreatedEditgroup(models::Editgroup),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateWebcaptureResponse {
    /// Created Entity
    CreatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateWebcaptureAutoBatchResponse {
    /// Created Editgroup
    CreatedEditgroup(models::Editgroup),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateWorkResponse {
    /// Created Entity
    CreatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum CreateWorkAutoBatchResponse {
    /// Created Editgroup
    CreatedEditgroup(models::Editgroup),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteContainerResponse {
    /// Deleted Entity
    DeletedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteContainerEditResponse {
    /// Deleted Edit
    DeletedEdit(models::Success),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteCreatorResponse {
    /// Deleted Entity
    DeletedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteCreatorEditResponse {
    /// Deleted Edit
    DeletedEdit(models::Success),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteFileResponse {
    /// Deleted Entity
    DeletedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteFileEditResponse {
    /// Deleted Edit
    DeletedEdit(models::Success),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteFilesetResponse {
    /// Deleted Entity
    DeletedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteFilesetEditResponse {
    /// Deleted Edit
    DeletedEdit(models::Success),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteReleaseResponse {
    /// Deleted Entity
    DeletedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteReleaseEditResponse {
    /// Deleted Edit
    DeletedEdit(models::Success),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteWebcaptureResponse {
    /// Deleted Entity
    DeletedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteWebcaptureEditResponse {
    /// Deleted Edit
    DeletedEdit(models::Success),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteWorkResponse {
    /// Deleted Entity
    DeletedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum DeleteWorkEditResponse {
    /// Deleted Edit
    DeletedEdit(models::Success),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetChangelogResponse {
    /// Success
    Success(Vec<models::ChangelogEntry>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetChangelogEntryResponse {
    /// Found Changelog Entry
    FoundChangelogEntry(models::ChangelogEntry),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetContainerResponse {
    /// Found Entity
    FoundEntity(models::ContainerEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetContainerEditResponse {
    /// Found Edit
    FoundEdit(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetContainerHistoryResponse {
    /// Found Entity History
    FoundEntityHistory(Vec<models::EntityHistoryEntry>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetContainerRedirectsResponse {
    /// Found Entity Redirects
    FoundEntityRedirects(Vec<String>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetContainerRevisionResponse {
    /// Found Entity Revision
    FoundEntityRevision(models::ContainerEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetCreatorResponse {
    /// Found Entity
    FoundEntity(models::CreatorEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetCreatorEditResponse {
    /// Found Edit
    FoundEdit(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetCreatorHistoryResponse {
    /// Found Entity History
    FoundEntityHistory(Vec<models::EntityHistoryEntry>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetCreatorRedirectsResponse {
    /// Found Entity Redirects
    FoundEntityRedirects(Vec<String>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetCreatorReleasesResponse {
    /// Found
    Found(Vec<models::ReleaseEntity>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetCreatorRevisionResponse {
    /// Found Entity Revision
    FoundEntityRevision(models::CreatorEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetEditgroupResponse {
    /// Found
    Found(models::Editgroup),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetEditgroupAnnotationsResponse {
    /// Success
    Success(Vec<models::EditgroupAnnotation>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetEditgroupsReviewableResponse {
    /// Found
    Found(Vec<models::Editgroup>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetEditorResponse {
    /// Found
    Found(models::Editor),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetEditorAnnotationsResponse {
    /// Success
    Success(Vec<models::EditgroupAnnotation>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetEditorEditgroupsResponse {
    /// Found
    Found(Vec<models::Editgroup>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFileResponse {
    /// Found Entity
    FoundEntity(models::FileEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFileEditResponse {
    /// Found Edit
    FoundEdit(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFileHistoryResponse {
    /// Found Entity History
    FoundEntityHistory(Vec<models::EntityHistoryEntry>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFileRedirectsResponse {
    /// Found Entity Redirects
    FoundEntityRedirects(Vec<String>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFileRevisionResponse {
    /// Found Entity Revision
    FoundEntityRevision(models::FileEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFilesetResponse {
    /// Found Entity
    FoundEntity(models::FilesetEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFilesetEditResponse {
    /// Found Edit
    FoundEdit(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFilesetHistoryResponse {
    /// Found Entity History
    FoundEntityHistory(Vec<models::EntityHistoryEntry>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFilesetRedirectsResponse {
    /// Found Entity Redirects
    FoundEntityRedirects(Vec<String>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetFilesetRevisionResponse {
    /// Found Entity Revision
    FoundEntityRevision(models::FilesetEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetReleaseResponse {
    /// Found Entity
    FoundEntity(models::ReleaseEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetReleaseEditResponse {
    /// Found Edit
    FoundEdit(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetReleaseFilesResponse {
    /// Found
    Found(Vec<models::FileEntity>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetReleaseFilesetsResponse {
    /// Found
    Found(Vec<models::FilesetEntity>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetReleaseHistoryResponse {
    /// Found Entity History
    FoundEntityHistory(Vec<models::EntityHistoryEntry>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetReleaseRedirectsResponse {
    /// Found Entity Redirects
    FoundEntityRedirects(Vec<String>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetReleaseRevisionResponse {
    /// Found Entity Revision
    FoundEntityRevision(models::ReleaseEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetReleaseWebcapturesResponse {
    /// Found
    Found(Vec<models::WebcaptureEntity>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWebcaptureResponse {
    /// Found Entity
    FoundEntity(models::WebcaptureEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWebcaptureEditResponse {
    /// Found Edit
    FoundEdit(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWebcaptureHistoryResponse {
    /// Found Entity History
    FoundEntityHistory(Vec<models::EntityHistoryEntry>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWebcaptureRedirectsResponse {
    /// Found Entity Redirects
    FoundEntityRedirects(Vec<String>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWebcaptureRevisionResponse {
    /// Found Entity Revision
    FoundEntityRevision(models::WebcaptureEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWorkResponse {
    /// Found Entity
    FoundEntity(models::WorkEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWorkEditResponse {
    /// Found Edit
    FoundEdit(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWorkHistoryResponse {
    /// Found Entity History
    FoundEntityHistory(Vec<models::EntityHistoryEntry>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWorkRedirectsResponse {
    /// Found Entity Redirects
    FoundEntityRedirects(Vec<String>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWorkReleasesResponse {
    /// Found
    Found(Vec<models::ReleaseEntity>),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetWorkRevisionResponse {
    /// Found Entity Revision
    FoundEntityRevision(models::WorkEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum LookupContainerResponse {
    /// Found Entity
    FoundEntity(models::ContainerEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum LookupCreatorResponse {
    /// Found Entity
    FoundEntity(models::CreatorEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum LookupFileResponse {
    /// Found Entity
    FoundEntity(models::FileEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum LookupReleaseResponse {
    /// Found Entity
    FoundEntity(models::ReleaseEntity),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateContainerResponse {
    /// Updated Entity
    UpdatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateCreatorResponse {
    /// Updated Entity
    UpdatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateEditgroupResponse {
    /// Updated Editgroup
    UpdatedEditgroup(models::Editgroup),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateEditorResponse {
    /// Updated Editor
    UpdatedEditor(models::Editor),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateFileResponse {
    /// Updated Entity
    UpdatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateFilesetResponse {
    /// Updated Entity
    UpdatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateReleaseResponse {
    /// Updated Entity
    UpdatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateWebcaptureResponse {
    /// Updated Entity
    UpdatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum UpdateWorkResponse {
    /// Updated Entity
    UpdatedEntity(models::EntityEdit),
    /// Bad Request
    BadRequest(models::ErrorResponse),
    /// Not Authorized
    NotAuthorized {
        body: models::ErrorResponse,
        www_authenticate: String,
    },
    /// Forbidden
    Forbidden(models::ErrorResponse),
    /// Not Found
    NotFound(models::ErrorResponse),
    /// Generic Error
    GenericError(models::ErrorResponse),
}

/// API
pub trait Api<C> {
    fn accept_editgroup(
        &self,
        editgroup_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = AcceptEditgroupResponse, Error = ApiError> + Send>;

    fn auth_check(
        &self,
        role: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = AuthCheckResponse, Error = ApiError> + Send>;

    fn auth_oidc(
        &self,
        auth_oidc: models::AuthOidc,
        context: &C,
    ) -> Box<dyn Future<Item = AuthOidcResponse, Error = ApiError> + Send>;

    fn create_auth_token(
        &self,
        editor_id: String,
        duration_seconds: Option<i32>,
        context: &C,
    ) -> Box<dyn Future<Item = CreateAuthTokenResponse, Error = ApiError> + Send>;

    fn create_container(
        &self,
        editgroup_id: String,
        container_entity: models::ContainerEntity,
        context: &C,
    ) -> Box<dyn Future<Item = CreateContainerResponse, Error = ApiError> + Send>;

    fn create_container_auto_batch(
        &self,
        container_auto_batch: models::ContainerAutoBatch,
        context: &C,
    ) -> Box<dyn Future<Item = CreateContainerAutoBatchResponse, Error = ApiError> + Send>;

    fn create_creator(
        &self,
        editgroup_id: String,
        creator_entity: models::CreatorEntity,
        context: &C,
    ) -> Box<dyn Future<Item = CreateCreatorResponse, Error = ApiError> + Send>;

    fn create_creator_auto_batch(
        &self,
        creator_auto_batch: models::CreatorAutoBatch,
        context: &C,
    ) -> Box<dyn Future<Item = CreateCreatorAutoBatchResponse, Error = ApiError> + Send>;

    fn create_editgroup(
        &self,
        editgroup: models::Editgroup,
        context: &C,
    ) -> Box<dyn Future<Item = CreateEditgroupResponse, Error = ApiError> + Send>;

    fn create_editgroup_annotation(
        &self,
        editgroup_id: String,
        editgroup_annotation: models::EditgroupAnnotation,
        context: &C,
    ) -> Box<dyn Future<Item = CreateEditgroupAnnotationResponse, Error = ApiError> + Send>;

    fn create_file(
        &self,
        editgroup_id: String,
        file_entity: models::FileEntity,
        context: &C,
    ) -> Box<dyn Future<Item = CreateFileResponse, Error = ApiError> + Send>;

    fn create_file_auto_batch(
        &self,
        file_auto_batch: models::FileAutoBatch,
        context: &C,
    ) -> Box<dyn Future<Item = CreateFileAutoBatchResponse, Error = ApiError> + Send>;

    fn create_fileset(
        &self,
        editgroup_id: String,
        fileset_entity: models::FilesetEntity,
        context: &C,
    ) -> Box<dyn Future<Item = CreateFilesetResponse, Error = ApiError> + Send>;

    fn create_fileset_auto_batch(
        &self,
        fileset_auto_batch: models::FilesetAutoBatch,
        context: &C,
    ) -> Box<dyn Future<Item = CreateFilesetAutoBatchResponse, Error = ApiError> + Send>;

    fn create_release(
        &self,
        editgroup_id: String,
        release_entity: models::ReleaseEntity,
        context: &C,
    ) -> Box<dyn Future<Item = CreateReleaseResponse, Error = ApiError> + Send>;

    fn create_release_auto_batch(
        &self,
        release_auto_batch: models::ReleaseAutoBatch,
        context: &C,
    ) -> Box<dyn Future<Item = CreateReleaseAutoBatchResponse, Error = ApiError> + Send>;

    fn create_webcapture(
        &self,
        editgroup_id: String,
        webcapture_entity: models::WebcaptureEntity,
        context: &C,
    ) -> Box<dyn Future<Item = CreateWebcaptureResponse, Error = ApiError> + Send>;

    fn create_webcapture_auto_batch(
        &self,
        webcapture_auto_batch: models::WebcaptureAutoBatch,
        context: &C,
    ) -> Box<dyn Future<Item = CreateWebcaptureAutoBatchResponse, Error = ApiError> + Send>;

    fn create_work(
        &self,
        editgroup_id: String,
        work_entity: models::WorkEntity,
        context: &C,
    ) -> Box<dyn Future<Item = CreateWorkResponse, Error = ApiError> + Send>;

    fn create_work_auto_batch(
        &self,
        work_auto_batch: models::WorkAutoBatch,
        context: &C,
    ) -> Box<dyn Future<Item = CreateWorkAutoBatchResponse, Error = ApiError> + Send>;

    fn delete_container(
        &self,
        editgroup_id: String,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteContainerResponse, Error = ApiError> + Send>;

    fn delete_container_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteContainerEditResponse, Error = ApiError> + Send>;

    fn delete_creator(
        &self,
        editgroup_id: String,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteCreatorResponse, Error = ApiError> + Send>;

    fn delete_creator_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteCreatorEditResponse, Error = ApiError> + Send>;

    fn delete_file(
        &self,
        editgroup_id: String,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteFileResponse, Error = ApiError> + Send>;

    fn delete_file_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteFileEditResponse, Error = ApiError> + Send>;

    fn delete_fileset(
        &self,
        editgroup_id: String,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteFilesetResponse, Error = ApiError> + Send>;

    fn delete_fileset_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteFilesetEditResponse, Error = ApiError> + Send>;

    fn delete_release(
        &self,
        editgroup_id: String,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteReleaseResponse, Error = ApiError> + Send>;

    fn delete_release_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteReleaseEditResponse, Error = ApiError> + Send>;

    fn delete_webcapture(
        &self,
        editgroup_id: String,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteWebcaptureResponse, Error = ApiError> + Send>;

    fn delete_webcapture_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteWebcaptureEditResponse, Error = ApiError> + Send>;

    fn delete_work(
        &self,
        editgroup_id: String,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteWorkResponse, Error = ApiError> + Send>;

    fn delete_work_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = DeleteWorkEditResponse, Error = ApiError> + Send>;

    fn get_changelog(
        &self,
        limit: Option<i64>,
        context: &C,
    ) -> Box<dyn Future<Item = GetChangelogResponse, Error = ApiError> + Send>;

    fn get_changelog_entry(
        &self,
        index: i64,
        context: &C,
    ) -> Box<dyn Future<Item = GetChangelogEntryResponse, Error = ApiError> + Send>;

    fn get_container(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetContainerResponse, Error = ApiError> + Send>;

    fn get_container_edit(
        &self,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetContainerEditResponse, Error = ApiError> + Send>;

    fn get_container_history(
        &self,
        ident: String,
        limit: Option<i64>,
        context: &C,
    ) -> Box<dyn Future<Item = GetContainerHistoryResponse, Error = ApiError> + Send>;

    fn get_container_redirects(
        &self,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetContainerRedirectsResponse, Error = ApiError> + Send>;

    fn get_container_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetContainerRevisionResponse, Error = ApiError> + Send>;

    fn get_creator(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetCreatorResponse, Error = ApiError> + Send>;

    fn get_creator_edit(
        &self,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetCreatorEditResponse, Error = ApiError> + Send>;

    fn get_creator_history(
        &self,
        ident: String,
        limit: Option<i64>,
        context: &C,
    ) -> Box<dyn Future<Item = GetCreatorHistoryResponse, Error = ApiError> + Send>;

    fn get_creator_redirects(
        &self,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetCreatorRedirectsResponse, Error = ApiError> + Send>;

    fn get_creator_releases(
        &self,
        ident: String,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetCreatorReleasesResponse, Error = ApiError> + Send>;

    fn get_creator_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetCreatorRevisionResponse, Error = ApiError> + Send>;

    fn get_editgroup(
        &self,
        editgroup_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetEditgroupResponse, Error = ApiError> + Send>;

    fn get_editgroup_annotations(
        &self,
        editgroup_id: String,
        expand: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetEditgroupAnnotationsResponse, Error = ApiError> + Send>;

    fn get_editgroups_reviewable(
        &self,
        expand: Option<String>,
        limit: Option<i64>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        since: Option<chrono::DateTime<chrono::Utc>>,
        context: &C,
    ) -> Box<dyn Future<Item = GetEditgroupsReviewableResponse, Error = ApiError> + Send>;

    fn get_editor(
        &self,
        editor_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetEditorResponse, Error = ApiError> + Send>;

    fn get_editor_annotations(
        &self,
        editor_id: String,
        limit: Option<i64>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        since: Option<chrono::DateTime<chrono::Utc>>,
        context: &C,
    ) -> Box<dyn Future<Item = GetEditorAnnotationsResponse, Error = ApiError> + Send>;

    fn get_editor_editgroups(
        &self,
        editor_id: String,
        limit: Option<i64>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        since: Option<chrono::DateTime<chrono::Utc>>,
        context: &C,
    ) -> Box<dyn Future<Item = GetEditorEditgroupsResponse, Error = ApiError> + Send>;

    fn get_file(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetFileResponse, Error = ApiError> + Send>;

    fn get_file_edit(
        &self,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetFileEditResponse, Error = ApiError> + Send>;

    fn get_file_history(
        &self,
        ident: String,
        limit: Option<i64>,
        context: &C,
    ) -> Box<dyn Future<Item = GetFileHistoryResponse, Error = ApiError> + Send>;

    fn get_file_redirects(
        &self,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetFileRedirectsResponse, Error = ApiError> + Send>;

    fn get_file_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetFileRevisionResponse, Error = ApiError> + Send>;

    fn get_fileset(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetFilesetResponse, Error = ApiError> + Send>;

    fn get_fileset_edit(
        &self,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetFilesetEditResponse, Error = ApiError> + Send>;

    fn get_fileset_history(
        &self,
        ident: String,
        limit: Option<i64>,
        context: &C,
    ) -> Box<dyn Future<Item = GetFilesetHistoryResponse, Error = ApiError> + Send>;

    fn get_fileset_redirects(
        &self,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetFilesetRedirectsResponse, Error = ApiError> + Send>;

    fn get_fileset_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetFilesetRevisionResponse, Error = ApiError> + Send>;

    fn get_release(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetReleaseResponse, Error = ApiError> + Send>;

    fn get_release_edit(
        &self,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetReleaseEditResponse, Error = ApiError> + Send>;

    fn get_release_files(
        &self,
        ident: String,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetReleaseFilesResponse, Error = ApiError> + Send>;

    fn get_release_filesets(
        &self,
        ident: String,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetReleaseFilesetsResponse, Error = ApiError> + Send>;

    fn get_release_history(
        &self,
        ident: String,
        limit: Option<i64>,
        context: &C,
    ) -> Box<dyn Future<Item = GetReleaseHistoryResponse, Error = ApiError> + Send>;

    fn get_release_redirects(
        &self,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetReleaseRedirectsResponse, Error = ApiError> + Send>;

    fn get_release_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetReleaseRevisionResponse, Error = ApiError> + Send>;

    fn get_release_webcaptures(
        &self,
        ident: String,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetReleaseWebcapturesResponse, Error = ApiError> + Send>;

    fn get_webcapture(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetWebcaptureResponse, Error = ApiError> + Send>;

    fn get_webcapture_edit(
        &self,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetWebcaptureEditResponse, Error = ApiError> + Send>;

    fn get_webcapture_history(
        &self,
        ident: String,
        limit: Option<i64>,
        context: &C,
    ) -> Box<dyn Future<Item = GetWebcaptureHistoryResponse, Error = ApiError> + Send>;

    fn get_webcapture_redirects(
        &self,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetWebcaptureRedirectsResponse, Error = ApiError> + Send>;

    fn get_webcapture_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetWebcaptureRevisionResponse, Error = ApiError> + Send>;

    fn get_work(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetWorkResponse, Error = ApiError> + Send>;

    fn get_work_edit(
        &self,
        edit_id: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetWorkEditResponse, Error = ApiError> + Send>;

    fn get_work_history(
        &self,
        ident: String,
        limit: Option<i64>,
        context: &C,
    ) -> Box<dyn Future<Item = GetWorkHistoryResponse, Error = ApiError> + Send>;

    fn get_work_redirects(
        &self,
        ident: String,
        context: &C,
    ) -> Box<dyn Future<Item = GetWorkRedirectsResponse, Error = ApiError> + Send>;

    fn get_work_releases(
        &self,
        ident: String,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetWorkReleasesResponse, Error = ApiError> + Send>;

    fn get_work_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = GetWorkRevisionResponse, Error = ApiError> + Send>;

    fn lookup_container(
        &self,
        issnl: Option<String>,
        wikidata_qid: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = LookupContainerResponse, Error = ApiError> + Send>;

    fn lookup_creator(
        &self,
        orcid: Option<String>,
        wikidata_qid: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = LookupCreatorResponse, Error = ApiError> + Send>;

    fn lookup_file(
        &self,
        md5: Option<String>,
        sha1: Option<String>,
        sha256: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = LookupFileResponse, Error = ApiError> + Send>;

    fn lookup_release(
        &self,
        doi: Option<String>,
        wikidata_qid: Option<String>,
        isbn13: Option<String>,
        pmid: Option<String>,
        pmcid: Option<String>,
        core: Option<String>,
        arxiv: Option<String>,
        jstor: Option<String>,
        ark: Option<String>,
        mag: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
        context: &C,
    ) -> Box<dyn Future<Item = LookupReleaseResponse, Error = ApiError> + Send>;

    fn update_container(
        &self,
        editgroup_id: String,
        ident: String,
        container_entity: models::ContainerEntity,
        context: &C,
    ) -> Box<dyn Future<Item = UpdateContainerResponse, Error = ApiError> + Send>;

    fn update_creator(
        &self,
        editgroup_id: String,
        ident: String,
        creator_entity: models::CreatorEntity,
        context: &C,
    ) -> Box<dyn Future<Item = UpdateCreatorResponse, Error = ApiError> + Send>;

    fn update_editgroup(
        &self,
        editgroup_id: String,
        editgroup: models::Editgroup,
        submit: Option<bool>,
        context: &C,
    ) -> Box<dyn Future<Item = UpdateEditgroupResponse, Error = ApiError> + Send>;

    fn update_editor(
        &self,
        editor_id: String,
        editor: models::Editor,
        context: &C,
    ) -> Box<dyn Future<Item = UpdateEditorResponse, Error = ApiError> + Send>;

    fn update_file(
        &self,
        editgroup_id: String,
        ident: String,
        file_entity: models::FileEntity,
        context: &C,
    ) -> Box<dyn Future<Item = UpdateFileResponse, Error = ApiError> + Send>;

    fn update_fileset(
        &self,
        editgroup_id: String,
        ident: String,
        fileset_entity: models::FilesetEntity,
        context: &C,
    ) -> Box<dyn Future<Item = UpdateFilesetResponse, Error = ApiError> + Send>;

    fn update_release(
        &self,
        editgroup_id: String,
        ident: String,
        release_entity: models::ReleaseEntity,
        context: &C,
    ) -> Box<dyn Future<Item = UpdateReleaseResponse, Error = ApiError> + Send>;

    fn update_webcapture(
        &self,
        editgroup_id: String,
        ident: String,
        webcapture_entity: models::WebcaptureEntity,
        context: &C,
    ) -> Box<dyn Future<Item = UpdateWebcaptureResponse, Error = ApiError> + Send>;

    fn update_work(
        &self,
        editgroup_id: String,
        ident: String,
        work_entity: models::WorkEntity,
        context: &C,
    ) -> Box<dyn Future<Item = UpdateWorkResponse, Error = ApiError> + Send>;
}

/// API without a `Context`
pub trait ApiNoContext {
    fn accept_editgroup(
        &self,
        editgroup_id: String,
    ) -> Box<dyn Future<Item = AcceptEditgroupResponse, Error = ApiError> + Send>;

    fn auth_check(
        &self,
        role: Option<String>,
    ) -> Box<dyn Future<Item = AuthCheckResponse, Error = ApiError> + Send>;

    fn auth_oidc(
        &self,
        auth_oidc: models::AuthOidc,
    ) -> Box<dyn Future<Item = AuthOidcResponse, Error = ApiError> + Send>;

    fn create_auth_token(
        &self,
        editor_id: String,
        duration_seconds: Option<i32>,
    ) -> Box<dyn Future<Item = CreateAuthTokenResponse, Error = ApiError> + Send>;

    fn create_container(
        &self,
        editgroup_id: String,
        container_entity: models::ContainerEntity,
    ) -> Box<dyn Future<Item = CreateContainerResponse, Error = ApiError> + Send>;

    fn create_container_auto_batch(
        &self,
        container_auto_batch: models::ContainerAutoBatch,
    ) -> Box<dyn Future<Item = CreateContainerAutoBatchResponse, Error = ApiError> + Send>;

    fn create_creator(
        &self,
        editgroup_id: String,
        creator_entity: models::CreatorEntity,
    ) -> Box<dyn Future<Item = CreateCreatorResponse, Error = ApiError> + Send>;

    fn create_creator_auto_batch(
        &self,
        creator_auto_batch: models::CreatorAutoBatch,
    ) -> Box<dyn Future<Item = CreateCreatorAutoBatchResponse, Error = ApiError> + Send>;

    fn create_editgroup(
        &self,
        editgroup: models::Editgroup,
    ) -> Box<dyn Future<Item = CreateEditgroupResponse, Error = ApiError> + Send>;

    fn create_editgroup_annotation(
        &self,
        editgroup_id: String,
        editgroup_annotation: models::EditgroupAnnotation,
    ) -> Box<dyn Future<Item = CreateEditgroupAnnotationResponse, Error = ApiError> + Send>;

    fn create_file(
        &self,
        editgroup_id: String,
        file_entity: models::FileEntity,
    ) -> Box<dyn Future<Item = CreateFileResponse, Error = ApiError> + Send>;

    fn create_file_auto_batch(
        &self,
        file_auto_batch: models::FileAutoBatch,
    ) -> Box<dyn Future<Item = CreateFileAutoBatchResponse, Error = ApiError> + Send>;

    fn create_fileset(
        &self,
        editgroup_id: String,
        fileset_entity: models::FilesetEntity,
    ) -> Box<dyn Future<Item = CreateFilesetResponse, Error = ApiError> + Send>;

    fn create_fileset_auto_batch(
        &self,
        fileset_auto_batch: models::FilesetAutoBatch,
    ) -> Box<dyn Future<Item = CreateFilesetAutoBatchResponse, Error = ApiError> + Send>;

    fn create_release(
        &self,
        editgroup_id: String,
        release_entity: models::ReleaseEntity,
    ) -> Box<dyn Future<Item = CreateReleaseResponse, Error = ApiError> + Send>;

    fn create_release_auto_batch(
        &self,
        release_auto_batch: models::ReleaseAutoBatch,
    ) -> Box<dyn Future<Item = CreateReleaseAutoBatchResponse, Error = ApiError> + Send>;

    fn create_webcapture(
        &self,
        editgroup_id: String,
        webcapture_entity: models::WebcaptureEntity,
    ) -> Box<dyn Future<Item = CreateWebcaptureResponse, Error = ApiError> + Send>;

    fn create_webcapture_auto_batch(
        &self,
        webcapture_auto_batch: models::WebcaptureAutoBatch,
    ) -> Box<dyn Future<Item = CreateWebcaptureAutoBatchResponse, Error = ApiError> + Send>;

    fn create_work(
        &self,
        editgroup_id: String,
        work_entity: models::WorkEntity,
    ) -> Box<dyn Future<Item = CreateWorkResponse, Error = ApiError> + Send>;

    fn create_work_auto_batch(
        &self,
        work_auto_batch: models::WorkAutoBatch,
    ) -> Box<dyn Future<Item = CreateWorkAutoBatchResponse, Error = ApiError> + Send>;

    fn delete_container(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteContainerResponse, Error = ApiError> + Send>;

    fn delete_container_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteContainerEditResponse, Error = ApiError> + Send>;

    fn delete_creator(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteCreatorResponse, Error = ApiError> + Send>;

    fn delete_creator_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteCreatorEditResponse, Error = ApiError> + Send>;

    fn delete_file(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteFileResponse, Error = ApiError> + Send>;

    fn delete_file_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteFileEditResponse, Error = ApiError> + Send>;

    fn delete_fileset(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteFilesetResponse, Error = ApiError> + Send>;

    fn delete_fileset_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteFilesetEditResponse, Error = ApiError> + Send>;

    fn delete_release(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteReleaseResponse, Error = ApiError> + Send>;

    fn delete_release_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteReleaseEditResponse, Error = ApiError> + Send>;

    fn delete_webcapture(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteWebcaptureResponse, Error = ApiError> + Send>;

    fn delete_webcapture_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteWebcaptureEditResponse, Error = ApiError> + Send>;

    fn delete_work(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteWorkResponse, Error = ApiError> + Send>;

    fn delete_work_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteWorkEditResponse, Error = ApiError> + Send>;

    fn get_changelog(
        &self,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetChangelogResponse, Error = ApiError> + Send>;

    fn get_changelog_entry(
        &self,
        index: i64,
    ) -> Box<dyn Future<Item = GetChangelogEntryResponse, Error = ApiError> + Send>;

    fn get_container(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetContainerResponse, Error = ApiError> + Send>;

    fn get_container_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetContainerEditResponse, Error = ApiError> + Send>;

    fn get_container_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetContainerHistoryResponse, Error = ApiError> + Send>;

    fn get_container_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetContainerRedirectsResponse, Error = ApiError> + Send>;

    fn get_container_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetContainerRevisionResponse, Error = ApiError> + Send>;

    fn get_creator(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetCreatorResponse, Error = ApiError> + Send>;

    fn get_creator_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetCreatorEditResponse, Error = ApiError> + Send>;

    fn get_creator_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetCreatorHistoryResponse, Error = ApiError> + Send>;

    fn get_creator_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetCreatorRedirectsResponse, Error = ApiError> + Send>;

    fn get_creator_releases(
        &self,
        ident: String,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetCreatorReleasesResponse, Error = ApiError> + Send>;

    fn get_creator_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetCreatorRevisionResponse, Error = ApiError> + Send>;

    fn get_editgroup(
        &self,
        editgroup_id: String,
    ) -> Box<dyn Future<Item = GetEditgroupResponse, Error = ApiError> + Send>;

    fn get_editgroup_annotations(
        &self,
        editgroup_id: String,
        expand: Option<String>,
    ) -> Box<dyn Future<Item = GetEditgroupAnnotationsResponse, Error = ApiError> + Send>;

    fn get_editgroups_reviewable(
        &self,
        expand: Option<String>,
        limit: Option<i64>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Box<dyn Future<Item = GetEditgroupsReviewableResponse, Error = ApiError> + Send>;

    fn get_editor(
        &self,
        editor_id: String,
    ) -> Box<dyn Future<Item = GetEditorResponse, Error = ApiError> + Send>;

    fn get_editor_annotations(
        &self,
        editor_id: String,
        limit: Option<i64>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Box<dyn Future<Item = GetEditorAnnotationsResponse, Error = ApiError> + Send>;

    fn get_editor_editgroups(
        &self,
        editor_id: String,
        limit: Option<i64>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Box<dyn Future<Item = GetEditorEditgroupsResponse, Error = ApiError> + Send>;

    fn get_file(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetFileResponse, Error = ApiError> + Send>;

    fn get_file_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetFileEditResponse, Error = ApiError> + Send>;

    fn get_file_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetFileHistoryResponse, Error = ApiError> + Send>;

    fn get_file_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetFileRedirectsResponse, Error = ApiError> + Send>;

    fn get_file_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetFileRevisionResponse, Error = ApiError> + Send>;

    fn get_fileset(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetFilesetResponse, Error = ApiError> + Send>;

    fn get_fileset_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetFilesetEditResponse, Error = ApiError> + Send>;

    fn get_fileset_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetFilesetHistoryResponse, Error = ApiError> + Send>;

    fn get_fileset_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetFilesetRedirectsResponse, Error = ApiError> + Send>;

    fn get_fileset_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetFilesetRevisionResponse, Error = ApiError> + Send>;

    fn get_release(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetReleaseResponse, Error = ApiError> + Send>;

    fn get_release_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetReleaseEditResponse, Error = ApiError> + Send>;

    fn get_release_files(
        &self,
        ident: String,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetReleaseFilesResponse, Error = ApiError> + Send>;

    fn get_release_filesets(
        &self,
        ident: String,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetReleaseFilesetsResponse, Error = ApiError> + Send>;

    fn get_release_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetReleaseHistoryResponse, Error = ApiError> + Send>;

    fn get_release_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetReleaseRedirectsResponse, Error = ApiError> + Send>;

    fn get_release_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetReleaseRevisionResponse, Error = ApiError> + Send>;

    fn get_release_webcaptures(
        &self,
        ident: String,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetReleaseWebcapturesResponse, Error = ApiError> + Send>;

    fn get_webcapture(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetWebcaptureResponse, Error = ApiError> + Send>;

    fn get_webcapture_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetWebcaptureEditResponse, Error = ApiError> + Send>;

    fn get_webcapture_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetWebcaptureHistoryResponse, Error = ApiError> + Send>;

    fn get_webcapture_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetWebcaptureRedirectsResponse, Error = ApiError> + Send>;

    fn get_webcapture_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetWebcaptureRevisionResponse, Error = ApiError> + Send>;

    fn get_work(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetWorkResponse, Error = ApiError> + Send>;

    fn get_work_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetWorkEditResponse, Error = ApiError> + Send>;

    fn get_work_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetWorkHistoryResponse, Error = ApiError> + Send>;

    fn get_work_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetWorkRedirectsResponse, Error = ApiError> + Send>;

    fn get_work_releases(
        &self,
        ident: String,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetWorkReleasesResponse, Error = ApiError> + Send>;

    fn get_work_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetWorkRevisionResponse, Error = ApiError> + Send>;

    fn lookup_container(
        &self,
        issnl: Option<String>,
        wikidata_qid: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = LookupContainerResponse, Error = ApiError> + Send>;

    fn lookup_creator(
        &self,
        orcid: Option<String>,
        wikidata_qid: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = LookupCreatorResponse, Error = ApiError> + Send>;

    fn lookup_file(
        &self,
        md5: Option<String>,
        sha1: Option<String>,
        sha256: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = LookupFileResponse, Error = ApiError> + Send>;

    fn lookup_release(
        &self,
        doi: Option<String>,
        wikidata_qid: Option<String>,
        isbn13: Option<String>,
        pmid: Option<String>,
        pmcid: Option<String>,
        core: Option<String>,
        arxiv: Option<String>,
        jstor: Option<String>,
        ark: Option<String>,
        mag: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = LookupReleaseResponse, Error = ApiError> + Send>;

    fn update_container(
        &self,
        editgroup_id: String,
        ident: String,
        container_entity: models::ContainerEntity,
    ) -> Box<dyn Future<Item = UpdateContainerResponse, Error = ApiError> + Send>;

    fn update_creator(
        &self,
        editgroup_id: String,
        ident: String,
        creator_entity: models::CreatorEntity,
    ) -> Box<dyn Future<Item = UpdateCreatorResponse, Error = ApiError> + Send>;

    fn update_editgroup(
        &self,
        editgroup_id: String,
        editgroup: models::Editgroup,
        submit: Option<bool>,
    ) -> Box<dyn Future<Item = UpdateEditgroupResponse, Error = ApiError> + Send>;

    fn update_editor(
        &self,
        editor_id: String,
        editor: models::Editor,
    ) -> Box<dyn Future<Item = UpdateEditorResponse, Error = ApiError> + Send>;

    fn update_file(
        &self,
        editgroup_id: String,
        ident: String,
        file_entity: models::FileEntity,
    ) -> Box<dyn Future<Item = UpdateFileResponse, Error = ApiError> + Send>;

    fn update_fileset(
        &self,
        editgroup_id: String,
        ident: String,
        fileset_entity: models::FilesetEntity,
    ) -> Box<dyn Future<Item = UpdateFilesetResponse, Error = ApiError> + Send>;

    fn update_release(
        &self,
        editgroup_id: String,
        ident: String,
        release_entity: models::ReleaseEntity,
    ) -> Box<dyn Future<Item = UpdateReleaseResponse, Error = ApiError> + Send>;

    fn update_webcapture(
        &self,
        editgroup_id: String,
        ident: String,
        webcapture_entity: models::WebcaptureEntity,
    ) -> Box<dyn Future<Item = UpdateWebcaptureResponse, Error = ApiError> + Send>;

    fn update_work(
        &self,
        editgroup_id: String,
        ident: String,
        work_entity: models::WorkEntity,
    ) -> Box<dyn Future<Item = UpdateWorkResponse, Error = ApiError> + Send>;
}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C>
where
    Self: Sized,
{
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
        ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {
    fn accept_editgroup(
        &self,
        editgroup_id: String,
    ) -> Box<dyn Future<Item = AcceptEditgroupResponse, Error = ApiError> + Send> {
        self.api().accept_editgroup(editgroup_id, &self.context())
    }

    fn auth_check(
        &self,
        role: Option<String>,
    ) -> Box<dyn Future<Item = AuthCheckResponse, Error = ApiError> + Send> {
        self.api().auth_check(role, &self.context())
    }

    fn auth_oidc(
        &self,
        auth_oidc: models::AuthOidc,
    ) -> Box<dyn Future<Item = AuthOidcResponse, Error = ApiError> + Send> {
        self.api().auth_oidc(auth_oidc, &self.context())
    }

    fn create_auth_token(
        &self,
        editor_id: String,
        duration_seconds: Option<i32>,
    ) -> Box<dyn Future<Item = CreateAuthTokenResponse, Error = ApiError> + Send> {
        self.api()
            .create_auth_token(editor_id, duration_seconds, &self.context())
    }

    fn create_container(
        &self,
        editgroup_id: String,
        container_entity: models::ContainerEntity,
    ) -> Box<dyn Future<Item = CreateContainerResponse, Error = ApiError> + Send> {
        self.api()
            .create_container(editgroup_id, container_entity, &self.context())
    }

    fn create_container_auto_batch(
        &self,
        container_auto_batch: models::ContainerAutoBatch,
    ) -> Box<dyn Future<Item = CreateContainerAutoBatchResponse, Error = ApiError> + Send> {
        self.api()
            .create_container_auto_batch(container_auto_batch, &self.context())
    }

    fn create_creator(
        &self,
        editgroup_id: String,
        creator_entity: models::CreatorEntity,
    ) -> Box<dyn Future<Item = CreateCreatorResponse, Error = ApiError> + Send> {
        self.api()
            .create_creator(editgroup_id, creator_entity, &self.context())
    }

    fn create_creator_auto_batch(
        &self,
        creator_auto_batch: models::CreatorAutoBatch,
    ) -> Box<dyn Future<Item = CreateCreatorAutoBatchResponse, Error = ApiError> + Send> {
        self.api()
            .create_creator_auto_batch(creator_auto_batch, &self.context())
    }

    fn create_editgroup(
        &self,
        editgroup: models::Editgroup,
    ) -> Box<dyn Future<Item = CreateEditgroupResponse, Error = ApiError> + Send> {
        self.api().create_editgroup(editgroup, &self.context())
    }

    fn create_editgroup_annotation(
        &self,
        editgroup_id: String,
        editgroup_annotation: models::EditgroupAnnotation,
    ) -> Box<dyn Future<Item = CreateEditgroupAnnotationResponse, Error = ApiError> + Send> {
        self.api()
            .create_editgroup_annotation(editgroup_id, editgroup_annotation, &self.context())
    }

    fn create_file(
        &self,
        editgroup_id: String,
        file_entity: models::FileEntity,
    ) -> Box<dyn Future<Item = CreateFileResponse, Error = ApiError> + Send> {
        self.api()
            .create_file(editgroup_id, file_entity, &self.context())
    }

    fn create_file_auto_batch(
        &self,
        file_auto_batch: models::FileAutoBatch,
    ) -> Box<dyn Future<Item = CreateFileAutoBatchResponse, Error = ApiError> + Send> {
        self.api()
            .create_file_auto_batch(file_auto_batch, &self.context())
    }

    fn create_fileset(
        &self,
        editgroup_id: String,
        fileset_entity: models::FilesetEntity,
    ) -> Box<dyn Future<Item = CreateFilesetResponse, Error = ApiError> + Send> {
        self.api()
            .create_fileset(editgroup_id, fileset_entity, &self.context())
    }

    fn create_fileset_auto_batch(
        &self,
        fileset_auto_batch: models::FilesetAutoBatch,
    ) -> Box<dyn Future<Item = CreateFilesetAutoBatchResponse, Error = ApiError> + Send> {
        self.api()
            .create_fileset_auto_batch(fileset_auto_batch, &self.context())
    }

    fn create_release(
        &self,
        editgroup_id: String,
        release_entity: models::ReleaseEntity,
    ) -> Box<dyn Future<Item = CreateReleaseResponse, Error = ApiError> + Send> {
        self.api()
            .create_release(editgroup_id, release_entity, &self.context())
    }

    fn create_release_auto_batch(
        &self,
        release_auto_batch: models::ReleaseAutoBatch,
    ) -> Box<dyn Future<Item = CreateReleaseAutoBatchResponse, Error = ApiError> + Send> {
        self.api()
            .create_release_auto_batch(release_auto_batch, &self.context())
    }

    fn create_webcapture(
        &self,
        editgroup_id: String,
        webcapture_entity: models::WebcaptureEntity,
    ) -> Box<dyn Future<Item = CreateWebcaptureResponse, Error = ApiError> + Send> {
        self.api()
            .create_webcapture(editgroup_id, webcapture_entity, &self.context())
    }

    fn create_webcapture_auto_batch(
        &self,
        webcapture_auto_batch: models::WebcaptureAutoBatch,
    ) -> Box<dyn Future<Item = CreateWebcaptureAutoBatchResponse, Error = ApiError> + Send> {
        self.api()
            .create_webcapture_auto_batch(webcapture_auto_batch, &self.context())
    }

    fn create_work(
        &self,
        editgroup_id: String,
        work_entity: models::WorkEntity,
    ) -> Box<dyn Future<Item = CreateWorkResponse, Error = ApiError> + Send> {
        self.api()
            .create_work(editgroup_id, work_entity, &self.context())
    }

    fn create_work_auto_batch(
        &self,
        work_auto_batch: models::WorkAutoBatch,
    ) -> Box<dyn Future<Item = CreateWorkAutoBatchResponse, Error = ApiError> + Send> {
        self.api()
            .create_work_auto_batch(work_auto_batch, &self.context())
    }

    fn delete_container(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteContainerResponse, Error = ApiError> + Send> {
        self.api()
            .delete_container(editgroup_id, ident, &self.context())
    }

    fn delete_container_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteContainerEditResponse, Error = ApiError> + Send> {
        self.api()
            .delete_container_edit(editgroup_id, edit_id, &self.context())
    }

    fn delete_creator(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteCreatorResponse, Error = ApiError> + Send> {
        self.api()
            .delete_creator(editgroup_id, ident, &self.context())
    }

    fn delete_creator_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteCreatorEditResponse, Error = ApiError> + Send> {
        self.api()
            .delete_creator_edit(editgroup_id, edit_id, &self.context())
    }

    fn delete_file(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteFileResponse, Error = ApiError> + Send> {
        self.api().delete_file(editgroup_id, ident, &self.context())
    }

    fn delete_file_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteFileEditResponse, Error = ApiError> + Send> {
        self.api()
            .delete_file_edit(editgroup_id, edit_id, &self.context())
    }

    fn delete_fileset(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteFilesetResponse, Error = ApiError> + Send> {
        self.api()
            .delete_fileset(editgroup_id, ident, &self.context())
    }

    fn delete_fileset_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteFilesetEditResponse, Error = ApiError> + Send> {
        self.api()
            .delete_fileset_edit(editgroup_id, edit_id, &self.context())
    }

    fn delete_release(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteReleaseResponse, Error = ApiError> + Send> {
        self.api()
            .delete_release(editgroup_id, ident, &self.context())
    }

    fn delete_release_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteReleaseEditResponse, Error = ApiError> + Send> {
        self.api()
            .delete_release_edit(editgroup_id, edit_id, &self.context())
    }

    fn delete_webcapture(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteWebcaptureResponse, Error = ApiError> + Send> {
        self.api()
            .delete_webcapture(editgroup_id, ident, &self.context())
    }

    fn delete_webcapture_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteWebcaptureEditResponse, Error = ApiError> + Send> {
        self.api()
            .delete_webcapture_edit(editgroup_id, edit_id, &self.context())
    }

    fn delete_work(
        &self,
        editgroup_id: String,
        ident: String,
    ) -> Box<dyn Future<Item = DeleteWorkResponse, Error = ApiError> + Send> {
        self.api().delete_work(editgroup_id, ident, &self.context())
    }

    fn delete_work_edit(
        &self,
        editgroup_id: String,
        edit_id: String,
    ) -> Box<dyn Future<Item = DeleteWorkEditResponse, Error = ApiError> + Send> {
        self.api()
            .delete_work_edit(editgroup_id, edit_id, &self.context())
    }

    fn get_changelog(
        &self,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetChangelogResponse, Error = ApiError> + Send> {
        self.api().get_changelog(limit, &self.context())
    }

    fn get_changelog_entry(
        &self,
        index: i64,
    ) -> Box<dyn Future<Item = GetChangelogEntryResponse, Error = ApiError> + Send> {
        self.api().get_changelog_entry(index, &self.context())
    }

    fn get_container(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetContainerResponse, Error = ApiError> + Send> {
        self.api()
            .get_container(ident, expand, hide, &self.context())
    }

    fn get_container_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetContainerEditResponse, Error = ApiError> + Send> {
        self.api().get_container_edit(edit_id, &self.context())
    }

    fn get_container_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetContainerHistoryResponse, Error = ApiError> + Send> {
        self.api()
            .get_container_history(ident, limit, &self.context())
    }

    fn get_container_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetContainerRedirectsResponse, Error = ApiError> + Send> {
        self.api().get_container_redirects(ident, &self.context())
    }

    fn get_container_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetContainerRevisionResponse, Error = ApiError> + Send> {
        self.api()
            .get_container_revision(rev_id, expand, hide, &self.context())
    }

    fn get_creator(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetCreatorResponse, Error = ApiError> + Send> {
        self.api().get_creator(ident, expand, hide, &self.context())
    }

    fn get_creator_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetCreatorEditResponse, Error = ApiError> + Send> {
        self.api().get_creator_edit(edit_id, &self.context())
    }

    fn get_creator_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetCreatorHistoryResponse, Error = ApiError> + Send> {
        self.api()
            .get_creator_history(ident, limit, &self.context())
    }

    fn get_creator_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetCreatorRedirectsResponse, Error = ApiError> + Send> {
        self.api().get_creator_redirects(ident, &self.context())
    }

    fn get_creator_releases(
        &self,
        ident: String,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetCreatorReleasesResponse, Error = ApiError> + Send> {
        self.api()
            .get_creator_releases(ident, hide, &self.context())
    }

    fn get_creator_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetCreatorRevisionResponse, Error = ApiError> + Send> {
        self.api()
            .get_creator_revision(rev_id, expand, hide, &self.context())
    }

    fn get_editgroup(
        &self,
        editgroup_id: String,
    ) -> Box<dyn Future<Item = GetEditgroupResponse, Error = ApiError> + Send> {
        self.api().get_editgroup(editgroup_id, &self.context())
    }

    fn get_editgroup_annotations(
        &self,
        editgroup_id: String,
        expand: Option<String>,
    ) -> Box<dyn Future<Item = GetEditgroupAnnotationsResponse, Error = ApiError> + Send> {
        self.api()
            .get_editgroup_annotations(editgroup_id, expand, &self.context())
    }

    fn get_editgroups_reviewable(
        &self,
        expand: Option<String>,
        limit: Option<i64>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Box<dyn Future<Item = GetEditgroupsReviewableResponse, Error = ApiError> + Send> {
        self.api()
            .get_editgroups_reviewable(expand, limit, before, since, &self.context())
    }

    fn get_editor(
        &self,
        editor_id: String,
    ) -> Box<dyn Future<Item = GetEditorResponse, Error = ApiError> + Send> {
        self.api().get_editor(editor_id, &self.context())
    }

    fn get_editor_annotations(
        &self,
        editor_id: String,
        limit: Option<i64>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Box<dyn Future<Item = GetEditorAnnotationsResponse, Error = ApiError> + Send> {
        self.api()
            .get_editor_annotations(editor_id, limit, before, since, &self.context())
    }

    fn get_editor_editgroups(
        &self,
        editor_id: String,
        limit: Option<i64>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Box<dyn Future<Item = GetEditorEditgroupsResponse, Error = ApiError> + Send> {
        self.api()
            .get_editor_editgroups(editor_id, limit, before, since, &self.context())
    }

    fn get_file(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetFileResponse, Error = ApiError> + Send> {
        self.api().get_file(ident, expand, hide, &self.context())
    }

    fn get_file_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetFileEditResponse, Error = ApiError> + Send> {
        self.api().get_file_edit(edit_id, &self.context())
    }

    fn get_file_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetFileHistoryResponse, Error = ApiError> + Send> {
        self.api().get_file_history(ident, limit, &self.context())
    }

    fn get_file_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetFileRedirectsResponse, Error = ApiError> + Send> {
        self.api().get_file_redirects(ident, &self.context())
    }

    fn get_file_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetFileRevisionResponse, Error = ApiError> + Send> {
        self.api()
            .get_file_revision(rev_id, expand, hide, &self.context())
    }

    fn get_fileset(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetFilesetResponse, Error = ApiError> + Send> {
        self.api().get_fileset(ident, expand, hide, &self.context())
    }

    fn get_fileset_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetFilesetEditResponse, Error = ApiError> + Send> {
        self.api().get_fileset_edit(edit_id, &self.context())
    }

    fn get_fileset_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetFilesetHistoryResponse, Error = ApiError> + Send> {
        self.api()
            .get_fileset_history(ident, limit, &self.context())
    }

    fn get_fileset_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetFilesetRedirectsResponse, Error = ApiError> + Send> {
        self.api().get_fileset_redirects(ident, &self.context())
    }

    fn get_fileset_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetFilesetRevisionResponse, Error = ApiError> + Send> {
        self.api()
            .get_fileset_revision(rev_id, expand, hide, &self.context())
    }

    fn get_release(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetReleaseResponse, Error = ApiError> + Send> {
        self.api().get_release(ident, expand, hide, &self.context())
    }

    fn get_release_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetReleaseEditResponse, Error = ApiError> + Send> {
        self.api().get_release_edit(edit_id, &self.context())
    }

    fn get_release_files(
        &self,
        ident: String,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetReleaseFilesResponse, Error = ApiError> + Send> {
        self.api().get_release_files(ident, hide, &self.context())
    }

    fn get_release_filesets(
        &self,
        ident: String,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetReleaseFilesetsResponse, Error = ApiError> + Send> {
        self.api()
            .get_release_filesets(ident, hide, &self.context())
    }

    fn get_release_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetReleaseHistoryResponse, Error = ApiError> + Send> {
        self.api()
            .get_release_history(ident, limit, &self.context())
    }

    fn get_release_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetReleaseRedirectsResponse, Error = ApiError> + Send> {
        self.api().get_release_redirects(ident, &self.context())
    }

    fn get_release_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetReleaseRevisionResponse, Error = ApiError> + Send> {
        self.api()
            .get_release_revision(rev_id, expand, hide, &self.context())
    }

    fn get_release_webcaptures(
        &self,
        ident: String,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetReleaseWebcapturesResponse, Error = ApiError> + Send> {
        self.api()
            .get_release_webcaptures(ident, hide, &self.context())
    }

    fn get_webcapture(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetWebcaptureResponse, Error = ApiError> + Send> {
        self.api()
            .get_webcapture(ident, expand, hide, &self.context())
    }

    fn get_webcapture_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetWebcaptureEditResponse, Error = ApiError> + Send> {
        self.api().get_webcapture_edit(edit_id, &self.context())
    }

    fn get_webcapture_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetWebcaptureHistoryResponse, Error = ApiError> + Send> {
        self.api()
            .get_webcapture_history(ident, limit, &self.context())
    }

    fn get_webcapture_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetWebcaptureRedirectsResponse, Error = ApiError> + Send> {
        self.api().get_webcapture_redirects(ident, &self.context())
    }

    fn get_webcapture_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetWebcaptureRevisionResponse, Error = ApiError> + Send> {
        self.api()
            .get_webcapture_revision(rev_id, expand, hide, &self.context())
    }

    fn get_work(
        &self,
        ident: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetWorkResponse, Error = ApiError> + Send> {
        self.api().get_work(ident, expand, hide, &self.context())
    }

    fn get_work_edit(
        &self,
        edit_id: String,
    ) -> Box<dyn Future<Item = GetWorkEditResponse, Error = ApiError> + Send> {
        self.api().get_work_edit(edit_id, &self.context())
    }

    fn get_work_history(
        &self,
        ident: String,
        limit: Option<i64>,
    ) -> Box<dyn Future<Item = GetWorkHistoryResponse, Error = ApiError> + Send> {
        self.api().get_work_history(ident, limit, &self.context())
    }

    fn get_work_redirects(
        &self,
        ident: String,
    ) -> Box<dyn Future<Item = GetWorkRedirectsResponse, Error = ApiError> + Send> {
        self.api().get_work_redirects(ident, &self.context())
    }

    fn get_work_releases(
        &self,
        ident: String,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetWorkReleasesResponse, Error = ApiError> + Send> {
        self.api().get_work_releases(ident, hide, &self.context())
    }

    fn get_work_revision(
        &self,
        rev_id: String,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = GetWorkRevisionResponse, Error = ApiError> + Send> {
        self.api()
            .get_work_revision(rev_id, expand, hide, &self.context())
    }

    fn lookup_container(
        &self,
        issnl: Option<String>,
        wikidata_qid: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = LookupContainerResponse, Error = ApiError> + Send> {
        self.api()
            .lookup_container(issnl, wikidata_qid, expand, hide, &self.context())
    }

    fn lookup_creator(
        &self,
        orcid: Option<String>,
        wikidata_qid: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = LookupCreatorResponse, Error = ApiError> + Send> {
        self.api()
            .lookup_creator(orcid, wikidata_qid, expand, hide, &self.context())
    }

    fn lookup_file(
        &self,
        md5: Option<String>,
        sha1: Option<String>,
        sha256: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = LookupFileResponse, Error = ApiError> + Send> {
        self.api()
            .lookup_file(md5, sha1, sha256, expand, hide, &self.context())
    }

    fn lookup_release(
        &self,
        doi: Option<String>,
        wikidata_qid: Option<String>,
        isbn13: Option<String>,
        pmid: Option<String>,
        pmcid: Option<String>,
        core: Option<String>,
        arxiv: Option<String>,
        jstor: Option<String>,
        ark: Option<String>,
        mag: Option<String>,
        expand: Option<String>,
        hide: Option<String>,
    ) -> Box<dyn Future<Item = LookupReleaseResponse, Error = ApiError> + Send> {
        self.api().lookup_release(
            doi,
            wikidata_qid,
            isbn13,
            pmid,
            pmcid,
            core,
            arxiv,
            jstor,
            ark,
            mag,
            expand,
            hide,
            &self.context(),
        )
    }

    fn update_container(
        &self,
        editgroup_id: String,
        ident: String,
        container_entity: models::ContainerEntity,
    ) -> Box<dyn Future<Item = UpdateContainerResponse, Error = ApiError> + Send> {
        self.api()
            .update_container(editgroup_id, ident, container_entity, &self.context())
    }

    fn update_creator(
        &self,
        editgroup_id: String,
        ident: String,
        creator_entity: models::CreatorEntity,
    ) -> Box<dyn Future<Item = UpdateCreatorResponse, Error = ApiError> + Send> {
        self.api()
            .update_creator(editgroup_id, ident, creator_entity, &self.context())
    }

    fn update_editgroup(
        &self,
        editgroup_id: String,
        editgroup: models::Editgroup,
        submit: Option<bool>,
    ) -> Box<dyn Future<Item = UpdateEditgroupResponse, Error = ApiError> + Send> {
        self.api()
            .update_editgroup(editgroup_id, editgroup, submit, &self.context())
    }

    fn update_editor(
        &self,
        editor_id: String,
        editor: models::Editor,
    ) -> Box<dyn Future<Item = UpdateEditorResponse, Error = ApiError> + Send> {
        self.api().update_editor(editor_id, editor, &self.context())
    }

    fn update_file(
        &self,
        editgroup_id: String,
        ident: String,
        file_entity: models::FileEntity,
    ) -> Box<dyn Future<Item = UpdateFileResponse, Error = ApiError> + Send> {
        self.api()
            .update_file(editgroup_id, ident, file_entity, &self.context())
    }

    fn update_fileset(
        &self,
        editgroup_id: String,
        ident: String,
        fileset_entity: models::FilesetEntity,
    ) -> Box<dyn Future<Item = UpdateFilesetResponse, Error = ApiError> + Send> {
        self.api()
            .update_fileset(editgroup_id, ident, fileset_entity, &self.context())
    }

    fn update_release(
        &self,
        editgroup_id: String,
        ident: String,
        release_entity: models::ReleaseEntity,
    ) -> Box<dyn Future<Item = UpdateReleaseResponse, Error = ApiError> + Send> {
        self.api()
            .update_release(editgroup_id, ident, release_entity, &self.context())
    }

    fn update_webcapture(
        &self,
        editgroup_id: String,
        ident: String,
        webcapture_entity: models::WebcaptureEntity,
    ) -> Box<dyn Future<Item = UpdateWebcaptureResponse, Error = ApiError> + Send> {
        self.api()
            .update_webcapture(editgroup_id, ident, webcapture_entity, &self.context())
    }

    fn update_work(
        &self,
        editgroup_id: String,
        ident: String,
        work_entity: models::WorkEntity,
    ) -> Box<dyn Future<Item = UpdateWorkResponse, Error = ApiError> + Send> {
        self.api()
            .update_work(editgroup_id, ident, work_entity, &self.context())
    }
}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
