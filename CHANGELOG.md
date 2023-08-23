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