// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::sync::Arc;
use log::warn;

use risingwave_common::error::{Result, ToRwResult};
use risingwave_connector::{extract_split_enumerator, SourceSplit};

use crate::barrier::BarrierManagerRef;
use crate::model::ActorId;
use crate::storage::MetaStore;

pub type SourceManagerRef<S> = Arc<SourceManager<S>>;

pub type SourceSplitID = String;

#[allow(dead_code)]
pub struct SourceManager<S>
    where
        S: MetaStore,
{
    meta_store_ref: Arc<S>,
    barrier_manager_ref: BarrierManagerRef<S>,
    last_assignment: HashMap<ActorId, Vec<String>>,
}


impl<S> SourceManager<S>
    where
        S: MetaStore,
{
    pub async fn new(meta_store: Arc<S>, barrier_manager: BarrierManagerRef<S>) -> Result<Self> {
        Ok(Self {
            meta_store_ref,
            barrier_manager_ref,
            last_assignment: Default::default(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        // todo: fill me
        Ok(())
    }

    async fn register_source(properties: HashMap<String, String>, actors: Vec<ActorId>) -> Result<()> {
        let mut enumerator = extract_split_enumerator(&properties).to_rw_result()?;

        tokio::spawn(async move {
            loop {
                let result = enumerator.list_splits().await;

                if result.is_err() {
                    warn!("Failed to list splits: {}", result.err().unwrap());
                } else {}


                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        Ok(())
    }

    // async fn fetch_split() -> Result<()> {
    //
    // }
}
