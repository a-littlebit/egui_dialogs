# egui_dialogs changelog

All notable changes to this crate will be documented in this file.

## 0.3.7 - 2025-03-18

### Changed

- Updated egui to 0.31.1

## 0.3.6 - 2025-01-12

### Changed

- Updated egui to 0.30.0

## 0.3.5 - 2024-10-14

### Added

- Global min/max size options for dialogs

### Fixed

- Dialog buttons overflow without extending the dialog

## 0.3.4 - 2024-10-14

### Added

- Added min_size and max_size options to `StandardDialog` struct

### Fixed

- Fixed dialog centering not properly

## 0.3.3 - 2024-10-05

### Changed

- Renamed helper functions for mapping replies with more clear names

- `Dialogs::confirm` now takes a closure to configure the dialog

## 0.3.2 - 2024-10-02

### Added

- Added helper functions to easily map `StandardReply` to other reply types

### Changed

- `Dialogs::confirm` now takes an ID instead of a callback function

## 0.3.1 - 2024-09-30

### Changed

- `DialogDetails::on_reply` now returns a new `DialogDetails` with a reply type mapped by the reply handler

### Fixed

- Fix: Dialog coundn't show fade-out animation after closed

## 0.3.0 - 2024-09-30

### New Features

- Allow handling dialog replies without callbacks - use IDs to identify your dialogs and handle replies!

### Added

- Added: `Dialogs::is_open`, `Dialogs::add_if_absent`, `DialogDetails::show_if_absent` to show a dialog only if it's not already open

## 0.2.6 - 2024-09-28

### Updated

- Updated egui to 0.29.0

## 0.2.5 - 2024-09-25

### Fixed
- Disallow focus on background when a dialog is being shown

## Earlier

#### 2024-09-24
- Added: override all dialogs with a custom style

#### 2024-09-23
- optimize text wrap policy

#### 2024-09-22
- use `egui::WidgetText` instead of `String` for dialog texts
- Added fade-in & fade-out animations

#### 2024-09-21: 
- First release