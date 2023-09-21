// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// SPDX-License-Identifier: MIT

use cargo_metadata::camino::Utf8PathBuf;
use fs2::FileExt;
use proc_macro2::TokenStream;
use quote::ToTokens;
use serde_json;
use std::{
    env,
    fs::{
        create_dir_all,
        File,
        OpenOptions,
    },
    io::{
        BufReader,
        Seek,
        SeekFrom,
    },
    str::FromStr,
};
use syn::{
    ItemTrait,
    TraitItem,
};

const TEMP_FOLDER: &str = "__openbrush_metadata_folder";

pub(crate) struct TraitDefinition(ItemTrait);

impl TraitDefinition {
    pub(crate) fn new(item: ItemTrait) -> Self {
        Self(item)
    }

    pub(crate) fn methods(&self) -> Vec<syn::TraitItemMethod> {
        self.0
            .items
            .clone()
            .into_iter()
            .filter_map(|item| {
                if let TraitItem::Method(method) = item {
                    Some(method)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub(crate) struct LockedTrait {
    file: File,
    pub(crate) trait_definition: Option<TraitDefinition>,
}

impl LockedTrait {
    pub(crate) fn new(trait_name: String) -> Self {
        let file = get_locked_file(trait_name);
        let reader = BufReader::new(&file);
        let token_string: String = serde_json::from_reader(reader).unwrap_or_default();

        let stream = TokenStream::from_str(token_string.as_str()).unwrap_or_default();
        let trait_item = syn::parse2::<ItemTrait>(stream).ok();
        let trait_definition;

        if let Some(trait_item) = trait_item {
            trait_definition = Some(TraitDefinition::new(trait_item));
        } else {
            trait_definition = None;
        }

        Self { file, trait_definition }
    }
}

impl Drop for LockedTrait {
    fn drop(&mut self) {
        self.file.set_len(0).expect("Can't truncate the file");
        self.file.seek(SeekFrom::Start(0)).expect("Can't set cursor position");
        if let Some(trait_definition) = &self.trait_definition {
            serde_json::to_writer(&self.file, &trait_definition.0.to_token_stream().to_string())
                .expect("Can't dump definition metadata to file");
        }
        self.file.unlock().expect("Can't remove exclusive lock");
    }
}

/// Function returns exclusively locked file for metadata.
/// It stores file in the target folder where `ink` is stored.
fn get_locked_file(name: String) -> File {
    use crate::internal::INK_PREFIX;
    const SUFFIX: &str = "target";

    let target: String = env::args()
        .find(|arg| arg.contains(INK_PREFIX))
        .unwrap_or_else(|| panic!("Unable to find PREFIX: {:?}", env::args()));
    let target: String = target
        .chars()
        .skip(INK_PREFIX.len())
        .take(target.find(SUFFIX).expect("Unable to find debug/deps") - INK_PREFIX.len() + SUFFIX.len())
        .collect();

    let target_dir = Utf8PathBuf::from_str(target.as_str()).expect("Can't generate Path from target");
    let dir = target_dir.join(TEMP_FOLDER);
    create_dir_all(&dir).expect("Couldn't create temporary storage");
    let dir = dir.join(name);

    let file = match OpenOptions::new().create(true).read(true).write(true).open(&dir) {
        Err(why) => panic!("Couldn't open temporary storage {} : {}", dir, why),
        Ok(file) => file,
    };
    file.lock_exclusive().expect("Can't do exclusive lock");

    file
}
