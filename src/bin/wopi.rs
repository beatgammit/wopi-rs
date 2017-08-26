#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

extern crate wopi;

use rocket_contrib::Json;

use wopi::*;
use wopi::models::*;
use wopi::errors::Result;

#[get("/wopi*/files/<id>")]
fn check_file_info(id: String) -> Option<Json<CheckFileInfoResponse>> {
    // TODO: handle header: X-WOPI-SessionContext: The value of the <sc> URI parameter
    // TODO: handle 500 and 401 errors
    // TODO: actually look up file stuff
    println!("TODO: look up id: {}", id);
    if id != "yolo".to_string() {
        return None;
    }
    Some(Json(CheckFileInfoResponse::default()))
}

#[post("/wopi*/files/<id>", rank = 1)]
fn delete_file(id: String, _ign: DeleteFile) -> String {
    // TODO: actually delete file
    // TODO: handle 404 (file unknown/unauthorized)
    // TODO: handle 500
    // TODO: handle 501 (unsupported)
    id
}

#[post("/wopi*/files/<id>", rank = 2)]
fn get_file_lock(id: String, _ign: GetLock) -> String {
    id
}

#[post("/wopi*/files/<id>", rank = 3)]
fn get_restricted_link(id: String, _ign: GetRestrictedLink) -> String {
    id
}

#[post("/wopi*/files/<id>", rank = 4)]
fn get_share_url(id: String, _ign: GetShareUrl) -> String {
    id
}

// this needs to be above Lock
#[post("/wopi*/files/<id>", rank = 5)]
fn unlock_and_relock(id: String, _ign: UnlockAndRelock) -> String {
    id
}

#[post("/wopi*/files/<id>", rank = 6)]
fn get_lock(id: String, _ign: Lock) -> String {
    id
}

#[post("/wopi*/files/<id>", rank = 7)]
fn put_relative_file(id: String, _ign: PutRelativeFile) -> String {
    id
}

#[post("/wopi*/files/<id>", rank = 8)]
fn put_user_info(id: String, _ign: PutUserInfo) -> String {
    id
}

#[post("/wopi*/files/<id>", rank = 9)]
fn read_secure_store(id: String, _ign: ReadSecureStore) -> String {
    id
}

#[post("/wopi*/files/<id>", rank = 10)]
fn refresh_lock(id: String, _ign: RefreshLock) -> String {
    id
}

#[post("/wopi*/files/<id>", rank = 11)]
fn rename_file(id: String, _ign: RenameFile) -> String {
    id
}

#[post("/wopi*/files/<id>", rank = 12)]
fn revoke_restricted_link(id: String, _ign: RevokeRestrictedLink) -> String {
    id
}

#[post("/wopi*/files/<id>", rank = 13)]
fn unlock(id: String, _ign: Unlock) -> String {
    id
}

#[get("/wopi*/folders/<id>")]
fn check_folder_info(id: String) -> Option<Json<CheckFolderInfoResponse>> {
    // TODO: handle header: X-WOPI-SessionContext: The value of the <sc> URI parameter
    // TODO: handle 500 and 401 errors
    // TODO: actually look up file stuff
    println!("TODO: look up id: {}", id);
    if id != "yolo".to_string() {
        return None;
    }
    Some(Json(CheckFolderInfoResponse::default()))
}

#[get("/wopi*/files/<id>/contents")]
fn get_file(id: String) {
    println!("Get file: {}", id);
}

#[post("/wopi*/files/<id>/contents")]
fn put_file(id: String) {
    println!("Put file: {}", id);
}

#[post("/wopi*/folders/<id>/children")]
fn enumerate_children(id: String) {
    println!("Enumerate children: {}", id);
}

// TODO: format = "application/json"?
#[post("/files", data = "<file_info>")]
fn create_file_handler(file_info: Json<CreateFileInfo>) -> Result<Json<File>> {
    println!("create_file: {:?}", file_info);
    create_file(file_info.into_inner()).map(|file| Json(file))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![create_file_handler,])
        .mount(
            "/",
            routes![
                // http://server/<...>/wopi*/files/<id>
                // Provides access to information about a file and allows for
                // file-level operations.

                // CheckFileInfo
                check_file_info,
                // DeleteFile:
                // Removes a file from the WOPI server.
                delete_file,
                // ExecuteCellStorageRelativeRequest:
                // Changes the contents of the file in accordance with [MS-FSSHTTP].

                // ExecuteCellStorageRequest:
                // Changes the contents of the file in accordance with [MS-FSSHTTP].

                // GetLock:
                // Retrieves a lock for editing a file.
                get_file_lock,
                // GetRestrictedLink:
                // Gets a link to a file through which a user is able to operate on a
                // file in a limited way.
                get_restricted_link,
                // GetShareUrl:
                // Gets a URI to the file that is suitable for sharing with other users.
                get_share_url,
                // Lock:
                // Takes a lock for editing a file.
                get_lock,
                // PutRelativeFile:
                // Creates a copy of a file on the WOPI server.
                put_relative_file,
                // PutUserInfo:
                // Stores user information on the WOPI server.
                put_user_info,
                // ReadSecureStore:
                // Accesses the WOPI server's implementation of a secure store.
                read_secure_store,
                // RefreshLock:
                // Refreshes a lock for editing a file.
                refresh_lock,
                // RenameFile:
                // Renames a file.
                rename_file,
                // RevokeRestrictedLink:
                // Revokes all links to a file through which a number of users are
                // able to operate on a file in a limited way.
                revoke_restricted_link,
                // Unlock:
                // Releases a lock for editing a file.
                unlock,
                // UnlockAndRelock:
                // Releases and then retakes a lock for editing a file.
                unlock_and_relock,
                // http://server/<...>/wopi*/folders/<id>
                // Return information about the folder and permissions that the
                // current user has relative to that file.
                check_folder_info,
                // http://server/<...>/wopi*/files/<id>/contents

                // GetFile:
                // Request message to retrieve a file for the
                // HTTP://server/<...>/wopi*/files/<id>/contents operation.
                get_file,
                // PutFile:
                // Request message to update a file for
                // the http://server/<...>/wopi*/files/<id>/contents
                // operation.
                put_file,
                // http://server/<...>/wopi*/folders/<id>/children

                // EnumerateChildren:
                // Returns a set of URIs that provides access to resources in the folder
                enumerate_children,
            ],
        )
}

fn main() {
    match parse_discovery("https://localhost:9980/hosting/discovery") {
        Ok(discovery) => println!("res: {:?}", discovery),
        Err(err) => println!("error fetching discovery: {}", err),
    };

    rocket().launch();
}
