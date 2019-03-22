// Copyright 2018-2019, Wayfair GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod debug;
pub mod generic;
pub mod grouper;
pub mod identity;
pub mod runtime;

use super::{Event, NodeConfig};
use crate::errors::*;
use hashbrown::HashMap;
use serde_json::Value;

pub trait Operator: std::fmt::Debug + Send {
    fn on_event(&mut self, port: &str, event: Event) -> Result<Vec<(String, Event)>>;

    fn handles_signal(&self) -> bool {
        false
    }
    // A lot of operators won't need to handle signals so we default to
    // passing the signal through
    fn on_signal(&mut self, _signal: &mut Event) -> Result<Vec<(String, Event)>> {
        Ok(vec![])
    }

    fn handles_contraflow(&self) -> bool {
        false
    }
    // A lot of operators won't need to handle insights so we default to
    // passing the isnight through
    fn on_contraflow(&mut self, _insight: &mut Event) {}

    // Returns metrics for this operator
    fn metrics(&self, _tags: HashMap<String, String>, _timestamp: u64) -> Vec<Value> {
        Vec::new()
    }
}

pub trait InitializableOperator {
    fn from_node(&self, node: &NodeConfig) -> Result<Box<dyn Operator>>;
}