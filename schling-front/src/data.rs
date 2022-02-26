use gloo_console::info;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};

use schling_common::{id, markup::Markup, ord_by};
use serde::{Deserialize, Serialize};

pub trait Head {
    type Body: Serialize + for<'de> Deserialize<'de>;
    fn key(&self) -> String;

    fn load(&self) -> Option<Self::Body> {
        let key = self.key();
        get(&key).ok()
    }

    fn update(&self, body: Self::Body) -> Self::Body {
        let key = self.key();
        set(&key, body)
    }
}

#[macro_export]
macro_rules! tname {
    ($type:ty) => {
        std::any::type_name::<$type>()
    };
}

macro_rules! data_for_head {
    ($head:ident, $body:ident, $content:ty) => {
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        pub struct $head {
            pub id: id::$body,
            pub order: u32,
            pub title: String,
        }
        ord_by!($head, order);

        impl $head {
            pub fn new(id: id::$body, order: u32, title: String) -> Self {
                Self { id, order, title }
            }

            pub fn body(self, content: $content) -> $body {
                $body {
                    head: self,
                    content,
                }
            }

            pub fn own_with_title(&self, title: String) -> Self {
                Self {
                    id: self.id,
                    order: self.order,
                    title,
                }
            }
        }

        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        pub struct $body {
            pub head: $head,
            pub content: $content,
        }
        ord_by!($body, head);

        impl $body {
            pub fn new(head: $head, content: $content) -> Self {
                Self { head, content }
            }

            pub fn own_with_title(&self, title: String) -> Self {
                Self {
                    head: self.head.own_with_title(title),
                    content: self.content.clone(),
                }
            }

            pub fn own_with_content(&self, content: $content) -> Self {
                Self {
                    head: self.head.clone(),
                    content,
                }
            }
        }
    };
}

data_for_head!(SecHead, Sec, Markup);
impl Head for id::Sec {
    type Body = Sec;

    fn key(&self) -> String {
        let key: String = (*self).into();
        format!("{}-{}", tname!(Sec), key)
    }
}

data_for_head!(DocHead, Doc, Vec<SecHead>);
impl Head for id::Doc {
    type Body = Doc;

    fn key(&self) -> String {
        let key: String = (*self).into();
        format!("{}-{}", tname!(Doc), key)
    }
}

data_for_head!(ProjHead, Proj, Vec<DocHead>);
impl Head for id::Proj {
    type Body = Proj;

    fn key(&self) -> String {
        let key: String = (*self).into();
        format!("{}-{}", tname!(Proj), key)
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
    LocalStorage::set(&key, &value).unwrap_or_else(|e| panic_set(key, e));
    value
}

fn panic_set(key: &str, e: StorageError) -> ! {
    panic!("unable to create the key '{}'.\nError: {}", key, e)
}
