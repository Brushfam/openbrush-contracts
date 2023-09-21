# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v4.0.0-beta]
## Changes

### Added
- [*BREAKING*] `implementation`, `override`, `default_impl` macros: [#78](https://github.com/Brushfam/openbrush-contracts/pull/78) 
- [*BREAKING*] `storage_item` macro, which implements `#[ink::storage_item]` macro, but also allows to make field of struct upgradeable by using `#[lazy]` attribute. For all fields that
 are either `Lazy`/`Mapping`/`MultiMapping` is generated it's own constant storage key. Also it allows OpenBrush to work correctly with every default implementation [#99](https://github.com/Brushfam/openbrush-contracts/pull/99)
- New OB feature: `Upgradeable`, which implements `set_code_hash` functionality [#99](https://github.com/Brushfam/openbrush-contracts/pull/99)
- `UI` tests for testing different scenarios for macros [#77](https://github.com/Brushfam/openbrush-contracts/pull/77)
- `openbrush::accessors` macro for automatic generation of getters/setters for storage items: [#66](https://github.com/Brushfam/openbrush-contracts/pull/66) and [61](https://github.com/Brushfam/openbrush-contracts/pull/61)

### Removed
- [*BREAKING*] `upgradeable_storage` macro, `OccupyStorage` trait [#99](https://github.com/Brushfam/openbrush-contracts/pull/99)
- [*BREAKING*] `min_specilization`, now OpenBrush can be used with `stable` toolchain: [#78](https://github.com/Brushfam/openbrush-contracts/pull/78)
- [*BREAKING*] `ZERO_ADDRESS`, now using `Option<AccountId>` instead: [#98](https://github.com/Brushfam/openbrush-contracts/pull/98)

### Changed

- [*BREAKING*] Now every field in OpenBrush's types that is not read/written directly in storage, is wrapped in `Lazy`, so all the types in OpenBrush can be considered upgradeable: [#99](https://github.com/Brushfam/openbrush-contracts/pull/99)

### Fixed
- Fixed reentrancy guard problem: [#88](https://github.com/Brushfam/openbrush-contracts/pull/88)
- Updated reentrancy example: [#108](https://github.com/Brushfam/openbrush-contracts/pull/108)

## [v4.0.0-beta.1]
## Changes

### Added
- Added new Governance feature with extensions: [#126](https://github.com/Brushfam/openbrush-contracts/pull/126) - was not under audit
- Added standard interface detection implementation: [#112](https://github.com/Brushfam/openbrush-contracts/pull/112) - was not under audit
- Added PSP22 Permit extension: [#109](https://github.com/Brushfam/openbrush-contracts/pull/109) - was not under audit
- Crypto module with some useful methods for better DX: [#109](https://github.com/Brushfam/openbrush-contracts/pull/109) - was not under audit

### Changes
- Bumped ink! version to 4.3.0: [#129](https://github.com/Brushfam/openbrush-contracts/pull/129)

## [v4.0.0]
## Changes

### Changed
- [*BREAKING*] Separated `_before_token_transfer` and `_after_token_transfer` in PSP22 to a trait `PSP22Transfer` and added 
different implementation on whether `Capped` extension is implemented: [#141](https://github.com/Brushfam/openbrush-contracts/pull/141)
- [*BREAKING*] Removed `PSP22::approve` method, because it creates a double-spending issue. We recommend using `increase_allowance` and `decrease_allowance` instead:
[#138](https://github.com/Brushfam/openbrush-contracts/pull/138)
- [*BREAKING*] Changed errors to `isNotSet` from `isZeroAddress`: [#136](https://github.com/Brushfam/openbrush-contracts/pull/136)
- Added verification for `PSP22Wrapper` for whether the account in `deposit_for` and `withdraw_to` is underlying wrapper: [#140](https://github.com/Brushfam/openbrush-contracts/pull/140)
- Added `max_supply` internal method in `PSP22`, fixed the issue that amount can be calculated wrong way when using `Capped` and `FlashLender` together: [#142](https://github.com/Brushfam/openbrush-contracts/pull/142)
- `PaymentSplitter` now emits `PaymentReceived` event in `receive` method: [#139](https://github.com/Brushfam/openbrush-contracts/pull/139)
- `_release_all` method removed from `PaymentSplitter`, since it was considered error-prone and unnecessary: [#145](https://github.com/Brushfam/openbrush-contracts/pull/145)
- Added `_flash_fee_receiver` method to `FlashLender`, fee is now sent to beneficiary instead of being burned: [#157](https://github.com/Brushfam/openbrush-contracts/pull/157)
- Added `releasable` method to `PaymentSplitter`: [#146](https://github.com/Brushfam/openbrush-contracts/pull/146)
- [*BREAKING*] Ownership now can't be transferred to `None`: [#137](https://github.com/Brushfam/openbrush-contracts/pull/137)
- Fixed validation in `macro_definition`: [#144](https://github.com/Brushfam/openbrush-contracts/pull/144)
- Use boolean values in `reentrancy_guard`: [#143](https://github.com/Brushfam/openbrush-contracts/pull/143)