// Copyright (C) 2019-2023 Howard Wu
// This file is part of the credits library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
mod tests;

use snarkvm_console_account::PrivateKey;
use snarkvm_console_program::{Identifier, ProgramID, Request, Value, ValueType};
pub type N = snarkvm_console_network::MainnetV0;

use anyhow::Result;
use rand::{CryptoRng, Rng};

/// Creates a Request for a function call with a corresponding fee.
pub fn create_request(
    private_key: &PrivateKey<N>,
    program_id: ProgramID<N>,
    function_name: Identifier<N>,
    inputs: Vec<Value<N>>,
    input_types: &[ValueType<N>],
    rng: &mut (impl Rng + CryptoRng),
) -> Result<Request<N>> {
    // Set is_root to true.
    let is_root = true;
    // This is the root request and we do not have a root_tvk to pass on.
    let root_tvk = None;
    // Compute the request.
    Request::sign(private_key, program_id, function_name, inputs.into_iter(), input_types, root_tvk, is_root, rng)
}
