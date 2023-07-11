# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased v4.0.0]

### Added
- [*BREAKING*] `implementation`, `override` macros: [78](https://github.com/Brushfam/openbrush-contracts/pull/78) 
- [*BREAKING*] `storage_item` macro, new OB feature: `Upgradeable`, which implements `set_code_hash` functionality [99](https://github.com/Brushfam/openbrush-contracts/pull/99)
- `UI` tests [77](https://github.com/Brushfam/openbrush-contracts/pull/77)
- `openbrush::accessors` macro for generating automatic getters/setters for storage items: [66](https://github.com/Brushfam/openbrush-contracts/pull/66) and [61](https://github.com/Brushfam/openbrush-contracts/pull/61)

### Removed
- [*BREAKING*] `min_specilization`, now openbrush is `stable`: [78](https://github.com/Brushfam/openbrush-contracts/pull/78)
- `ZERO_ADDRESS`, now using `Option<AccountId>` instead: [98](https://github.com/Brushfam/openbrush-contracts/pull/98)

### Fixed
- Fixed reentrancy guard problem: [#88](https://github.com/Brushfam/openbrush-contracts/pull/88)
