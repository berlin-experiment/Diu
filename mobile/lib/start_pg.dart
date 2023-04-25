import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';
import 'package:provider/provider.dart';
import 'ble.dart';

class StartPage extends StatefulWidget {
  const StartPage({super.key});

  @override
  State<StartPage> createState() => _StartPageState();
}

class _StartPageState extends State<StartPage> {
  @override
  Widget build(BuildContext context) {
    return Consumer<BleService>(builder: (context, ble, child) {
      void selectResult(ScanResult result) async {
        ble.connectToDevice(result).then((result) => null);
      }

      Widget createTileItem(ScanResult result) {
        return ListTile(
          onTap: () {
            selectResult(result);
          },
          title: Text(result.device.name),
          trailing: Text(
            result.rssi.toString(),
          ),
        );
      }

      return Scaffold(
        appBar: AppBar(
          title: const Text("Diu"),
        ),
        body: ListView(
          children: [
            const Text(
              "hello",
              style: TextStyle(color: Colors.black),
            ),
            Text(
              (ble.isOn && ble.isAvailable)
                  ? "BLE is Active"
                  : "Please turn on BLE",
              style: const TextStyle(color: Colors.black),
            ),
            ...ble.scanResults.map(createTileItem).toList()
          ],
        ),
      );
    });
  }
}
