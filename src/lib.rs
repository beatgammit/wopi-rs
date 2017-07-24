extern crate rocket;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate reqwest;

use std::io::Read;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

const X_WOPI_OVERRIDE: &str = "X-WOPI-Override";
const X_WOPI_SIZE: &str = "X-WOPI-Size";
const X_WOPI_RELATIVE_TARGET: &str = "X-WOPI-RelativeTarget";
const X_WOPI_LOCK: &str = "X-WOPI-Lock";
const X_WOPI_OLD_LOCK: &str = "X-WOPI-OldLock";

#[derive(Serialize)]
pub struct CheckFileInfoResponse {
    // A Boolean value that indicates the WOPI client allows connections to
    // external Microsoft services to provide additional functionality.
    //
    // If this value is false, then the WOPI client MUST NOT allow such connections.
    #[serde(rename = "AllowAdditionalMicrosoftServices")]
    allow_additional_microsoft_services: Option<bool>,

    // A Boolean value that indicates the WOPI client allows connections to
    // external services referenced in the file (for example, a marketplace of
    // embeddable JavaScript apps).
    //
    // If this value is f alse, then the WOPI client MUST NOT allow such connections.
    #[serde(rename = "AllowExternalMarketplace")]
    allow_external_marketplace: Option<bool>,

    // The name of the file without the path. Used for display in user
    // interface (UI), and determining the extension of the file.
    #[serde(rename = "BaseFileName")]
    base_file_name: String,

    // A string that the WOPI client displays to the user that indicates the
    // brand name of the WOPI server.
    #[serde(rename = "BreadcrumbBrandName")]
    breadcrumb_brand_name: Option<String>,

    // A URI to a web page that the WOPI client navigates to when the user
    // clicks on UI that displays BreadcrumbBrandName.
    #[serde(rename = "BreadcrumbBrandUrl")]
    breadcrumb_brand_url: String,

    // A string that the WOPI client displays to the user that indicates the
    // name of the file.
    #[serde(rename = "BreadcrumbDocName")]
    breadcrumb_doc_name: Option<String>,

    // A URI to a web page that the WOPI client navigates to when the user
    // clicks on UI that displays BreadcrumbDocName.
    #[serde(rename = "BreadcrumbDocUrl")]
    breadcrumb_doc_url: Option<String>,

    // A string that the WOPI client displays to the user that indicates the
    // name of the folder that contains the file.
    #[serde(rename = "BreadcrumbFolderName")]
    breadcrumb_folder_name: Option<String>,

    // A URI to a web page that the WOPI client navigates to when the user
    // clicks on UI that displays BreadcrumbFolderName.
    #[serde(rename = "BreadcrumbFolderUrl")]
    breadcrumb_folder_url: Option<String>,

    // A user - accessible URI directly to the file int ended for opening the
    // file through a client.
    //
    // Can be a DAV URL ([RFC5323]), but MAY be any URL that can be handled by
    // a client that can open a file of the given type.
    #[serde(rename = "ClientUrl")]
    client_url: Option<String>,

    // A Boolean value that indicates that the WOPI client SHOULD close the
    // browser window containing the output of the WOPI client when the user
    // calls the close UI.
    #[serde(rename = "CloseButtonClosesWindow")]
    close_button_closes_window: Option<bool>,

    // A Boolean value that indicates that the WOPI client SHOULD notify the
    // WOPI server in the event that the user closes the rendering or editing
    // client currently using this file.
    #[serde(rename = "ClosePostMessage")]
    close_post_message: bool,

    // A URI to a web page that the implementer deems useful to a user in the
    // event that the user closes the rendering or editing client currently using this file.
    #[serde(rename = "CloseUrl")]
    close_url: Option<String>,

    // A Boolean value that indicates that the WOPI client MUST disable caching
    // of file contents in the browser cache.
    #[serde(rename = "DisableBrowserCachingOfUserContent")]
    disable_browser_caching_of_user_content: Option<bool>,

    // A Boolean value t hat indicates that the WOPI client MUST disable any
    // print functionality under its control.
    #[serde(rename = "DisablePrint")]
    disable_print: Option<bool>,

