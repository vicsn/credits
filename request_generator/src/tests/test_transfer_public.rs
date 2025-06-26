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

/// Samples the request for `transfer_public`.
pub(crate) fn sample_transfer_public(rng: &mut ChaChaRng) -> Result<Request<CurrentNetwork>> {
    // Sample the sender.
    let (private_key, _) = sample_account(rng);
    // Sample the receiver.
    let (_, receiver_address) = sample_account(rng);
    // Sample the amount in microcredits.
    let amount_in_microcredits = rng.gen_range(10000000..100000000);
    // Sample the program ID and function name.
    let program_id = ProgramID::from_str("credits.aleo").unwrap();
    let function_name = Identifier::from_str("transfer_public").unwrap();
    // Construct the inputs to the function.
    let inputs = vec![
        Value::<CurrentNetwork>::from(Literal::Address(receiver_address)),
        Value::from(Literal::U64(U64::new(amount_in_microcredits))),
    ];
    // Construct the input types.
    let input_types = [ValueType::from_str("address.public").unwrap(), ValueType::from_str("u64.public").unwrap()];
    // Create the request.
    create_request(&private_key, program_id, function_name, inputs, &input_types, rng)
}

#[test]
fn test_transfer_public_local() {
    let rng = &mut ChaChaRng::seed_from_u64(0u64);
    // let rng = &mut TestRng::fixed(0);

    // Sample the request.
    let request = sample_transfer_public(rng).unwrap();
    // Serialize the request to a string.
    let request_string = request.to_string();
    // Write the request to a file.
    std::fs::write("../transfer_public_request.txt", request_string).unwrap();
}
