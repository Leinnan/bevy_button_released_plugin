# Changelog

## [0.8.0]

### Added

- `GameButton` component requires `Interaction` component. Even before that release it was the case, but now it is embedded in code.

### Changed

- Updated to Bevy 0.15.
- Put `ButtonReleasedEvent` behind the `global_event` feature that is not enabled by default.

## [0.7.0]

### Added

- `OnButtonReleased` event that works with observer pattern.

### Changed

- Updated sample code.
- Deprecate `ButtonReleasedEvent`.

## [0.6.0]

### Changed

- Updated to Bevy 0.14

## [0.5.1]

### Added

- Support for touch input.

## [0.5]

### Changed

- Updated to Bevy 0.13

## [0.4]

### Added

- `auto_add` default feature- `GameButton` struct gets added automatically to entities with `Button`

## [0.3]

### Changed

- Updated to Bevy 0.12

## [0.2]

### Changed

- Updated to Bevy 0.11

## [0.1]

- Initial version