    // A Boolean value that indicates that the WOPI client MUST NOT permit the
    // use of machine translation functionality that is exposed by the WOPI
    // client.
    #[serde(rename = "DisableTranslation")]
    disable_translation: bool,

    // A user - accessible URI to the file intended to allow the user to
    // download a copy of the file.
    #[serde(rename = "DownloadUrl")]
    download_url: Option<String>,

    // A URI to a web page that provides an editing experience for the file,
    // utilizing the WOPI client.
    #[serde(rename = "EditAndReplyUrl")]
    edit_and_reply_url: Option<String>,

    // A Boolean v alue that indicates that the WOPI client SHOULD notify the
    // WOPI server in the event that the user attempts to edit the file.
    #[serde(rename = "EditModePostMessage")]
    edit_mode_post_message: Option<bool>,

    // A Boolean value that indicates that the WOPI client SHOULD notify the
    // WOPI server in the event that th e user attempts to edit the file.
    #[serde(rename = "EditNotificationPostMessage")]
    edit_notification_post_message: bool,

    // A string specifying the file extension of the file.
    // This value MUST begin with a ".".
    #[serde(rename = "FileExtension")]
    file_extension: Option<String>,

    // An integer indicating the maximum length for file names, including the
    // file extension, supported by the WOPI server.
    #[serde(rename = "FileNameMaxLength")]
    file_name_max_length: Option<i64>,

    // A Boolean va lue that indicates that the WOPI client SHOULD notify the
    // WOPI server in the event that the user attempts to share the file.
    #[serde(rename = "FileSharingPostMessage")]
    file_sharing_post_message: bool,

    // A URI to a location that allows the user to share the file.
    #[serde(rename = "FileSharingUrl")]
    file_sharing_url: Option<String>,

    // A URI to the file location that the WOPI client uses to get the file.
    //
    // If this is provided, a WOPI client MUST use this URI to get the file
    // instead of HTTP://server/<...>/wopi*/files/<id>/contents (see section 3.3.5.3).
    #[serde(rename = "FileUrl")]
    file_url: Option<String>,

    // A string that is used by the WOPI server to uniquely identify the user.
    #[serde(rename = "HostAuthenticationId")]
    host_authentication_id: Option<String>,

    // A URI to a web page that provides an editing experience for the file,
    // utilizing the WOPI client.
    #[serde(rename = "HostEditUrl")]
    host_edit_url: String,

    // A URI to a web page that provides access to an editing experience for the
    // file that can be embedded in another HTML page.
    //
    // For example, a page that provides an HTML snippet that can be inserted
    // into the HTML of a blog.
    #[serde(rename = "HostEmbeddedEditUrl")]
    host_embedded_edit_url: Option<String>,

    // A URI to a web page that provides access to a viewing experience for the
    // file that can be embedded in another HTML page.
    //
    // For example, a page that p rovides an HTML snippet that can be inserted
    // into the HTML of a blog
    #[serde(rename = "HostEmbeddedViewUrl")]
    host_embedded_view_url: Option<String>,

    // A string that is the name provided by the WOPI server used to identify it
    // for logging and other informational purposes.
    #[serde(rename = "HostName")]
    host_name: Option<String>,

    // A string that is used by the WOPI server to pass arbitrary information to
    // the WOPI client. The WOPI client MAY ignore this string if it does not
    // recognize the contents.
    //
    // A WOPI server MUST NOT require that a WOPI client understand the contents
    // of this string to operate.
    #[serde(rename = "HostNotes")]
    host_notes: Option<String>,

    // A URI that is the base URI for REST operations for the file.
    #[serde(rename = "HostRestUrl")]
    host_rest_url: Option<String>,

    // A URI to a web page that provides a viewing experience for the file
    // utilizing the WOPI client.
    #[serde(rename = "HostViewUrl")]
    host_view_url: Option<String>,

    // A string that the WOPI client SHOULD display to the user indicating the
    // Information Rights Management (IRM) policy for the file.
    //
    // This value SHOULD be combined with IrmPolicyTitle.
    #[serde(rename = "IrmPolicyDescription")]
    irm_policy_description: Option<String>,

