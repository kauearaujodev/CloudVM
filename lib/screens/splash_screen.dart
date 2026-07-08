import 'dart:async';
import 'package:flutter/material.dart';
import 'login_screen.dart';

class SplashScreen extends StatefulWidget {
  const SplashScreen({super.key});

  @override
  State<SplashScreen> createState() => _SplashScreenState();
}

class _SplashScreenState extends State<SplashScreen> {

  @override
  void initState() {
    super.initState();

    Timer(const Duration(seconds: 3), () {
      Navigator.pushReplacement(
        context,
        MaterialPageRoute(
          builder: (_) => const LoginScreen(),
        ),
      );
    });
  }

  @override
  Widget build(BuildContext context) {

    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,

          children: [

            Container(
              padding: const EdgeInsets.all(25),

              decoration: BoxDecoration(
                shape: BoxShape.circle,

                border: Border.all(
                  color: Colors.blue,
                  width: 3,
                ),
              ),

              child: const Icon(
                Icons.cloud,
                color: Colors.blue,
                size: 90,
              ),
            ),

            const SizedBox(height: 25),

            const Text(
              "CloudVM",

              style: TextStyle(
                fontSize: 42,
                fontWeight: FontWeight.bold,
                color: Colors.white,
              ),
            ),

            const SizedBox(height: 10),

            const Text(
              "Seu PC na nuvem",

              style: TextStyle(
                fontSize: 18,
                color: Colors.blue,
              ),
            ),

            const SizedBox(height: 40),

            const CircularProgressIndicator(
              color: Colors.blue,
            ),

          ],
        ),
      ),
    );
  }
}
