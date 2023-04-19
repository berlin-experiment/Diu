## Flutter Installation & Simulator Setup Guide

This guide is - _should be_ - useful for MacOS, Windows and Linux Users.

<!-- ![Image Text]() -->

## Table of Contents

- [Installing Flutter and Setting Up Android Studio and iOS Simulator on MacOS](#installing-flutter-and-setting-up-android-studio-and-ios-simulator-on-macos)
  - [Step 1: Install Flutter](#step-1-install-flutter)
  - [Step 2: Set up Android Studio](#step-2-set-up-android-studio)
  - [Step 3: Set up the iOS Simulator](#step-3-set-up-the-ios-simulator)
  - [Step 4: Create a new Flutter project](#step-4-create-a-new-flutter-project)
  - [Step 5: Run the app on the Android emulator or iOS simulator](#step-5-run-the-app-on-the-android-emulator-or-ios-simulator)
- [Installing Flutter and Setting Up Android Studio and iOS Simulator on Windows](#installing-flutter-and-setting-up-android-studio-and-ios-simulator-on-windows)
  - [Step 1: Install Flutter](#step-1-install-flutter-1)
  - [Step 2: Set up Android Studio](#step-2-set-up-android-studio-1)
  - [Step 3: Set up the iOS Simulator](#step-3-set-up-the-ios-simulator-1)
  - [Step 4: Create a new Flutter project](#step-4-create-a-new-flutter-project-1)
  - [Step 5: Run the app on the Android emulator](#step-5-run-the-app-on-the-android-emulator)
- [Installing Flutter and Setting Up Android Studio and iOS Simulator on Linux](#installing-flutter-and-setting-up-android-studio-and-ios-simulator-on-linux)
  - [Step 1: Install Flutter](#step-1-install-flutter-2)
  - [Step 2: Set up Android Studio](#step-2-set-up-android-studio-2)
  - [Step 3: Set up the iOS Simulator](#step-3-set-up-the-ios-simulator-2)
  - [Step 4: Create a new Flutter project](#step-4-create-a-new-flutter-project-2)
  - [Step 5: Run the app on the Android emulator](#step-5-run-the-app-on-the-android-emulator-1)

---

# Installing Flutter and Setting Up Android Studio and iOS Simulator on MacOS

This guide will walk you through the process of installing Flutter on your Mac and preparing Android Studio and the iOS simulator for use with Flutter projects.

### Step 1: Install Flutter

1. Go to the [Flutter website](https://flutter.dev/docs/get-started/install/macos) and download the Flutter SDK for macOS.
2. Extract the tar file to a desired location on your computer.
3. Add the Flutter SDK's `bin` directory path to your system's environment variable `Path`.

### Step 2: Set up Android Studio

1. Download and install [Android Studio](https://developer.android.com/studio).
2. Open Android Studio and click on "Configure" in the welcome screen.
3. user: please continue
   assistant: 3. Click on "SDK Manager" and navigate to the "SDK Platforms" tab.
4. Check the box next to "Show Package Details".
5. Expand the "Android 11.0 (R)" or later folder and make sure that the following items are checked:
   - Android SDK Platform 30 (or later)
   - Intel x86 Atom_64 System Image (or later)
   - Google APIs Intel x86 Atom_64 System Image (or later)
6. Click on the "SDK Tools" tab and make sure that the following items are checked:
   - Android SDK Build-Tools 30.0.3 (or later)
   - Android Emulator (or later)
   - Android SDK Platform-Tools (or later)
   - Android SDK Tools (or later)
   - Google Play Services (or later)
   - Google USB Driver (or later)
7. Click "Apply" to install the selected packages.

### Step 3: Set up the iOS Simulator

1. Install Xcode from the [Mac App Store](https://itunes.apple.com/us/app/xcode/id497799835?mt=12).
2. Open Xcode and go to "Preferences".
3. Click on the "Components" tab and make sure that the following items are installed:
   - iOS simulator
   - Command Line Tools
4. Close Xcode.

### Step 4: Create a new Flutter project

1. Open Android Studio.
2. Click on "Create New Flutter Project" in the welcome screen.
3. Choose the Flutter application template and click "Next".
4. Enter the project name, project location, and other information as needed.
5. Choose your desired Android and iOS minimum SDK versions.
6. Choose your desired project organization and click "Finish".
7. Wait for the project to be created.

### Step 5: Run the app on the Android emulator or iOS simulator

1. In Android Studio, select your Flutter project from the project selector dropdown.
2. Click on the "Run" button in the toolbar, or press the "Run" button in the Run menu.
3. Choose either an Android emulator or an iOS simulator from the device selector window.
4. Click "OK" to launch the emulator/simulator.
5. Wait for the emulator/simulator to launch.
6. Once the emulator/simulator is running, the app should automatically launch and run on the device.

Congratulations! You have successfully installed Flutter and set up Android Studio and the iOS simulator for use with Flutter projects on macOS.

---

# Installing Flutter and Setting Up Android Studio and iOS Simulator on Windows

This guide will walk you through the process of installing Flutter on your Windows computer and preparing Android Studio and the iOS simulator for use with Flutter projects.

### Step 1: Install Flutter

1. Go to the [Flutter website](https://flutter.dev/docs/get-started/install/windows) and download the Flutter SDK for Windows.
2. Extract the zip file to a desired location on your computer.
3. Add the Flutter SDK's `bin` directory path to your system's environment variable `Path`.

### Step 2: Set up Android Studio

1. Download and install [Android Studio](https://developer.android.com/studio).
2. Open Android Studio and click on "Configure" in the welcome screen.
3. Click on "SDK Manager" and navigate to the "SDK Platforms" tab.
4. Check the box next to "Show Package Details".
5. Expand the "Android 11.0 (R)" or later folder and make sure that the following items are checked:
   - Android SDK Platform 30 (or later)
   - Intel x86 Atom_64 System Image (or later)
   - Google APIs Intel x86 Atom_64 System Image (or later)
6. Click on the "SDK Tools" tab and make sure that the following items are checked:
   - Android SDK Build-Tools 30.0.3 (or later)
   - Android Emulator (or later)
   - Android SDK Platform-Tools (or later)
   - Android SDK Tools (or later)
   - Google Play Services (or later)
   - Google USB Driver (or later)
7. Click "Apply" to install the selected packages.

### Step 3: Set up the iOS Simulator

1. You cannot use the iOS simulator on Windows.

### Step 4: Create a new Flutter project

1. Open Android Studio.
2. Click on "Create New Flutter Project" in the welcome screen.
3. Choose the Flutter application template and click "Next".
4. Enter the project name, project location, and other information as needed.
5. Choose your desired Android minimum SDK version.
6. Choose your desired project organization and click "Finish".
7. Wait for the project to be created.

### Step 5: Run the app on the Android emulator

1. In Android Studio, select your Flutter project from the project selector dropdown.
2. Click on the "Run" button in the toolbar, or press the "Run" button in the Run menu.
3. Choose an Android emulator from the device selector window.
4. Click "OK" to launch the emulator.
5. Wait for the emulator to launch.
6. Once the emulator is running, the app should automatically launch and run on the device.

Congratulations! You have successfully installed Flutter and set up Android Studio for use with Flutter projects on Windows.

---

# Installing Flutter and Setting Up Android Studio and iOS Simulator on Linux

This guide will walk you through the process of installing Flutter on your Linux computer and preparing Android Studio and the iOS simulator for use with Flutter projects.

#### Step 1: Install Flutter

1. Go to the [Flutter website](https://flutter.dev/docs/get-started/install/linux) and download the Flutter SDK for Linux.
2. Extract the tar file to a desired location on your computer.
3. Add the Flutter SDK's `bin` directory path to your system's environment variable `Path`.

#### Step 2: Set up Android Studio

1. Download and install [Android Studio](https://developer.android.com/studio).
2. Open Android Studio and click on "Configure" in the welcome screen.
3. Click on "SDK Manager" and navigate to the "SDK Platforms" tab.
4. Check the box next to "Show Package Details".
5. Expand the "Android 11.0 (R)" or later folder and make sure that the following items are checked:
   - Android SDK Platform 30 (or later)
   - Intel x86 Atom_64 System Image (or later)
   - Google APIs Intel x86 Atom_64 System Image (or later)
6. Click on the "SDK Tools" tab and make sure that the following items are checked:
   - Android SDK Build-Tools 30.0.3 (or later)
   - Android Emulator (or later)
   - Android SDK Platform-Tools (or later)
   - Android SDK Tools (or later)
   - Google Play Services (or later)
   - Google USB Driver (or later)
7. Click "Apply" to install the selected packages.

#### Step 3: Set up the iOS Simulator

1. You cannot use the iOS simulator on Linux.

#### Step 4: Create a new Flutter project

1. Open Android Studio.
2. Click on "Create New Flutter Project" in the welcome screen.
3. Choose the Flutter application template and click "Next".
4. Enter the project name, project location, and other information as needed.
5. Choose your desired Android minimum SDK version.
6. Choose your desired project organization and click "Finish".
7. Wait for the project to be created.

#### Step 5: Run the app on the Android emulator

1. In Android Studio, select your Flutter project from the project selector dropdown.
2. Click on the "Run" button in the toolbar, or press the "Run" button in the Run menu.
3. Choose an Android emulator from the device selector window.
4. Click "OK" to launch the emulator.
5. Wait for the emulator to launch.
6. Once the emulator is running, the app should automatically launch and run on the device.

Congratulations! You have successfully installed Flutter and set up Android Studio for use with Flutter projects on Linux.

---

### Hope this helped.
