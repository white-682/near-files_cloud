use near_sdk::{AccountId, env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use crate::file::{File, ShareFileInfo};
mod file;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: String,
    /// Used to store the creation time of Account
    pub created: UnorderedMap<String, u64>,
    /// Used to store the update time of Account
    pub updated: UnorderedMap<String, u64>,

    ///This is a record of the information stored in the file. key:Account; value:vec<String>
    pub files_store: UnorderedMap<String, Vec<String>>,
    ///This is the folder that records the user's ownership. key:Account; value:vec<folder_name>
    pub folder_store: UnorderedMap<String, Vec<String>>,
    ///This is all the files contained in the records folder. key:Account+folder_name; value:vec<File>
    pub file_index: UnorderedMap<String, File>,

    ///This is the information that records file sharing. key:Account; value:vec<ShareFileInfo>
    pub file_share: UnorderedMap<String, Vec<ShareFileInfo>>,
    ///This is the total size of the record file share. key:Account; value:f64
    pub file_share_size: UnorderedMap<String, f64>,
    ///This is the information used to index file sharing. key:Account; value:ShareFileInfo
    pub file_share_index: UnorderedMap<String, ShareFileInfo>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner_id: String::new(),
            created: UnorderedMap::new(b"j".to_vec()),
            updated: UnorderedMap::new(b"h".to_vec()),

            files_store: UnorderedMap::new(b"f".to_vec()),
            folder_store: UnorderedMap::new(b"e".to_vec()),
            file_index: UnorderedMap::new(b"i".to_vec()),
            file_share: UnorderedMap::new(b"s".to_vec()),
            file_share_size: UnorderedMap::new(b"z".to_vec()),
            file_share_index: UnorderedMap::new(b"x".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    ///set contract owner
    pub fn set_owner(&mut self, owner_id: String) -> bool {
        assert!(!env::state_exists(), "Already set_owner");
        let _ = self.owner_id = owner_id;
        true
    }
}

pub fn gen_account(account_id: AccountId) -> String {
    String::from("files_cloud:") + account_id.as_str()
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Status {
    VALID = 0x00,
    DEACTIVATED = 0x01,
}