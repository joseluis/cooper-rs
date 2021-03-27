// cooper-rs/examples/threaded_keyval.rs
//
// This is an example app for the Rust "cooper" library.
//
// Copyright (c) 2021, Frank Pagliughi <fpagliughi@mindspring.com>
// All Rights Reserved
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use std::collections::HashMap;
use cooper::ThreadedActor;

/// The internal state type for the Actor
type State = HashMap<String, String>;

/// An actor that can act as a shared key/value store of strings.
#[derive(Clone)]
pub struct SharedMap {
    actor: ThreadedActor<State>,
}

impl SharedMap {
    /// Create a new actor to share a key/value map of string.
    pub fn new() -> Self {
        Self { actor: ThreadedActor::new() }
    }

    /// Insert a value into the shared map.
    pub fn insert<K,V>(&self, key: K, val: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        let val = val.into();

        self.actor.cast(move |state| {
            state.insert(key, val);
        });
    }


    /// Gets the value, if any, from the shared map that is
    /// associated with the key.
    pub fn get<K: Into<String>>(&self, key: K) -> Option<String> {
        let key = key.into();
        self.actor.call(move |state| {
            state.get(&key).map(|v| v.to_string())
        })
    }
}

// --------------------------------------------------------------------------

fn main() {
    let map = SharedMap::new();

    println!("Inserting entry 'city'...");
    map.insert("city", "Boston");

    println!("Retrieving entry...");
    match map.get("city") {
        Some(s) => println!("Got: {}", s),
        None => println!("Error: No entry found"),
    }
}

