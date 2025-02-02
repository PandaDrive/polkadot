// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_message_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `i9`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet-message-queue
// --extrinsic=*
// --heap-pages=4096
// --header=file_header.txt
// --output
// runtime/kusama/src/weights/pallet_message_queue.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_message_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_message_queue::WeightInfo for WeightInfo<T> {
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn ready_ring_knit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `837`
		//  Estimated: `5554`
		// Minimum execution time: 5_669 nanoseconds.
		Weight::from_parts(5_925_000, 0)
			.saturating_add(Weight::from_parts(0, 5554))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	fn ready_ring_unknit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `837`
		//  Estimated: `5554`
		// Minimum execution time: 5_604 nanoseconds.
		Weight::from_parts(5_993_000, 0)
			.saturating_add(Weight::from_parts(0, 5554))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn service_queue_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `576`
		//  Estimated: `2527`
		// Minimum execution time: 2_104 nanoseconds.
		Weight::from_parts(2_241_000, 0)
			.saturating_add(Weight::from_parts(0, 2527))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn service_page_base_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `648`
		//  Estimated: `68060`
		// Minimum execution time: 3_175 nanoseconds.
		Weight::from_parts(3_289_000, 0)
			.saturating_add(Weight::from_parts(0, 68060))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn service_page_base_no_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `648`
		//  Estimated: `68060`
		// Minimum execution time: 3_198 nanoseconds.
		Weight::from_parts(3_308_000, 0)
			.saturating_add(Weight::from_parts(0, 68060))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_page_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `969`
		//  Estimated: `0`
		// Minimum execution time: 46_864 nanoseconds.
		Weight::from_parts(47_073_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:0)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn bump_service_head() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `712`
		//  Estimated: `3027`
		// Minimum execution time: 3_552 nanoseconds.
		Weight::from_parts(3_710_000, 0)
			.saturating_add(Weight::from_parts(0, 3027))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn reap_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66857`
		//  Estimated: `70587`
		// Minimum execution time: 38_491 nanoseconds.
		Weight::from_parts(46_983_000, 0)
			.saturating_add(Weight::from_parts(0, 70587))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn execute_overweight_page_removed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66857`
		//  Estimated: `70587`
		// Minimum execution time: 88_529 nanoseconds.
		Weight::from_parts(144_649_000, 0)
			.saturating_add(Weight::from_parts(0, 70587))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn execute_overweight_page_updated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66857`
		//  Estimated: `70587`
		// Minimum execution time: 79_494 nanoseconds.
		Weight::from_parts(84_895_000, 0)
			.saturating_add(Weight::from_parts(0, 70587))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