    // A string that the WOPI client SHOULD display to the user indicating the
    // IRM policy for the file.
    //
    // This value SHOULD be combined with IrmPolicyDescription.
    #[serde(rename = "IrmPolicyTitle")]
    irm_policy_title: String,

    // A Boolean value that indicates that the WOPI client SHOULD take measures
    // to ensure the user is properly licensed prior to allowing editing of the
    // file.
    #[serde(rename = "LicenseCheckForEditIsEnabled")]
    license_check_for_edit_is_enabled: Option<bool>,

    // A string that SHOULD uniquely identify the owner of the file.
    #[serde(rename = "OwnerId")]
    owner_id: String,

    // A string that the WOPI client MUST use as the targetOrigin parameter when
    // sending messages as described in [W3C - HTML5WEBMSG].
    #[serde(rename = "PostMessageOrigin")]
    post_message_origin: Option<String>,

    // A string that identifies the provider of information that a WOPI client
    // uses to discover information about the user ’s online status (for
    // example, whether a user is available via instant messenger).
    //
    // A WOPI client requires knowledge of specific presence providers to be
    // able to take advantage of this value.
    #[serde(rename = "PresenceProvider")]
    presence_provider: Option<String>,

    // A string that identifies the user in the context of the PresenceProvider.
    #[serde(rename = "PresenceUserId")]
    presence_user_id: Option<String>,

    // A URI to a webpage that explains the privacy policy of the WOPI server.
    #[serde(rename = "PrivacyUrl")]
    privacy_url: Option<String>,

    // A Boolean value that indicates that the WOPI client SHOULD take measures
    // to prevent copying and printing of the file.
    //
    // This is intended to help enforce IRM in WOPI clients.
    #[serde(rename = "ProtectInClient")]
    protect_in_client: Option<bool>,

    // Indicates that, for this user, the file cannot be changed.
    #[serde(rename = "ReadOnly")]
    read_only: Option<bool>,

    // A Boolean value that indicates that the WOPI client MUST NOT allow the
    // user to download the file or open the file in a separate application.
    #[serde(rename = "RestrictedWebViewOnly")]
    restricted_web_view_only: Option<bool>,

    // If it is present and not empty, it is a 256 bit SHA-2 encoded [FIPS1802]
    // hash of the file contents.
    #[serde(rename = "SHA256")]
    sha256: Option<String>,

    // A URI that will sign the current user into the WOPI server supported
    // authentication system.
    #[serde(rename = "SignInUrl")]
    sign_in_url: Option<String>,

    // A URI that will sign the current user out of the WOPI server supported
    // authentication system.
    #[serde(rename = "SignoutUrl")]
    signout_url: String,

    // The size of the file expressed in bytes.
    #[serde(rename = "Size")]
    size: i64,

    // An array of strings indicating the share URL types supported by the WOPI
    // server.
    #[serde(rename = "SupportedShareUrlTypes")]
    supported_share_url_types: Option<Vec<String>>,

