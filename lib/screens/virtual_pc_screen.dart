
import 'package:flutter/material.dart';

class VirtualPCScreen extends StatelessWidget {

  final Map<String, dynamic> vm;

  const VirtualPCScreen({
    super.key,
    required this.vm,
  });


  @override
  Widget build(BuildContext context) {

    return Scaffold(

      backgroundColor: Colors.black,

      appBar: AppBar(

        backgroundColor: Colors.black,

        title: Text(
          vm["nome"],
        ),

        actions: [

          IconButton(

            icon:
                const Icon(
                  Icons.power_settings_new,
                ),

            onPressed: () {

              Navigator.pop(context);

            },

          ),

        ],

      ),


      body: Container(

        decoration: const BoxDecoration(

          gradient: LinearGradient(

            colors: [

              Color(0xff001122),
              Color(0xff000000),

            ],

            begin:
                Alignment.topCenter,

            end:
                Alignment.bottomCenter,

          ),

        ),


        child: Column(

          children: [


            const SizedBox(
              height: 30,
            ),



            Text(

              "CloudVM Desktop",

              style:
                  const TextStyle(

                color:
                    Colors.white,

                fontSize:
                    30,

                fontWeight:
                    FontWeight.bold,

              ),

            ),



            const SizedBox(
              height: 40,
            ),



            Expanded(

              child:
              GridView.count(

                crossAxisCount:
                    3,


                padding:
                    const EdgeInsets.all(20),


                children: [


                  appIcon(
                    Icons.folder,
                    "Arquivos",
                  ),


                  appIcon(
                    Icons.web,
                    "Navegador",
                  ),


                  appIcon(
                    Icons.settings,
                    "Configurações",
                  ),


                  appIcon(
                    Icons.download,
                    "Downloads",
                  ),


                  appIcon(
                    Icons.terminal,
                    "Terminal",
                  ),


                  appIcon(
                    Icons.apps,
                    "Aplicativos",
                  ),


                ],

              ),

            ),



            Container(

              height:
                  60,

              color:
                  Colors.black54,


              child:
                  Center(

                child:
                    Text(

                  "${vm["sistema"]}  •  ${vm["ram"]}GB RAM  •  ${vm["armazenamento"]}GB",

                  style:
                      const TextStyle(

                    color:
                        Colors.white70,

                  ),

                ),

              ),

            ),


          ],

        ),

      ),

    );

  }



  Widget appIcon(
      IconData icon,
      String nome,
      ) {

    return Column(

      children: [

        CircleAvatar(

          radius:
              30,

          backgroundColor:
              Colors.blue,

          child:
              Icon(

            icon,

            color:
                Colors.white,

          ),

        ),


        const SizedBox(
          height: 8,
        ),


        Text(

          nome,

          style:
              const TextStyle(

            color:
                Colors.white,

          ),

        ),

      ],

    );

  }

}
