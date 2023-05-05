use web_sys::window;
use web_sys::Storage;

pub struct Cache(Storage);

impl Cache {
    pub fn new() -> Self {
        Cache(
            window()
                .expect("error while getting the Window object")
                .local_storage()
                .expect("error while getting the LocalStorage object")
                .expect("LocalStorage object is None"),
        )
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.0
            .get_item(key)
            .expect("error while getting item from LocalStorage")
    }

    pub fn set(&self, key: &str, value: &str) {
        self.0
            .set_item(key, value)
            .expect("error while setting item in LocalStorage");
    }

    // pub fn remove(&self, key: &str) {
    //     self.0
    //         .remove_item(key)
    //         .expect("error while deleting item from LocalStorage");
    // }
}
