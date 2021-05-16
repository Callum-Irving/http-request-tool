use petgraph::graph::Graph;

pub fn init_ui_graph() -> Graph<usize, usize> {
    let mut graph = Graph::<usize, usize>::new();
    let mut indices = vec![];

    // INDICES:
    // 0 - TABS
    // 1 - REQUEST BLOCK
    // 2 - REPONSE BLOCK
    // 3 - ENDPOINT ENTRY (INSIDE REQUEST BLOCK)
    // 4 - BODY/HEADER/QUERY TABS (INSIDE REQUEST BLOCK)
    // 5 - PARAGRAPH ENTRY (INSIDE REQUEST BLOCK)
    // 6 - METHOD SELECT (INSIDE REQUEST BLOCK)
    // 7 - SEND REQUEST BUTTON (INSIDE REQUEST BLOCK)
    for i in 0..8 {
        indices.push(graph.add_node(i));
    }

    // WEIGHTS REPRESENT DIRECTION
    // 1 - LEFT
    // 2 - RIGHT
    // 3 - UP
    // 4 - DOWN
    // 5 - IN
    // 6 - OUT

    // Tabs
    graph.add_edge(indices[0], indices[1], 4);
    // Request
    graph.add_edge(indices[1], indices[0], 3);
    graph.add_edge(indices[1], indices[2], 2);
    graph.add_edge(indices[1], indices[3], 5);
    // Response
    graph.add_edge(indices[2], indices[0], 3);
    graph.add_edge(indices[2], indices[1], 1);
    // Endpoint entry
    graph.add_edge(indices[3], indices[1], 6);
    graph.add_edge(indices[3], indices[4], 4);
    graph.add_edge(indices[3], indices[6], 2);
    // Body/header/query select
    graph.add_edge(indices[4], indices[1], 6);
    graph.add_edge(indices[4], indices[3], 3);
    graph.add_edge(indices[4], indices[5], 4);
    // Paragraph entry
    graph.add_edge(indices[5], indices[1], 6);
    graph.add_edge(indices[5], indices[4], 3);
    // Method select
    graph.add_edge(indices[6], indices[3], 1);
    graph.add_edge(indices[6], indices[7], 2);
    graph.add_edge(indices[6], indices[1], 6);
    // Send request button
    graph.add_edge(indices[7], indices[6], 1);
    graph.add_edge(indices[7], indices[4], 4);
    graph.add_edge(indices[7], indices[1], 6);

    graph
}
