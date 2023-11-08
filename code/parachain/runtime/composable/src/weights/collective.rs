
//! Autogenerated weights for `collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-01, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `d10f4923b852`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("composable-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/jif3kmz9kgiwz8hg8nzb9d2kiga1rnga-composable/bin/composable
// benchmark
// pallet
// --chain=composable-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/composable/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> collective::WeightInfo for WeightInfo<T> {
	/// Storage: Council Members (r:1 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:100 w:100)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3233 ±0) + p * (3223 ±0)`
		//  Estimated: `16054 + m * (7806 ±30) + p * (10238 ±30)`
		// Minimum execution time: 30_474 nanoseconds.
		Weight::from_ref_time(30_838_000)
			.saturating_add(Weight::from_proof_size(16054))
			// Standard Error: 144_410
			.saturating_add(Weight::from_ref_time(9_043_468).saturating_mul(m.into()))
			// Standard Error: 144_410
			.saturating_add(Weight::from_ref_time(13_857_141).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_proof_size(7806).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(10238).saturating_mul(p.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// Proof: CallFilter DisabledCalls (max_values: None, max_size: Some(212), added: 2687, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177 + m * (32 ±0)`
		//  Estimated: `3360 + m * (32 ±0)`
		// Minimum execution time: 34_900 nanoseconds.
		Weight::from_ref_time(34_264_013)
			.saturating_add(Weight::from_proof_size(3360))
			// Standard Error: 292
			.saturating_add(Weight::from_ref_time(2_303).saturating_mul(b.into()))
			// Standard Error: 3_018
			.saturating_add(Weight::from_ref_time(50_685).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_proof_size(32).saturating_mul(m.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:0)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// Proof: CallFilter DisabledCalls (max_values: None, max_size: Some(212), added: 2687, mode: MaxEncodedLen)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177 + m * (32 ±0)`
		//  Estimated: `6013 + m * (64 ±0)`
		// Minimum execution time: 38_827 nanoseconds.
		Weight::from_ref_time(37_948_667)
			.saturating_add(Weight::from_proof_size(6013))
			// Standard Error: 287
			.saturating_add(Weight::from_ref_time(2_066).saturating_mul(b.into()))
			// Standard Error: 2_958
			.saturating_add(Weight::from_ref_time(83_504).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(Weight::from_proof_size(64).saturating_mul(m.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalCount (r:1 w:1)
	/// Proof Skipped: Council ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `423 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `5680 + m * (165 ±0) + p * (180 ±0)`
		// Minimum execution time: 46_676 nanoseconds.
		Weight::from_ref_time(37_705_900)
			.saturating_add(Weight::from_proof_size(5680))
			// Standard Error: 567
			.saturating_add(Weight::from_ref_time(7_507).saturating_mul(b.into()))
			// Standard Error: 5_919
			.saturating_add(Weight::from_ref_time(79_126).saturating_mul(m.into()))
			// Standard Error: 5_844
			.saturating_add(Weight::from_ref_time(498_242).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_proof_size(165).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(180).saturating_mul(p.into()))
	}
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `873 + m * (64 ±0)`
		//  Estimated: `4714 + m * (128 ±0)`
		// Minimum execution time: 45_911 nanoseconds.
		Weight::from_ref_time(50_867_042)
			.saturating_add(Weight::from_proof_size(4714))
			// Standard Error: 8_015
			.saturating_add(Weight::from_ref_time(185_981).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(128).saturating_mul(m.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `493 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `5353 + m * (260 ±0) + p * (144 ±0)`
		// Minimum execution time: 50_949 nanoseconds.
		Weight::from_ref_time(42_432_089)
			.saturating_add(Weight::from_proof_size(5353))
			// Standard Error: 6_758
			.saturating_add(Weight::from_ref_time(142_491).saturating_mul(m.into()))
			// Standard Error: 6_590
			.saturating_add(Weight::from_ref_time(448_396).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(260).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(144).saturating_mul(p.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// Proof: CallFilter DisabledCalls (max_values: None, max_size: Some(212), added: 2687, mode: MaxEncodedLen)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `905 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `11611 + b * (4 ±0) + m * (264 ±0) + p * (160 ±0)`
		// Minimum execution time: 81_279 nanoseconds.
		Weight::from_ref_time(74_497_827)
			.saturating_add(Weight::from_proof_size(11611))
			// Standard Error: 1_035
			.saturating_add(Weight::from_ref_time(4_709).saturating_mul(b.into()))
			// Standard Error: 10_949
			.saturating_add(Weight::from_ref_time(93_525).saturating_mul(m.into()))
			// Standard Error: 10_673
			.saturating_add(Weight::from_ref_time(608_670).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(4).saturating_mul(b.into()))
			.saturating_add(Weight::from_proof_size(264).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(160).saturating_mul(p.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:0)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `513 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `6420 + m * (325 ±0) + p * (180 ±0)`
		// Minimum execution time: 56_715 nanoseconds.
		Weight::from_ref_time(50_972_543)
			.saturating_add(Weight::from_proof_size(6420))
			// Standard Error: 6_412
			.saturating_add(Weight::from_ref_time(102_494).saturating_mul(m.into()))
			// Standard Error: 6_253
			.saturating_add(Weight::from_ref_time(432_836).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(325).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(180).saturating_mul(p.into()))
	}
	/// Storage: Council Voting (r:1 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Members (r:1 w:0)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:0)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:1 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// Proof: CallFilter DisabledCalls (max_values: None, max_size: Some(212), added: 2687, mode: MaxEncodedLen)
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `925 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `12952 + b * (5 ±0) + m * (330 ±0) + p * (200 ±0)`
		// Minimum execution time: 84_912 nanoseconds.
		Weight::from_ref_time(80_675_566)
			.saturating_add(Weight::from_proof_size(12952))
			// Standard Error: 1_054
			.saturating_add(Weight::from_ref_time(6_805).saturating_mul(b.into()))
			// Standard Error: 11_149
			.saturating_add(Weight::from_ref_time(97_989).saturating_mul(m.into()))
			// Standard Error: 10_867
			.saturating_add(Weight::from_ref_time(566_467).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(5).saturating_mul(b.into()))
			.saturating_add(Weight::from_proof_size(330).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(200).saturating_mul(p.into()))
	}
	/// Storage: Council Proposals (r:1 w:1)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Voting (r:0 w:1)
	/// Proof Skipped: Council Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council ProposalOf (r:0 w:1)
	/// Proof Skipped: Council ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258 + p * (32 ±0)`
		//  Estimated: `1269 + p * (96 ±0)`
		// Minimum execution time: 26_570 nanoseconds.
		Weight::from_ref_time(30_605_112)
			.saturating_add(Weight::from_proof_size(1269))
			// Standard Error: 5_316
			.saturating_add(Weight::from_ref_time(420_315).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(96).saturating_mul(p.into()))
	}
}
