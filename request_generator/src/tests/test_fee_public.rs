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

/// Samples the request for `fee_public`.
pub(crate) fn sample_fee_public(
    rng: &mut ChaChaRng,
    execution_id: Field<CurrentNetwork>,
) -> Result<Request<CurrentNetwork>> {
    // Sample the sender.
    let (private_key, _) = sample_account(rng);
    // Sample the program ID and function name.
    let program_id = ProgramID::from_str("credits.aleo").unwrap();
    // Determine the base fee in microcredits.
    let base_fee_in_microcredits = 100; // TODO: set the correct base fee.
    // Determine the priority fee in microcredits.
    let priority_fee_in_microcredits = 10; // TODO: set the correct priority fee.
    // Authorize the fee.
    let fee_function_name = Identifier::from_str("fee_public")?;
    // Construct the fee inputs.
    let fee_inputs = vec![
        Value::from(Literal::U64(U64::<CurrentNetwork>::new(base_fee_in_microcredits))),
        Value::from(Literal::U64(U64::<CurrentNetwork>::new(priority_fee_in_microcredits))),
        Value::from(Literal::Field(execution_id)),
    ];
    // Construct the fee input types.
    let input_types = [
        ValueType::from_str("u64.public").unwrap(),
        ValueType::from_str("u64.public").unwrap(),
        ValueType::from_str("field.public").unwrap(),
    ];
    // Authorize the fee function.
    create_request(&private_key, program_id, fee_function_name, fee_inputs, &input_types, rng)
}

#[test]
fn test_fee_public_local() {
    let rng = &mut ChaChaRng::seed_from_u64(0u64);
    // let rng = &mut TestRng::fixed(0); // TODO: passing a private key file around might be a safer default, even if this is PoC code.

    // Read the execution ID from the file.
    let execution_id_string = std::fs::read_to_string("../execution_id.txt").unwrap();
    let execution_id = Field::<CurrentNetwork>::from_str(&execution_id_string).unwrap();
    // Sample the request.
    let request = sample_fee_public(rng, execution_id).unwrap();
    // Serialize the request to a string.
    let request_string = request.to_string();
    // Write the request to a file.
    std::fs::write("../fee_public_request.txt", request_string).unwrap();
}
