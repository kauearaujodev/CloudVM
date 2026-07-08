import 'package:flutter/material.dart';
import 'create_vm_screen.dart';
import 'virtual_pc_screen.dart';

class HomeScreen extends StatefulWidget {
const HomeScreen({super.key});

@override
State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {

final List<Map<String, dynamic>> vmList = [];

final TextEditingController searchController =
TextEditingController();

List<Map<String, dynamic>> get vmFiltradas {

if (searchController.text.isEmpty) {  
  return vmList;  
}  

return vmList.where((vm) {  

  return vm["nome"]  
      .toString()  
      .toLowerCase()  
      .contains(  
        searchController.text.toLowerCase(),  
      );  

}).toList();

}

Future<void> abrirCriarVM() async {

final resultado = await Navigator.push(  
  context,  
  MaterialPageRoute(  
    builder: (_) => const CreateVMScreen(),  
  ),  
);  


if (resultado != null) {  


  if (vmList.length >= 10) {  

    ScaffoldMessenger.of(context).showSnackBar(  

      const SnackBar(  
        content: Text(  
          "Limite de 10 VMs atingido.",  
        ),  
      ),  

    );  

    return;  

  }  



  setState(() {  

    vmList.add({  

      "nome": resultado["nome"],  

      "sistema": resultado["sistema"],  

      "ram": resultado["ram"],  

      "armazenamento":  
          resultado["armazenamento"],  

      "criadaEm":  
          DateTime.now(),  

    });  

  });  


}

}

@override
Widget build(BuildContext context) {

return Scaffold(  

  backgroundColor:  
      const Color(0xff050505),  



  appBar: AppBar(  

    backgroundColor:  
        Colors.transparent,  

    elevation: 0,  


    title: const Text(  

      "CloudVM",  

      style: TextStyle(  
        fontWeight:  
            FontWeight.bold,  
      ),  

    ),  

  ),  



  floatingActionButton:  
      FloatingActionButton.extended(  

    backgroundColor:  
        Colors.blue,  


    onPressed:  
        abrirCriarVM,  


    icon:  
        const Icon(Icons.add),  


    label:  
        const Text(  
          "Criar VM",  
        ),  

  ),  



  body: Padding(  

    padding:  
        const EdgeInsets.all(15),  



    child: Column(  

      children: [  


        TextField(  

          controller:  
              searchController,  


          onChanged: (value) {  

            setState(() {});  

          },  


          style:  
              const TextStyle(  
                color: Colors.white,  
              ),  



          decoration:  
              InputDecoration(  

            hintText:  
                "Procurar VM",  


            prefixIcon:  
                const Icon(  
                  Icons.search,  
                ),  


            filled:  
                true,  


            fillColor:  
                Colors.black54,  


            border:  
                OutlineInputBorder(  

              borderRadius:  
                  BorderRadius.circular(15),  

            ),  

          ),  

        ),  



        const SizedBox(  
          height: 20,  
        ),  




        Expanded(  

          child:  

          vmFiltradas.isEmpty  


          ?  

          const Center(  

            child: Text(  

              "Nenhuma VM criada.",  


              style:  
                  TextStyle(  

                color:  
                    Colors.white54,  


                fontSize:  
                    18,  

              ),  

            ),  

          )  



          :  



          ListView.builder(  

            itemCount:  
                vmFiltradas.length,  



            itemBuilder:  
                (context,index){  


              final vm =  
                  vmFiltradas[index];  



              final pronta =  
                  DateTime.now()  
                  .difference(  
                    vm["criadaEm"],  
                  )  
                  .inHours >= 24;  



              return GestureDetector(  

                onTap: () {  


                  Navigator.push(  

                    context,  


                    MaterialPageRoute(  

                      builder: (_) =>  

                          VirtualPCScreen(  

                            vm: vm,  

                          ),  

                    ),  

                  );  


                },  



                child: Card(  

                  color:  
                      const Color(0xff111111),  


                  margin:  
                      const EdgeInsets.only(  
                        bottom: 15,  
                      ),  



                  child:  
                  Padding(  

                    padding:  
                        const EdgeInsets.all(15),  



                    child:  
                    Column(  

                      crossAxisAlignment:  
                          CrossAxisAlignment.start,  


                      children: [  



                        Text(  

                          vm["nome"],  


                          style:  
                              const TextStyle(  

                            color:  
                                Colors.white,  


                            fontSize:  
                                22,  


                            fontWeight:  
                                FontWeight.bold,  

                          ),  

                        ),  




                        Text(  

                          "Sistema: ${vm["sistema"]}",  


                          style:  
                              const TextStyle(  

                            color:  
                                Colors.white70,  

                          ),  

                        ),  



                        Text(  

                          "RAM: ${vm["ram"]} GB",  


                          style:  
                              const TextStyle(  

                            color:  
                                Colors.white70,  

                          ),  

                        ),  




                        Text(  

                          "Armazenamento: ${vm["armazenamento"]} GB",  


                          style:  
                              const TextStyle(  

                            color:  
                                Colors.white70,  

                          ),  

                        ),  




                        const SizedBox(  
                          height: 15,  
                        ),  




                        Row(  

                          children: [  


                            Icon(  

                              pronta  

                              ? Icons.check_circle  

                              : Icons.sync,  


                              color:  

                              pronta  

                              ? Colors.green  

                              : Colors.orange,  

                            ),  



                            const SizedBox(  
                              width: 8,  
                            ),  




                            Text(  

                              pronta  

                              ? "Pronta"  

                              : "Criando...",  


                              style:  
                                  TextStyle(  

                                color:  

                                pronta  

                                ? Colors.green  

                                : Colors.orange,  


                                fontWeight:  
                                    FontWeight.bold,  

                              ),  

                            ),  


                          ],  

                        ),  




                        const SizedBox(  
                          height: 10,  
                        ),  




                        LinearProgressIndicator(  

                          value:  

                          pronta  

                          ? 1  

                          : 0.25,  

                        ),  



                      ],  

                    ),  

                  ),  

                ),  

              );  


            },  

          ),  

        ),  


      ],  

    ),  

  ),  

);

}

@override
void dispose() {

searchController.dispose();  

super.dispose();

}

}
