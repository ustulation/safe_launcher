// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

// TODO(Spanda) needs a get timeout - Modify Rfc

#[derive(RustcDecodable, Debug)]
pub struct GetFile {
    offset          : i64,
    length          : i64,
    file_path       : String,
    is_path_shared  : bool,
    include_metadata: bool,
}

impl ::launcher::parser::traits::Action for GetFile {
    fn execute(&mut self, params: ::launcher::parser::ParameterPacket) -> ::launcher::parser::ResponseType {
        if self.is_path_shared && !*eval_result!(params.safe_drive_access.lock()) {
            return Err(::errors::LauncherError::PermissionDenied)
        }

        let mut tokens = ::launcher::parser::helper::tokenise_path(&self.file_path, false);
        let file_to_get = try!(tokens.pop().ok_or(::errors::LauncherError::InvalidPath));

        let start_dir_key = if self.is_path_shared {
            &params.safe_drive_dir_key
        } else {
            &params.app_root_dir_key
        };

        let parent_sub_dir = try!(::launcher::parser::helper::get_final_subdirectory(params.client.clone(),
                                                                                     &tokens,
                                                                                     Some(start_dir_key)));

        unimplemented!();

        Ok(None)
    }
}

#[derive(Debug)]
struct GetFileResponse {
    content : String,
    metadata: Option<Metadata>,
}

impl ::rustc_serialize::json::ToJson for GetFileResponse {
    fn to_json(&self) -> ::rustc_serialize::json::Json {
        let mut response_tree = ::std::collections::BTreeMap::new();
        let _ = response_tree.insert("content".to_string(), self.content.to_json());
        if let Some(ref metadata) = self.metadata {
            let json_metadata_str = eval_result!(::rustc_serialize::json::encode(metadata));
            let _ = response_tree.insert("metadata".to_string(), json_metadata_str.to_json());
        }

        ::rustc_serialize::json::Json::Object(response_tree)
    }
}

#[derive(RustcEncodable, Debug)]
struct Metadata {
    name                  : String,
    size                  : i64,
    user_metadata         : String,
    creation_time_sec     : i64,
    creation_time_nsec    : i64,
    modification_time_sec : i64,
    modification_time_nsec: i64,
}