    // A Boolean value that indicates that the WOPI server supports multiple
    // users making changes to this file simultaneously.
    #[serde(rename = "SupportsCoauth")]
    supports_coauth: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports
    // ExecuteCellStorageRequest (see section 3.3.5.1.4) and
    // ExcecuteCellStorageRelativeRequest (see section 3.3.5.1.3) operations for
    // this file.
    #[serde(rename = "SupportsCobalt")]
    supports_cobalt: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports lock IDs up
    // to 1024 ASCII characters in length.
    #[serde(rename = "SupportsExtendedLockLength")]
    supports_extended_lock_length: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports creating new
    // files using the WOPI client.
    #[serde(rename = "SupportsFileCreation")]
    supports_file_creation: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports
    // EnumerateChildren (see section 3.3.5.4.1) and DeleteFile (see section
    // 3.3.5.1.2) operations for this file.
    #[serde(rename = "SupportsFolders")]
    supports_folders: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports GetLock (see
    // section 3.3.5.1.5).
    #[serde(rename = "SupportsGetLock")]
    supports_get_lock: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports Lock (see
    // section 3.3.5.1.8), Unlock (see section 3.3.5.1.15), RefreshLock (see
    // section 3.3.5.1.12), and UnlockAndRelock (see section 3.3.5.1.16)
    // operations for this file.
    #[serde(rename = "SupportsLocks")]
    supports_locks: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports RenameFile
    // (see section 3.3.5.1.13).
    #[serde(rename = "SupportsRename")]
    supports_rename: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports scenarios
    // where users can operate on files in limited ways via restricted URLs.
    #[serde(rename = "SupportsScenarioLinks")]
    supports_scenario_links: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports calls to a
    // secure data store utilizing credential s stored in the file.
    #[serde(rename = "SupportsSecureStore")]
    supports_secure_store: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports PutFile (see
    // section 3.3.5.3.2) and PutRelativeFile (see section 3.3.5.1.9) operations
    // for this file.
    #[serde(rename = "SupportsUpdate")]
    supports_update: Option<bool>,

    // A Boolean value that indicates that the WOPI server supports PutUserInfo
    // (see section 3.3.5.1.10).
    #[serde(rename = "SupportsUserInfo")]
    supports_user_info: Option<bool>,

    // A string that is used by the WOPI server to uniquely identify the tenant.
    #[serde(rename = "TenantId")]
    tenant_id: Option<String>,

    // A URI to a webpage that explains the terms of use policy of the WOPI server.
    #[serde(rename = "TermsOfUseUrl")]
    terms_of_use_url: Option<String>,

    // A string that is used to pass time zone information to a WOPI client in
    // the format chosen by the WOPI server.
    #[serde(rename = "TimeZone")]
    time_zone: Option<String>,

    // A string that uniquely represents the file contents.
    //
    // This value MUST change when the file content changes.
    // This value MUST also be equivalent for all files with equivalent contents.
    #[serde(rename = "UniqueContentId")]
    unique_content_id: Option<String>,

    // A Boolean value that indicates that the user has permission to view a
    // broadcast of this file. A broadcast is file activity that involves one or
    // more presenters controlling the view of the file for a set of attendees.
    //
    // For example, a slideshow can be broadcast by a presenter to many
    // attendees.
    #[serde(rename = "UserCanAttend")]
    user_can_attend: Option<bool>,

    // A Boolean value that indicates the user does not have sufficient
    // permissions to create new files on the WOPI server.
    #[serde(rename = "UserCanNotWriteRelative")]
    user_can_not_write_relative: Option<bool>,

    // A Boolean value that indicates that the user has permission to broadcast
    // this file to a set of users who have permission to broadcast or view a
    // broadcast of this file. A broadcast is file activity that involves one or
    // more presenters controlling the view of the file for a set of attendees.
    //
    // For example, a slideshow can be broadcast by a presenter to many
    // attendees.
    #[serde(rename = "UserCanPresent")]
    user_can_present: Option<bool>,

    // A Boolean value that indicates the user has permissions to rename the file.
    #[serde(rename = "UserCanRename")]
    user_can_rename: Option<bool>,

    // A Boolean value that indicates that the user has permissions to alter the file.
    #[serde(rename = "UserCanWrite")]
    user_can_write: Option<bool>,

    // A string that is the name of the user.
    //
    // If blank, the WOPI client MAY be configured to use a placeholder string
    // in some scenarios, or to show no name at all.
    #[serde(rename = "UserFriendlyName")]
    user_friendly_name: Option<String>,

    // A string that is used by the WOPI server to uniquely identify the user.
    #[serde(rename = "UserId")]
    user_id: Option<String>,

    // A string that the WOPI client SHOULD use to verify the user’s licensing status.
    // The WOPI client MAY ignore this string if it does not recognize the contents.
    #[serde(rename = "UserInfo")]
    user_info: Option<String>,

    // A string representing the current version of the file based on the WOPI
    // server’s file versioning schema.
    //
    // This value MUST change when the file content changes, and version values
    // MUST never repeat for a given file.
    #[serde(rename = "Version")]
    version: String,

    // A Boolean value that indicates that the WOPI client MUST NOT allow the
    // user to use the WOPI client’s editing functionality to operate on the
    // file. This does not mean that the user doesn’t have rights to edit the
    // file.
    #[serde(rename = "WebEditingDisabled")]
    web_editing_disabled: Option<bool>,

    // An array of strings representing the workflow types that are available for the file.
    #[serde(rename = "WorkflowType")]
    workflow_type: Option<Vec<String>>,

    // A string representing the current ver sion of the file based on the WOPI
    // server’s file versioning schema.
    //
    // This value MUST change when the file content changes, and version values
    // MUST never repeat for a given file.
    #[serde(rename = "WorkflowUrl")]
    workflow_url: Option<String>,
}

impl Default for CheckFileInfoResponse {
    fn default() -> Self {
        CheckFileInfoResponse {
            allow_additional_microsoft_services: Some(false),
            allow_external_marketplace: Some(false),
            base_file_name: "".to_string(),
            breadcrumb_brand_name: Some("".to_string()),
            breadcrumb_brand_url: "".to_string(),
            breadcrumb_doc_name: Some("".to_string()),
            breadcrumb_doc_url: Some("".to_string()),
            breadcrumb_folder_name: Some("".to_string()),
            breadcrumb_folder_url: Some("".to_string()),
            client_url: Some("".to_string()),
            close_button_closes_window: Some(false),
            close_post_message: false,
            close_url: Some("".to_string()),
            disable_browser_caching_of_user_content: Some(false),
            disable_print: Some(false),
            disable_translation: false,
            download_url: Some("".to_string()),
            edit_and_reply_url: Some("".to_string()),
            edit_mode_post_message: Some(false),
            edit_notification_post_message: false,
            file_extension: Some("".to_string()),
            file_name_max_length: Some(250),
            file_sharing_post_message: false,
            file_sharing_url: Some("".to_string()),
            file_url: Some("".to_string()),
            host_authentication_id: Some("".to_string()),
            host_edit_url: "".to_string(),
            host_embedded_edit_url: Some("".to_string()),
            host_embedded_view_url: Some("".to_string()),
            host_name: Some("".to_string()),
            host_notes: Some("".to_string()),
            host_rest_url: Some("".to_string()),
            host_view_url: Some("".to_string()),
            irm_policy_description: Some("".to_string()),
            irm_policy_title: "".to_string(),
            license_check_for_edit_is_enabled: Some(false),
            owner_id: "".to_string(),
            post_message_origin: Some("".to_string()),
            presence_provider: Some("".to_string()),
            presence_user_id: Some("".to_string()),
            privacy_url: Some("".to_string()),
            protect_in_client: Some(false),
            read_only: Some(false),
            restricted_web_view_only: Some(false),
            sha256: None,
            sign_in_url: Some("".to_string()),
            signout_url: "".to_string(),
            size: 0,
            supported_share_url_types: Some(vec![]),
            supports_coauth: Some(false),
            supports_cobalt: Some(false),
            supports_extended_lock_length: Some(false),
            supports_file_creation: Some(false),
            supports_folders: Some(false),
            supports_get_lock: Some(false),
            supports_locks: Some(false),
            supports_rename: Some(false),
            supports_scenario_links: Some(false),
            supports_secure_store: Some(false),
            supports_update: Some(false),
            supports_user_info: Some(false),
            tenant_id: Some("".to_string()),
            terms_of_use_url: Some("".to_string()),
            time_zone: Some("".to_string()),
            unique_content_id: Some("".to_string()),
            user_can_attend: Some(false),
            user_can_not_write_relative: Some(false),
            user_can_present: Some(false),
            user_can_rename: Some(false),
            user_can_write: Some(false),
            user_friendly_name: Some("".to_string()),
            user_id: Some("".to_string()),
            user_info: Some("".to_string()),
            version: "".to_string(),
            web_editing_disabled: Some(false),
            workflow_type: Some(vec![]),
            workflow_url: Some("".to_string()),
        }
    }
}

#[derive(Serialize)]
pub struct CheckFolderInfoResponse {
    // The name of the folder without the path. Used for display in the UI.
    #[serde(rename = "FolderName")]
    folder_name: String,

    // A URI to an image that the WOPI client displays to the user as the
    // branding image of the WOPI server.
    #[serde(rename = "BreadcrumbBrandIconUrl")]
    breadcrumb_brand_icon_url: Option<String>,

    // A string that the WOPI client displays to the user that indicates the
    // brand name of the WOPI server.
    #[serde(rename = "BreadcrumbBrandName")]
    breadcrumb_brand_name: Option<String>,

    //  A URI to a web page that the WOPI client navigates to when the user
    //  clicks on the UI that displays BreadcrumbBrandName.
    #[serde(rename = "BreadcrumbBrandUrl")]
    breadcrumb_brand_url: Option<String>,

    // A string that the WOPI client displays to the user that indicates the
    // name of the file.
    #[serde(rename = "BreadcrumbDocName")]
    breadcrumb_doc_name: Option<String>,

    // A URI to a web page that the WOPI client navigates to when the user
    // clicks on the UI that displays BreadcrumbDocName.
    #[serde(rename = "BreadcrumbDocUrl")]
    breadcrumb_doc_url: Option<String>,

    // A string that the WOPI client displays to the user that indicates the
    // name of the folder that contains the file.
    #[serde(rename = "BreadcrumbFolderName")]
    breadcrumb_folder_name: Option<String>,

    // A URI to a web page that the WOPI client navigates to when the user
    // clicks on the UI that displays BreadcrumbFolderName.
    #[serde(rename = "BreadcrumbFolderUrl")]
    breadcrumb_folder_url: Option<String>,

    // A user - accessible URI directly to the folder intended for opening the
    // file through a client.
    //
    // Can be a DAV URL ( [RFC5323]), but MAY be any URL that can be handled by
    // a client that can open a file of the given type.
    #[serde(rename = "ClientUrl")]
    client_url: Option<String>,

    // A Boolean value that indicates that the WOPI client SHOULD close the
    // browser window containing the output of the WOPI client when the user
    // calls the close UI.
    #[serde(rename = "CloseButtonClosesWindow")]
    close_button_closes_window: Option<bool>,

    // A URI to a web page that the implementer deems useful to a user in the
    // event that the user closes the rendering or editing client currently
    // using this folder.
    #[serde(rename = "CloseUrl")]
    close_url: Option<String>,

    // A URI to a location that allows the user to share the file.
    #[serde(rename = "FileSharingUrl")]
    file_sharing_url: Option<String>,

    // A string that is used by the WOPI server to uniquely identify the user.
    #[serde(rename = "HostAuthenticationId")]
    host_authentication_id: Option<String>,

    // A URI to a web page that provides an editing experience for the folder
    // utilizing the WOPI client.
    #[serde(rename = "HostEditUrl")]
    host_edit_url: Option<String>,

    // A URI to a web page that provides access to an editing experience for the
    // folder that can be embedded in another HTML page.
    //
    // For example, a page that provides an HTML snippet that can be inserted
    // into the HTML of a blog.
    #[serde(rename = "HostEmbeddedEditUrl")]
    host_embedded_edit_url: Option<String>,

    // A URI to a web page that provides access to a viewing experience for the
    // folder that can be embedded in another HTML page.
    //
    // For example, a page that provides an HTML snippet that can be inserted
    // into the HTML of a blog.
    #[serde(rename = "HostEmbeddedViewUrl")]
    host_embedded_view_url: Option<String>,

    // A string that is the name provided by the WOPI server used to identify it
    // for logging and other informational purposes.
    #[serde(rename = "HostName")]
    host_name: Option<String>,

    // A URI to a web page that provides a viewing experience for the folder
    // utilizing the WOPI client.
    #[serde(rename = "HostViewUrl")]
    host_view_url: Option<String>,

    // A string that SHOULD uniquely identify the owner of the file.
    #[serde(rename = "OwnerId")]
    owner_id: String,

    // A string that identifies the provider of information that a WOPI client
    // uses to discover information about the user’s online status (for example,
    // whether a user is available via instant messenger).
    //
    // A WOPI client requires knowledge of specific presence providers to be
    // able to take advantage of this value.
    #[serde(rename = "PresenceProvider")]
    presence_provider: Option<String>,

    // A string that identifies the user in the context of the PresenceProvider.
    #[serde(rename = "PresenceUserId")]
    presence_user_id: Option<String>,

    // A URI to a webpage that explains the privacy policy of the WOPI server.
    #[serde(rename = "PrivacyUrl")]
    privacy_url: Option<String>,

    // A URI that will sign the current user out of the WOPI server supported
    // authentication system.
    #[serde(rename = "SignoutUrl")]
    signout_url: Option<String>,

    // A Boolean value that indicates that the WOPI server supports calls to a
    // secure data store utilizing credentials stored in the file.
    #[serde(rename = "SupportsSecureStore")]
    supports_secure_store: Option<bool>,

    // A string that is used by the WOPI server to uniquely identify the tenant.
    #[serde(rename = "TenantId")]
    tenant_id: Option<String>,

    // A URI to a webpage that explains the terms of use policy of the WOPI server.
    #[serde(rename = "TermsOfUseUrl")]
    terms_of_use_url: Option<String>,

    // Indicates that the user has permissions to alter the folder.
    #[serde(rename = "UserCanWrite")]
    user_can_write: Option<bool>,

    // A string that is the name of the user.
    //
    // If blank, the WOPI client MAY be configured to use a placeholder string
    // in some scenarios, or to show no name at all.
    #[serde(rename = "UserFriendlyName")]
    user_friendly_name: Option<String>,

    // A string that is used by the WOPI server to uniquely identify the user.
    #[serde(rename = "UserId")]
    user_id: Option<String>,

    // A Boolean value that indicates that the WOPI client MUST NOT allow the
    // user to use the WOPI client’s editing functionality to operate on the
    // file. This does not mean that the user doesn’t have rights to edit the
    // file.
    #[serde(rename = "WebEditingDisabled")]
    web_editing_disabled: Option<bool>,
}

impl Default for CheckFolderInfoResponse {
    fn default() -> Self {
        CheckFolderInfoResponse {
            folder_name: "".to_string(),
            breadcrumb_brand_icon_url: Some("".to_string()),
            breadcrumb_brand_name: Some("".to_string()),
            breadcrumb_brand_url: Some("".to_string()),
            breadcrumb_doc_name: Some("".to_string()),
            breadcrumb_doc_url: Some("".to_string()),
            breadcrumb_folder_name: Some("".to_string()),
            breadcrumb_folder_url: Some("".to_string()),
            client_url: Some("".to_string()),
            close_button_closes_window: Some(false),
            close_url: Some("".to_string()),
            file_sharing_url: Some("".to_string()),
            host_authentication_id: Some("".to_string()),
            host_edit_url: Some("".to_string()),
            host_embedded_edit_url: Some("".to_string()),
            host_embedded_view_url: Some("".to_string()),
            host_name: Some("".to_string()),
            host_view_url: Some("".to_string()),
            owner_id: "".to_string(),
            presence_provider: Some("".to_string()),
            presence_user_id: Some("".to_string()),
            privacy_url: Some("".to_string()),
            signout_url: Some("".to_string()),
            supports_secure_store: Some(false),
            tenant_id: Some("".to_string()),
            terms_of_use_url: Some("".to_string()),
            user_can_write: Some(false),
            user_friendly_name: Some("".to_string()),
            user_id: Some("".to_string()),
            web_editing_disabled: Some(false),
        }
    }
}

fn header_is(request: &Request, header: &str, exp: &str) -> bool {
    if let Some(val) = request.headers().get_one(header) {
        if val == exp {
            return true;
        }
    }
    false
}

pub struct DeleteFile;
impl<'a, 'r> FromRequest<'a, 'r> for DeleteFile {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "DELETE") {
            Outcome::Success(DeleteFile {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct GetLock;
impl<'a, 'r> FromRequest<'a, 'r> for GetLock {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        println!(
            "Get lock handler: {:?}",
            request.headers().get_one(X_WOPI_OVERRIDE)
        );
        if header_is(request, X_WOPI_OVERRIDE, "GET_LOCK") {
            Outcome::Success(GetLock {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct GetRestrictedLink;
impl<'a, 'r> FromRequest<'a, 'r> for GetRestrictedLink {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "GET_RESTRICTED_LINK") {
            Outcome::Success(GetRestrictedLink {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct GetShareUrl;
impl<'a, 'r> FromRequest<'a, 'r> for GetShareUrl {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "GET_SHARE_URL") {
            Outcome::Success(GetShareUrl {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct Lock;
impl<'a, 'r> FromRequest<'a, 'r> for Lock {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "LOCK") {
            Outcome::Success(Lock {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct PutRelativeFile;
impl<'a, 'r> FromRequest<'a, 'r> for PutRelativeFile {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "PUT_RELATIVE") {
            Outcome::Success(PutRelativeFile {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct PutUserInfo;
impl<'a, 'r> FromRequest<'a, 'r> for PutUserInfo {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "PUT_USER_INFO") {
            Outcome::Success(PutUserInfo {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct ReadSecureStore;
impl<'a, 'r> FromRequest<'a, 'r> for ReadSecureStore {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "READ_SECURE_STORE") {
            Outcome::Success(ReadSecureStore {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct RefreshLock;
impl<'a, 'r> FromRequest<'a, 'r> for RefreshLock {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "REFRESH_LOCK") {
            Outcome::Success(RefreshLock {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct RenameFile;
impl<'a, 'r> FromRequest<'a, 'r> for RenameFile {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "RENAME_FILE") {
            Outcome::Success(RenameFile {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct RevokeRestrictedLink;
impl<'a, 'r> FromRequest<'a, 'r> for RevokeRestrictedLink {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "REVOKE_RESTRICTED_LINK") {
            Outcome::Success(RevokeRestrictedLink {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct Unlock;
impl<'a, 'r> FromRequest<'a, 'r> for Unlock {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if header_is(request, X_WOPI_OVERRIDE, "UNLOCK") {
            Outcome::Success(Unlock {})
        } else {
            Outcome::Forward(())
        }
    }
}

pub struct UnlockAndRelock;
impl<'a, 'r> FromRequest<'a, 'r> for UnlockAndRelock {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        println!("UnlockAndRelock: {:?}", request.headers());
        if request.headers().contains(X_WOPI_OLD_LOCK) {
            if header_is(request, X_WOPI_OVERRIDE, "LOCK") {
                Outcome::Success(UnlockAndRelock {})
            } else {
                Outcome::Failure((Status::BadRequest, ()))
            }
        } else {
            Outcome::Forward(())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryNetZone {
    name: String,
    #[serde(rename = "app")]
    apps: Vec<DiscoveryApp>,
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryApp {
    name: String,
    action: DiscoveryAction,
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryAction {
    ext: String,
    name: String,
    urlsrc: String,
}

#[derive(Debug, Deserialize)]
pub struct Discovery {
    #[serde(rename = "net-zone")]
    net_zone: DiscoveryNetZone,
}

pub fn get_certs() -> std::io::Result<Vec<Vec<u8>>> {
    std::fs::read_dir("certs")?
        .map(|entry| {
            let mut buf = Vec::new();
            std::fs::File::open(entry?.path())?.read_to_end(&mut buf)?;
            Ok(buf)
        })
        .collect()
}

pub fn parse_discovery(uri: &str) -> reqwest::Result<Discovery> {
    let mut res = {
        let mut builder = reqwest::Client::builder()?;
        // TODO: handle errors
        if let Ok(certs) = get_certs() {
            for cert in certs {
                let cert = reqwest::Certificate::from_der(cert.as_slice())?;
                builder.add_root_certificate(cert)?;
            }
        }

        builder.build()?.get(uri)?.send()?
    };

    Ok(serde_xml_rs::deserialize(res).unwrap())
}
