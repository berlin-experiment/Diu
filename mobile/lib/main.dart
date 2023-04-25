import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'themes/custom_theme.dart';
import 'start_pg.dart';
import 'ble.dart';

import 'control_pg.dart';

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
      theme: customLightTheme, // set the light theme
      darkTheme: customDarkTheme, // set the dark theme
      themeMode: ThemeMode.system,
      title: 'Diu',
      home: ControlPage(),
      // home: TimePicker(),
    );
  }
}
