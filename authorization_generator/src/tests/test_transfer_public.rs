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

use super::*;

/// Samples the execution for `transfer_public`.
pub(crate) fn sample_transfer_public(rng: &mut TestRng) -> Result<Transaction<CurrentNetwork>> {
    // Sample the sender.
    let (private_key, _) = sample_account(rng);
    // Read the request from the file.
    let request_string = std::fs::read_to_string("../transfer_public_request.txt")?;
    // Parse the request from the string.
    let request: Request<CurrentNetwork> = Request::from_str(&request_string)?;
    // Create the authorization from the request.
    let authorization = PROCESS.authorize_credits_public::<CurrentAleo, _>(&private_key, request, rng)?;
    // Create a DB store for the consensus.
    let store = ConsensusStore::<CurrentNetwork, CurrentStorage>::open(StorageMode::new_test(None)).unwrap();
    // Create a VM from the store.
    let vm = VM::from(store).unwrap();
    // Execute the transaction.
    vm.execute_authorization(authorization, None, None, rng)
}

#[test]
fn test_transfer_public_local() {
    let rng = &mut TestRng::fixed(1);

    // Sample the tx.
    let tx = sample_transfer_public(rng).unwrap();
    // Extract the execution ID.
    let execution_id = tx.execution().unwrap().to_execution_id().unwrap();
    // Serialize the tx to a string.
    let tx_string = tx.to_string();
    // Write the tx to a file.
    std::fs::write("../tx.txt", tx_string).unwrap();
    // Write the execution ID to a file.
    std::fs::write("../execution_id.txt", execution_id.to_string()).unwrap()
}
