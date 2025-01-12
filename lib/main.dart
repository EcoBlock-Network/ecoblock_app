import 'package:ecoblock_app/src/rust/api/node_management/operations.dart';
import 'package:ecoblock_app/src/rust/frb_generated.dart';
import 'package:flutter/material.dart';

class NodeManagementScreen extends StatefulWidget {
  const NodeManagementScreen({Key? key}) : super(key: key);

  @override
  _NodeManagementScreenState createState() => _NodeManagementScreenState();
}

class _NodeManagementScreenState extends State<NodeManagementScreen> {
  String _response = 'No actions yet.';
  List<String> _nodes = [];

  Future<void> _addNode(String address) async {
    try {
      // Appel de la fonction Rust générée
      final result = await addNode(address: address);
      final nodes = await listNodes();
      setState(() {
        _response = result;
        _nodes = nodes;
      });
    } catch (e) {
      setState(() {
        _response = "Error: ${e.toString()}";
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Node Management')),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            TextField(
              onSubmitted: _addNode,
              decoration: const InputDecoration(
                labelText: 'Enter node address',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            Text(
              _response,
              style: const TextStyle(color: Colors.blue),
            ),
            const SizedBox(height: 16),
            Expanded(
              child: ListView.builder(
                itemCount: _nodes.length,
                itemBuilder: (context, index) {
                  return ListTile(
                    title: Text(_nodes[index]),
                    leading: const Icon(Icons.device_hub),
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized(); // Assurez-vous que Flutter est initialisé
  await RustLib.init(); // Initialisez la bibliothèque Rust
  runApp(const MaterialApp(
    home: NodeManagementScreen(),
  ));
}