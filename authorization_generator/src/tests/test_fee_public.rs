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

/// Samples the execution for `fee_public`.
pub(crate) fn sample_fee_public(rng: &mut TestRng) -> Result<Authorization<CurrentNetwork>> {
    // Read the request from the file.
    let request_string = std::fs::read_to_string("../fee_public_request.txt")?;
    // Parse the request from the string.
    let request: Request<CurrentNetwork> = Request::from_str(&request_string)?;
    // Create the authorization from the request.
    PROCESS.authorize_request::<CurrentAleo, _>(request, rng)
}

#[test]
fn test_fee_public_local() {
    let rng = &mut TestRng::fixed(1);

    // Sample the authorization.
    let fee_authorization = sample_fee_public(rng).unwrap();
    // Read the tx from the file.
    let tx_string = std::fs::read_to_string("../tx.txt").unwrap();
    // Parse the tx from the string.
    let tx: Transaction<CurrentNetwork> = Transaction::from_str(&tx_string).unwrap();
    // Create a DB store for the consensus.
    let store = ConsensusStore::<CurrentNetwork, CurrentStorage>::open(StorageMode::new_test(None)).unwrap();
    // Create a VM from the store.
    let vm = VM::from(store).unwrap();
    // Execute the fee transaction.
    let fee = vm.execute_fee_authorization(fee_authorization, None, rng).unwrap();
    // Destructure the Transaction.
    let tx = if let Transaction::Execute(_, _, execution, _) = tx {
        Transaction::from_execution(*execution, Some(fee)).unwrap()
    } else {
        panic!("Expected a Transaction::Execute variant");
    };
    // Verify the transaction.
    vm.check_transaction(&tx, None, rng).unwrap();
}
