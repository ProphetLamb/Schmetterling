use std::collections::{hash_map::Entry::*, HashMap};

use gloo_console::{info, warn};
use gloo_storage::{errors::StorageError, LocalStorage, Storage};

use schling_common::{id, markup::Markup, ord_by};
use serde::{Deserialize, Serialize};

pub trait Head {
    type Body: Serialize + for<'de> Deserialize<'de>;
    fn key(&self) -> String;

    fn load_body(&self) -> Option<Self::Body> {
        get(&self.key()).ok()
    }

    fn set_body(&self, body: Self::Body) {
        set(&self.key(), body);
    }
}

macro_rules! tname {
    ($type:ty) => {
        std::any::type_name::<$type>()
    };
}

macro_rules! body_with_head {
    ($body:ident, $head:ident, $($content:tt)*) => {
        #[derive(PartialEq, Clone, Serialize, Deserialize)]
        pub struct $body {
            pub head: $head,
            $($content)*
        }
        ord_by!($body, head);

        #[derive(PartialEq, Clone, Serialize, Deserialize)]
        pub struct $head {
            pub id: id::$body,
            pub order: u32,
            pub title: String,
        }
        ord_by!($head, order);
    };
}

body_with_head!(Sec, SecHead, pub content: Markup);
impl Head for SecHead {
    type Body = Sec;

    fn key(&self) -> String {
        let key: String = self.id.into();
        format!("{}-{}", tname!(Sec), key)
    }
}

body_with_head!(Doc, DocHead, pub content: Vec<SecHead>);
impl Head for DocHead {
    type Body = Doc;

    fn key(&self) -> String {
        let key: String = self.id.into();
        format!("{}-{}", tname!(Doc), key)
    }
}

body_with_head!(Proj, ProjHead, pub content: Vec<DocHead>);
impl Head for ProjHead {
    type Body = Proj;

    fn key(&self) -> String {
        let key: String = self.id.into();
        format!("{}-{}", tname!(Proj), key)
    }
}

/// Attempts to deserialize a key in the `LocalStorage`, if successful returns the value;
/// otherwise creates the key of the given type, if that fails panics;
/// otherwise attempts to get the created key, if that fails panics.
fn get_or_create<T, F>(key: &str, f: F) -> T
where
    T: Serialize + for<'de> Deserialize<'de>,
    F: FnOnce() -> T,
{
    match get(key) {
        Ok(set) => set,
        Err(err) => {
            warn!(format!("{}", err));
            let value = f();
            set(key, value)
        }
    }
}

fn get<T>(key: &str) -> Result<T, StorageError>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    info!(format!("get key {} of type {}", key, tname!(T)));
    LocalStorage::get(&key)
}

fn set<T>(key: &str, value: T) -> T
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    info!(format!("set key {} of type {}", &key, tname!(T)));
    LocalStorage::set(&key, &value).unwrap_or_else(|e| panic_set(&key, e));
    value
}

fn panic_set(key: &str, e: StorageError) -> ! {
    panic!("unable to create the key '{}'.\nError: {}", key, e)
}
