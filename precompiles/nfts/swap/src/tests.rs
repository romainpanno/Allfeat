// This file is part of Allfeat.

// Copyright (C) 2022-2024 Allfeat.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::{mock::*, NftsSwapPrecompileCall};
use pallet_evm_precompile_nfts_tests::{ExtBuilder, ALICE, BOB};
use precompile_utils::testing::*;
use sp_core::U256;
use pallet_evm_precompile_nfts_types::solidity::{OptionalU256,OptionalPriceWithDirection, PriceWithDirection, PriceDirection};
use pallet_nfts::Pallet as NftsPallet;
use frame_support::assert_ok;
use frame_system::pallet_prelude::OriginFor;

type PCall = NftsSwapPrecompileCall<Runtime>;

fn _precompiles() -> Precompiles<Runtime> {
	PrecompilesValue::get()
}

fn mint_each_collections() {
	assert_ok!(NftsPallet::<Runtime>::mint(
		OriginFor::<Runtime>::signed(ALICE.into()),
		0,
		1,
		ALICE.into(),
		None
	));
	assert_ok!(NftsPallet::<Runtime>::mint(
		OriginFor::<Runtime>::signed(BOB.into()),
		1,
		1,
		BOB.into(),
		None
	));
}

fn get_owner_of_item(collection_id: u128, item_id: u128) -> Option<AccountId> {
	let owner_id: Option<AccountId>  = NftsPallet::<Runtime>::owner(collection_id, item_id);

	owner_id
}

#[test]
fn selectors() {
	assert!(PCall::create_swap_selectors().contains(&0xf93f143a));
	assert!(PCall::cancel_swap_selectors().contains(&0x83698d19));
	assert!(PCall::claim_swap_selectors().contains(&0x0406ab6c));
}

#[test]
fn create_swap_works() {
	ExtBuilder::<Runtime>::default()
        .with_balances(vec![(ALICE.into(), 1000), (BOB.into(), 1000)])
        .build_with_collections()
        .execute_with(|| {
			mint_each_collections();

			assert_eq!(get_owner_of_item(0, 1), Some(ALICE.into()));
			_precompiles().prepare_test(
				ALICE,
				Precompile1,
				PCall::create_swap {
					offered_collection: 0.into(),
					offered_item: 1.into(),
					desired_collection: 1.into(),
					maybe_desired_item: OptionalU256::from(Some(U256::from(1))),
					maybe_price: OptionalPriceWithDirection::from(Some(PriceWithDirection::new(U256::from(100), PriceDirection::Receive))),
					duration: 100.into(),
				},
			).execute_returns(true);
			assert_eq!(get_owner_of_item(0, 1), Some(ALICE.into()));
        });
}

#[test]
fn cancel_swap_works() {
	ExtBuilder::<Runtime>::default()
        .with_balances(vec![(ALICE.into(), 1000), (BOB.into(), 1000)])
        .build_with_collections()
        .execute_with(|| {
			mint_each_collections();

			assert_eq!(get_owner_of_item(0, 1), Some(ALICE.into()));
			_precompiles().prepare_test(
				ALICE,
				Precompile1,
				PCall::create_swap {
					offered_collection: 0.into(),
					offered_item: 1.into(),
					desired_collection: 1.into(),
					maybe_desired_item: OptionalU256::from(Some(U256::from(1))),
					maybe_price: OptionalPriceWithDirection::from(Some(PriceWithDirection::new(U256::from(100), PriceDirection::Receive))),
					duration: 100.into(),
				},
			).execute_returns(true);
			_precompiles().prepare_test(
				ALICE,
                Precompile1,
                PCall::cancel_swap {
                    offered_collection: 0.into(),
                    offered_item: 1.into(),
                },
			).execute_returns(true);
			assert_eq!(get_owner_of_item(0, 1), Some(ALICE.into()));
        });
}

#[test]
fn claim_swap_works() {
	ExtBuilder::<Runtime>::default()
		.with_balances(vec![(ALICE.into(), 1000), (BOB.into(), 1000)])
		.build_with_collections()
		.execute_with(|| {
			mint_each_collections();

			assert_eq!(get_owner_of_item(0, 1), Some(ALICE.into()));
			_precompiles().prepare_test(
				ALICE,
				Precompile1,
				PCall::create_swap {
					offered_collection: 0.into(),
					offered_item: 1.into(),
					desired_collection: 1.into(),
					maybe_desired_item: OptionalU256::from(Some(U256::from(1))),
					maybe_price: OptionalPriceWithDirection::from(Some(PriceWithDirection::new(U256::from(100), PriceDirection::Receive))),
					duration: 100.into(),
				},
			).execute_returns(true);
			_precompiles().prepare_test(
				BOB,
				Precompile1,
				PCall::claim_swap {
					send_collection: 1.into(),
					send_item: 1.into(),
					receive_collection: 0.into(),
					receive_item: 1.into(),
					witness_price: OptionalPriceWithDirection::from(Some(PriceWithDirection::new(U256::from(100), PriceDirection::Receive))),
				},
			).execute_returns(true);
			assert_eq!(get_owner_of_item(0, 1), Some(BOB.into()));
		});
}
