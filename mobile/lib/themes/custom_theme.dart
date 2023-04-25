import 'package:flutter/material.dart';

final customLightTheme = ThemeData(
  brightness: Brightness.light,
  primaryColor: Colors.blueGrey,
  fontFamily: 'Raleway',
  textTheme: TextTheme(
    displayLarge: TextStyle(
      fontFamily: 'Raleway',
      fontSize: 24.0,
      fontWeight: FontWeight.bold,
      color: Colors.grey[800],
    ),
    bodyLarge: TextStyle(
      fontFamily: 'Raleway',
      fontSize: 16.0,
      color: Colors.grey[800],
    ),
  ),
  colorScheme: const ColorScheme.light(
    primary: Colors.grey,
    onPrimary: Colors.grey,
    secondary: Colors.grey,
    onSecondary: Colors.white,
    surface: Colors.white,
    onSurface: Colors.grey,
  ).copyWith(secondary: Colors.grey),
  appBarTheme: const AppBarTheme(
    elevation: 0,
    backgroundColor: Colors.transparent,
  ),
  elevatedButtonTheme: ElevatedButtonThemeData(
    style: ElevatedButton.styleFrom(
      elevation: 0,
      backgroundColor: Colors.transparent,
      shape: RoundedRectangleBorder(
        side: const BorderSide(
          color: Colors.grey,
          width: 1.0,
        ),
        borderRadius: BorderRadius.circular(8.0),
      ),
    ),
  ),
);

final customDarkTheme = ThemeData(
  brightness: Brightness.dark,
  primaryColor: Colors.grey,
  fontFamily: 'Raleway',
  textTheme: const TextTheme(
    displayLarge: TextStyle(
      fontFamily: 'Raleway',
      fontSize: 24.0,
      fontWeight: FontWeight.bold,
      color: Colors.blue,
    ),
    bodyLarge: TextStyle(
      fontFamily: 'Raleway',
      fontSize: 16.0,
      color: Colors.blue,
    ),
  ),
  colorScheme: const ColorScheme.dark(
    primary: Colors.blueGrey,
    onPrimary: Colors.blueGrey,
    secondary: Colors.blueGrey,
    onSecondary: Colors.grey,
    surface: Colors.grey,
    onSurface: Colors.blueGrey,
  ).copyWith(secondary: Colors.blueGrey),
  appBarTheme: const AppBarTheme(
    elevation: 0,
    backgroundColor: Colors.transparent,
  ),
  elevatedButtonTheme: ElevatedButtonThemeData(
    style: ElevatedButton.styleFrom(
      elevation: 0,
      backgroundColor: Colors.transparent,
      shape: RoundedRectangleBorder(
        side: const BorderSide(
          color: Colors.blueGrey,
          width: 1.0,
        ),
        borderRadius: BorderRadius.circular(8.0),
      ),
    ),
  ),
);
