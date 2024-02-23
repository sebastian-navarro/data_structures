// Graphs

// Los grafos no son mas que otra cosa que nodos y vertices
// Los nodos son conocidos tambien como aristas (edges)
// Las vertices conocidas como arista (vertices or vertex)

// Los vertices pueden ser de una sola direccion o bidireccionales

// Ejemplos: Los mapas como google maps son realizados con grafos
// Las estaciones de un metro o de buses son representados con grafos

// Existen distintos tipos de GRAPHS

// binary tree , direct acycling graphs (dags) or anothers.

// Entonces para representarlos necesitamos una estructura y que dentro de ella
// contenga vertices y aristas (edges)

// Nuestra estructura debe aceptar tant numeros como letras por eso la mejor opcion es utilizar Generics
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Graph<VId, E= (), V= ()> {
    vertices: HashMap<VId,V>,
    adjacency: HashMap<VId, Vec<(VId, E)>>,

}

impl<VId, E, V> Graph<VId, E,V>
where 
    VId: Eq + Hash,
    V: Hash
{
        pub fn new() -> Graph<VId, E, V>{
            Graph {
                vertices: HashMap::new(), adjacency:  HashMap::new(),
            }
        }

        pub fn push_vertex(self: &mut Graph<VId,E,V>, vid: VId,vertex: V){
            self.vertices.insert(vid,vertex);
        }

        pub fn push_edge(self: &mut Self, from: VId, to: VId, edge: E ) {
            let adjacent_to_from = self.adjacency.entry(from).or_default();
            adjacent_to_from.push((to, edge));
        }

    }

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}


fn main() {
    let mut graph: Graph<&str, Direction> = Graph::new();
    graph.push_vertex("A", ());
    graph.push_vertex("E", ());
    graph.push_vertex("B", ());
    graph.push_vertex("D", ());
    graph.push_edge("A", "B", Direction::Right );
    graph.push_edge("B", "E", Direction::Down );
    println!("Graph: {:?}", graph);
}
