use near_sdk::{env, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use crate::*;

#[near_bindgen]
impl Contract {
    ///call

    ///contract
    ///
    ///this method adds the relevant file information
    pub fn store(&mut self, sid: String, file_name: String, file_type: String, file_size: f64, file_owner_folder: String) -> bool {
        let account_id = env::signer_account_id();
        let id = gen_account(account_id);

        self.check_file(&sid, &id);
        let file_folder = self.check_folder(&id, file_owner_folder);

        let f = File::new_default(
            id.clone(),
            sid.clone(),
            file_name, file_type,
            file_size,
            file_folder,
            env::block_timestamp());

        self.file_index.insert(&sid, &f);
        self.files_store(sid.clone(), &id);
        self.updated.insert(&id, &env::block_timestamp());
        env::log_str(&format!("contract, id:{}, sid : {}", &id, &sid));
        true
    }

    ///file_delete
    ///
    ///this method will delete file record
    pub fn file_delete(&mut self, sid: String) -> bool {
        let account_id = env::signer_account_id();
        let id = gen_account(account_id);

        let file_status = self.files_store.get(&id).unwrap_or(vec![]);
        if !file_status.contains(&sid) {
            env::panic_str("store, file not exists");
        };

        self.file_index.remove(&sid);

        let mut files = self.files_store.get(&id).unwrap();
        let index = files
            .iter()
            .position(|x| x == &sid)
            .unwrap();

        files.remove(index);
        self.files_store.insert(&id, &files);
        self.updated.insert(&id, &env::block_timestamp());
        env::log_str(&format!("contract, id:{}, sid : {}", &id, &sid));
        true
    }

    ///folder_create
    ///
    ///this method will create a new folder
    pub fn folder_create(&mut self, folder: String) -> bool {
        let account_id = env::signer_account_id();
        let id = gen_account(account_id);

        let mut folders = self.folder_store.get(&id).unwrap_or(vec![]);
        if folders.contains(&folder) {
            env::panic_str("folder_create, folder exists");
        };
        folders.push(folder.clone());
        self.folder_store.insert(&id, &folders);
        self.updated.insert(&id, &env::block_timestamp());
        env::log_str(&format!("folder_create, id:{}, folder : {}", &id, &folder));
        true
    }

    ///folder_rename
    ///
    ///1.delete folder
    ///2.create folder
    ///3.copy the previous folder information
    ///4.Delete previous folder information
    ///this method will rename the folder
    ///if the folder is renamed, all the files in the current folder will be changed based on the folder as the key
    pub fn folder_rename(&mut self, pre_folder: String, folder: String) -> bool {
        let account_id = env::signer_account_id();
        let id = gen_account(account_id);

        let mut files = self.files_view(id.clone());
        for file in files.iter_mut() {
            if file.file_folder.contains(&pre_folder) {
                file.file_folder.retain(|x| x != &pre_folder);
                file.file_folder.push(folder.clone());
                self.file_index.insert(&file.sid, &file);
            }
        }

        let mut folders = self.folder_store.get(&id).unwrap();
        let index = folders
            .iter()
            .position(|x| x == &pre_folder)
            .unwrap();
        folders.remove(index);
        folders.push(folder.clone());
        self.folder_store.insert(&id, &folders);
        self.updated.insert(&id, &env::block_timestamp());
        true
    }

    ///folder_delete
    ///
    ///this method will delete a new folder
    pub fn folder_delete(&mut self, folder: String) -> bool {
        let account_id = env::signer_account_id();
        let id = gen_account(account_id);

        let mut folders = self.folder_store.get(&id).unwrap();
        if !folders.contains(&folder) {
            env::panic_str("folder_delete, folder not exists");
        };

        let index = folders
            .iter()
            .position(|x| x == &folder)
            .unwrap();
        folders.remove(index);
        self.folder_store.insert(&id, &folders);
        self.updated.insert(&id, &env::block_timestamp());
        true
    }

    ///file_copy_to_folder
    ///
    ///this method moves and copies the file to another folder
    pub fn file_copy_to_folder(&mut self, sid: String, folder: String) -> bool {
        let account_id = env::signer_account_id();
        let id = gen_account(account_id);

        let file = self.file_index.get(&sid);
        let folders = self.folder_store.get(&id).unwrap();
        if !folders.contains(&folder) {
            env::panic_str("folder_delete, folder not exists");
        };

        for mut f in file {
            f.file_folder.push(folder.clone());
            self.file_index.insert(&sid, &f);
        }
        self.updated.insert(&id, &env::block_timestamp());
        true
    }

    ///file_delete_in_folder
    ///
    ///this method will delete the files in the folder
    pub fn file_delete_in_folder(&mut self, sid: String, folder: String) -> bool {
        let account_id = env::signer_account_id();
        let id = gen_account(account_id);

        let file = self.file_index.get(&sid);

        let folders = self.folder_store.get(&id).unwrap();
        if !folders.contains(&folder) {
            env::panic_str("folder_delete, folder not exists");
        };

        for mut f in file {
            let index = folders
                .iter()
                .position(|x| x == &folder)
                .unwrap();
            f.file_folder.remove(index);
            self.file_index.insert(&sid, &f);
        }
        self.updated.insert(&id, &env::block_timestamp());
        true
    }

    ///check_file
    ///
    fn check_file(&self, sid: &String, id: &String) {
        let file_status = self.files_store.get(id).unwrap_or(vec![]);
        if file_status.contains(sid) {
            env::panic_str("store, file exists");
        };
    }

    ///check_folder
    ///
    fn check_folder(&self, id: &String, folder: String) -> Vec<String> {
        let mut file_folder = vec![];
        if folder.is_empty() {
            return file_folder;
        }

        let folders = self.folder_store.get(id).unwrap_or(vec![]);
        if !folders.contains(&folder) {
            env::panic_str("check_folder, folder not exists");
        };
        file_folder.push(folder);
        file_folder
    }

    ///files_store
    ///
    fn files_store(&mut self, sid: String, id: &String) {
        let mut s = self.files_store.get(id).unwrap_or(vec![]);
        s.push(sid);
        self.files_store.insert(id, &s);
    }

    ///file_share
    ///
    ///this method stores information about shared files
    pub fn file_share(&mut self, sid: String, cid: String) -> bool {
        let account_id = env::signer_account_id();
        let id = gen_account(account_id);

        let file_status = self.file_index.get(&sid).unwrap();
        let fs = ShareFileInfo::new_default(
            sid.clone(),
            cid.clone(),
            file_status.file_size,
            id.clone(),
            env::block_timestamp());

        let mut s = self.file_share.get(&id).unwrap_or(vec![]);
        let size = fs.file_size.clone();
        self.file_share_index.insert(&sid, &fs);
        s.push(fs);

        self.updated.insert(&id, &env::block_timestamp());
        self.file_share.insert(&id, &s);
        self.update_share_file_size(&id, size);
        true
    }

    ///update_share_file_size
    ///
    ///this method updates the total file size of the file share
    fn update_share_file_size(&mut self, id: &String, size: f64) {
        let mut s = self.file_share_size.get(id).unwrap_or(0 as f64);
        s += size;
        self.file_share_size.insert(&id, &s);
    }

    ///view
    ///
    ///this method looks at all the stored information for the id owner
    ///
    ///return StoreInfo  todo retrun json StoreInfo
    pub fn view_account(&self, id: String) -> ViewInfo {
        let files = self.files_view(id.clone());
        let folders = self.folder_view(id.clone());
        let num = self.file_share.
            get(&id)
            .unwrap_or(vec![])
            .len()
            ;
        let size = self.file_share_size.get(&id).unwrap_or(0 as f64);
        ViewInfo::new_default(
            id,
            files,
            folders, num as u64, size)
    }


    pub(crate) fn files_view(&self, id: String) -> Vec<File> {
        let mut files = vec![];
        let sids = self.files_store.get(&id).unwrap_or(vec![]);
        for s in sids.into_iter() {
            let file = self.file_index.get(&s).expect("files_view: no files!");
            files.push(file);
        }
        files
    }

    pub(crate) fn folder_view(&self, id: String) -> Vec<String> {
        self.folder_store.get(&id).unwrap_or(vec![])
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Debug)]
pub struct ViewInfo {
    pub holder: String,
    pub files: Vec<File>,
    pub file_folders: Vec<String>,
    pub file_share_num: u64,
    pub file_share_size: f64,
}

impl ViewInfo {
    pub fn new_default(holder: String,files: Vec<File>, file_folders: Vec<String>, file_share_num: u64, file_share_size: f64) -> Self {
        ViewInfo {
            holder,
            files,
            file_folders,
            file_share_num,
            file_share_size,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct ShareFileInfo {
    pub sid: String,
    pub cid: String,
    pub file_size: f64,
    pub holder: String,
    pub created: u64,
}

impl ShareFileInfo {
    pub fn new_default(sid: String, cid: String, file_size: f64, holder: String, created: u64) -> Self {
        ShareFileInfo {
            sid,
            cid,
            file_size,
            holder,
            created,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Debug)]
pub struct File {
    pub holder: String,
    pub sid: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: f64,
    pub file_folder: Vec<String>,
    pub created: u64,
}

impl File {
    pub fn new_default(holder: String, sid: String, file_name: String, file_type: String, file_size: f64, file_folder: Vec<String>, created: u64) -> Self {
        File {
            holder,
            sid,
            file_name,
            file_type,
            file_size,
            file_folder,
            created,
        }
    }
}


#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::{testing_env,Balance};
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use super::*;
    const ONE_YOCTO: Balance = 1;

    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        testing_env!(context.attached_deposit(ONE_YOCTO).build());
        testing_env!(context.block_timestamp(1638790720000).build());
        let contract = Contract::default();
        (context, contract)
    }

    #[test]
    fn store_test() {
        let (_context, mut contract) = setup_contract();
        let flag = contract.store(
            "testsid12345".to_string(),
            "test_name".to_string(),
            "test_type".to_string(),
            123.into(),
            String::new(),
        );
        assert_eq!(true,flag);
    }

    #[test]
    fn file_delete_test() {
        let (_context, mut contract) = setup_contract();
        contract.store(
            "testsid12345".to_string(),
            "test_name".to_string(),
            "test_type".to_string(),
            123.into(),
            String::new(),
        );
        let flag = contract.file_delete("testsid12345".to_string());
        assert_eq!(true,flag);
    }

    #[test]
    fn file_folder_test() {
        let (_context, mut contract) = setup_contract();
        let flag1 = contract.folder_create("test_folder".to_string());
        assert_eq!(true,flag1);

        let flag2 = contract.folder_delete("test_folder".to_string());
        assert_eq!(true,flag2);

        contract.folder_create("test_folder1".to_string());
        let flag3 = contract.folder_rename("test_folder1".to_string(),"test_folder2".to_string());
        assert_eq!(true,flag3);
    }

    #[test]
    fn file_view() {
        let (context, mut contract) = setup_contract();
        contract.store(
            "testsid12345".to_string(),
            "test_name".to_string(),
            "test_type".to_string(),
            123.into(),
            String::new(),
        );

        let v = contract.view_account(gen_account(context.context.signer_account_id.parse().unwrap()));
        println!("view_account: {:?}", v);
    }
}