    use std::collections::HashMap;
    use std::sync::Mutex;

    use prism_client::Account;

    // In memory database for storing data in application
    pub struct Database {
        // Map of user id to account
        pub accounts: Mutex<HashMap<String, Account>>,

        // Map of user id to keys
        pub keys: Mutex<HashMap<String, Vec<String>>>,

        // Map of user id to data
        pub data: Mutex<HashMap<String, Vec<String>>>,
    }

    impl Default for Database {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Database {
        pub fn new() -> Self {
            Self {
                accounts: Mutex::new(HashMap::new()),
                keys: Mutex::new(HashMap::new()),
                data: Mutex::new(HashMap::new()),
            }
        }

        pub fn get_accounts(&self) -> Vec<String> {
            self.accounts.lock().unwrap().keys().cloned().collect()
        }

        pub fn get_keys(&self, id: String) -> Vec<String> {
            self.keys.lock().unwrap().get(&id).cloned().unwrap_or_default()
        }

        pub fn insert_account(&self, id: String, account: Account) {
            self.accounts.lock().unwrap().insert(id, account);
        }

        pub fn insert_key(&self, id: String, key: String) {
            self.keys.lock().unwrap().entry(id).or_default().push(key);
        }

        pub fn insert_data(&self, id: String, data: String) {
            self.data.lock().unwrap().entry(id).or_default().push(data);
        }

        pub fn get_data(&self, id: String) -> Vec<String> {
            self.data.lock().unwrap().get(&id).cloned().unwrap_or_default()
        }

        pub fn get_key(&self, id: String) -> Vec<String> {
            self.keys.lock().unwrap().get(&id).cloned().unwrap_or_default()
        }
    }
