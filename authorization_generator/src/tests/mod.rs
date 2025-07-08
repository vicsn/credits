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

use crate::PROCESS;

use aleo_std::StorageMode;
use snarkvm::{
    console::program::Request,
    ledger::store::{ConsensusStore, helpers::memory::ConsensusMemory},
    prelude::{Authorization, Transaction},
    synthesizer::VM,
    utilities::TestRng,
};

use std::str::FromStr;

use anyhow::Result;

type CurrentNetwork = snarkvm::console::network::MainnetV0;
type CurrentAleo = snarkvm::circuit::AleoV0;
type CurrentStorage = ConsensusMemory<CurrentNetwork>;
