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

mod test_bond_public;
mod test_fee_public;
mod test_transfer_public;

use crate::create_request;

use snarkvm_console_account::{Address, PrivateKey};
use snarkvm_console_program::{Identifier, Literal, ProgramID, Request, Value, ValueType};
use snarkvm_console_types::{Field, U64};

type CurrentNetwork = snarkvm_console_network::MainnetV0;

use std::str::FromStr;

use anyhow::Result;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

// TODO: use proper RNG.

/// Samples a random private key and address.
fn sample_account(rng: &mut ChaChaRng) -> (PrivateKey<CurrentNetwork>, Address<CurrentNetwork>) {
    let private_key = PrivateKey::<CurrentNetwork>::new(rng).unwrap();
    let address = Address::<CurrentNetwork>::try_from(&private_key).unwrap();
    (private_key, address)
}
