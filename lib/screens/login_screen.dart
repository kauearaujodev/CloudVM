import 'package:flutter/material.dart';
import 'home_screen.dart';

class LoginScreen extends StatefulWidget {
  const LoginScreen({super.key});

  @override
  State<LoginScreen> createState() => _LoginScreenState();
}

class _LoginScreenState extends State<LoginScreen> {

  final TextEditingController nomeController =
      TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xff050505),

      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(25),

          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,

            children: [

              const Icon(
                Icons.cloud,
                color: Colors.blue,
                size: 90,
              ),

              const SizedBox(height: 20),

              const Text(
                "CloudVM",

                style: TextStyle(
                  color: Colors.white,
                  fontSize: 35,
                  fontWeight: FontWeight.bold,
                ),
              ),

              const SizedBox(height: 40),

              TextField(
                controller: nomeController,

                style: const TextStyle(
                  color: Colors.white,
                ),

                decoration: InputDecoration(
                  hintText: "Nome de usuário",

                  hintStyle:
                      const TextStyle(
                    color: Colors.grey,
                  ),

                  filled: true,
                  fillColor: Colors.black54,

                  border: OutlineInputBorder(
                    borderRadius:
                        BorderRadius.circular(15),
                  ),
                ),
              ),

              const SizedBox(height: 30),

              SizedBox(
                width: double.infinity,
                height: 55,

                child: ElevatedButton(

                  onPressed: () {

                    Navigator.pushReplacement(
                      context,

                      MaterialPageRoute(
                        builder: (_) =>
                            const HomeScreen(),
                      ),
                    );

                  },

                  child: const Text(
                    "Entrar",
                    style: TextStyle(
                      fontSize: 18,
                    ),
                  ),

                ),
              ),

            ],
          ),
        ),
      ),
    );
  }
}
