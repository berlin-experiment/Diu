import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';

// Define the class that extends ChangeNotifier
class BleService extends ChangeNotifier {
  BleService() {
    init();
  }

  Future<void> init() async {
    isOn = await _flutterBlue.isOn;
    isAvailable = await _flutterBlue.isAvailable;

    notifyListeners();

    // Start scanning
    if (!isOn) {
      _flutterBlue.turnOn();
    }

    startScan();
  }

  void startScan() {
    stopScan();
    _scanResults = [];
    // Start scanning
    if (isOn && isAvailable) {
      _flutterBlue.startScan(timeout: Duration(seconds: 6));
    }
    // Listen to scan results
    _flutterBlue.scanResults.listen((results) {
      log(results.length.toString());

      results.removeWhere((element) => element.device.name.isEmpty);
      results.retainWhere((element) =>
          element.device.name.toString().toLowerCase().contains("diu"));
      // do something with scan results
      _scanResults = results;
      notifyListeners();
    });
  }

  void stopScan() {
    _flutterBlue.stopScan();
  }

  Future<void> connectToDevice(ScanResult result) async {
    await disconnect();
    _currentDevice = result;
    await _currentDevice.device.connect();
    notifyListeners();
  }

  Future<void> disconnect() async {
    if (isConnected) {
      await _currentDevice.device.disconnect();
      isConnected = false;
      notifyListeners();
    }
  }

  late bool isOn = false;
  late bool isAvailable = false;
  late bool isConnected = false;
  final FlutterBluePlus _flutterBlue = FlutterBluePlus.instance;
  List<ScanResult> _scanResults = [];
  List<ScanResult> get scanResults => _scanResults;
  late ScanResult _currentDevice;
}
