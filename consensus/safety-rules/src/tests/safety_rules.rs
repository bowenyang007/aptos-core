// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{test_utils, tests::suite, SafetyRules};
use aptos_types::validator_signer::ValidatorSigner;

#[test]
fn test() {
    let boolean_values = [false, true];
    for verify_vote_proposal_signature in &boolean_values {
        for export_consensus_key in &boolean_values {
            suite::run_test_suite(&safety_rules(
                *verify_vote_proposal_signature,
                *export_consensus_key,
            ));
        }
    }
}

fn safety_rules(
    verify_vote_proposal_signature: bool,
    export_consensus_key: bool,
) -> suite::Callback {
    Box::new(move || {
        let signer = ValidatorSigner::from_int(0);
        let storage = test_utils::test_storage(&signer);
        let safety_rules = Box::new(SafetyRules::new(
            storage,
            verify_vote_proposal_signature,
            export_consensus_key,
        ));
        (safety_rules, signer)
    })
}
