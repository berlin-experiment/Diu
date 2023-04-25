import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'start_pg.dart';
import 'ble.dart';

void main() {
  runApp(const DiuApp());
}

class DiuApp extends StatelessWidget {
  const DiuApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      builder: (context, child) {
        return MultiProvider(providers: [
          ChangeNotifierProvider(create: (context) => BleService())
        ], child: child);
      },

      debugShowCheckedModeBanner: false,
      title: 'Diu',
      home: const StartPage(),
      // home: TimePicker(),
    );
  }
}
