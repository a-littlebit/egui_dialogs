# egui_dialogs changelog

All notable changes to this crate will be documented in this file.

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