use petgraph::graph::Graph;

pub mod pane_identifiers {
    pub const PANE_TABS: usize = 0;
    pub const PANE_ENDPOINT: usize = 1;
    pub const PANE_BODY_HEADER_SELECT: usize = 2;
    pub const PANE_REQUEST_ENTRY: usize = 3;
    pub const PANE_METHOD_SELECT: usize = 4;
    pub const PANE_SEND_BUTTON: usize = 5;
    pub const PANE_RESPONSE_TABS: usize = 6;
    pub const PANE_RESPONSE_TEXT: usize = 7;
}

pub fn init_ui_graph() -> Graph<usize, usize> {
    let mut graph = Graph::<usize, usize>::new();
    let mut indices = vec![];

    // INDICES:
    // 0 - TABS
    // 1 - ENDPOINT ENTRY
    // 2 - BODY/HEADER/QUERY TABS
    // 3 - PARAGRAPH ENTRY
    // 4 - METHOD SELECT
    // 5 - SEND REQUEST BUTTON
    // 6 - RESPONSE TABS
    // 7 - RESPONSE PARAGRAPH
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
    // Endpoint entry
    graph.add_edge(indices[1], indices[0], 3);
    graph.add_edge(indices[1], indices[2], 4);
    graph.add_edge(indices[1], indices[6], 2);
    // Body/header/query tabs
    graph.add_edge(indices[2], indices[1], 3);
    graph.add_edge(indices[2], indices[7], 2);
    graph.add_edge(indices[2], indices[3], 4);
    // Paragraph entry
    graph.add_edge(indices[3], indices[2], 3);
    graph.add_edge(indices[3], indices[7], 2);
    graph.add_edge(indices[3], indices[4], 4);
    // Method select
    graph.add_edge(indices[4], indices[3], 3);
    graph.add_edge(indices[4], indices[5], 2);
    // Send button
    graph.add_edge(indices[5], indices[4], 1);
    graph.add_edge(indices[5], indices[3], 3);
    graph.add_edge(indices[5], indices[7], 2);
    // Response tabs
    graph.add_edge(indices[6], indices[0], 3);
    graph.add_edge(indices[6], indices[1], 1);
    graph.add_edge(indices[6], indices[7], 4);
    // Response paragraph
    graph.add_edge(indices[7], indices[6], 3);
    graph.add_edge(indices[7], indices[3], 1);

    graph
}
