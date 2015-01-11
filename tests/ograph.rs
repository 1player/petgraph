extern crate petgraph;

use petgraph::{
    OGraph,
    Undirected,
    Reversed,
    BreadthFirst,
    dijkstra,
    Incoming,
    Outgoing,
};

use petgraph::ograph::{
    min_spanning_tree,
    is_cyclic,
    NodeIndex,
};

#[test]
fn undirected()
{
    let mut og = OGraph::new_undirected();
    let a = og.add_node(0);
    let b = og.add_node(1);
    let c = og.add_node(2);
    let d = og.add_node(3);
    let _ = og.add_edge(a, b, 0);
    let _ = og.add_edge(a, c, 1);
    og.add_edge(c, a, 2);
    og.add_edge(a, a, 3);
    og.add_edge(b, c, 4);
    og.add_edge(b, a, 5);
    og.add_edge(a, d, 6);
    assert_eq!(og.node_count(), 4);
    assert_eq!(og.edge_count(), 7);

    assert!(og.find_edge(a, b).is_some());
    assert!(og.find_edge(d, a).is_some());
    assert!(og.find_edge(a, a).is_some());

    for edge in og.raw_edges().iter() {
        assert!(og.find_edge(edge.source(), edge.target()).is_some());
        assert!(og.find_edge(edge.target(), edge.source()).is_some());
    }

    assert_eq!(og.neighbors(b).collect::<Vec<_>>(), vec![a, c, a]);

    og.remove_node(a);
    assert_eq!(og.neighbors(b).collect::<Vec<_>>(), vec![c]);
    assert_eq!(og.node_count(), 3);
    assert_eq!(og.edge_count(), 1);
    assert!(og.find_edge(a, b).is_none());
    assert!(og.find_edge(d, a).is_none());
    assert!(og.find_edge(a, a).is_none());
    assert!(og.find_edge(b, c).is_some());

}

#[test]
fn dfs() {
    let mut gr = OGraph::new();
    let h = gr.add_node("H");
    let i = gr.add_node("I");
    let j = gr.add_node("J");
    let k = gr.add_node("K");
    // Z is disconnected.
    let _ = gr.add_node("Z");
    gr.add_edge(h, i, 1.);
    gr.add_edge(h, j, 3.);
    gr.add_edge(i, j, 1.);
    gr.add_edge(i, k, 2.);

    let mut visited = 0is;
    petgraph::depth_first_search(&gr, h, |_| {
        visited += 1;
        true
    });
    assert_eq!(visited, 4);

    let mut visited = 0is;
    petgraph::depth_first_search(&Reversed(&gr), h, |_| {
        visited += 1;
        true
    });
    assert_eq!(visited, 1);

    let mut visited = 0is;
    petgraph::depth_first_search(&Reversed(&gr), k, |_| {
        visited += 1;
        true
    });
    assert_eq!(visited, 3);

    let mut visited = 0is;
    petgraph::depth_first_search(&gr, i, |_| {
        visited += 1;
        true
    });
    assert_eq!(visited, 3);

    let mut visited = 0is;
    petgraph::depth_first_search(&Undirected(&gr), i, |_| {
        visited += 1;
        true
    });
    assert_eq!(visited, 4);
}



#[test]
fn mst() {
    let mut gr = OGraph::<_,_>::new();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    let d = gr.add_node("D");
    let e = gr.add_node("E");
    let f = gr.add_node("F");
    let g = gr.add_node("G");
    gr.add_edge(a, b, 7.0_f32);  // closure capture below doesn't work with default float type
    gr.add_edge(a, d, 5.);
    gr.add_edge(d, b, 9.);
    gr.add_edge(b, c, 8.);
    gr.add_edge(b, e, 7.);
    gr.add_edge(c, e, 5.);
    gr.add_edge(d, e, 15.);
    gr.add_edge(d, f, 6.);
    gr.add_edge(f, e, 8.);
    gr.add_edge(f, g, 11.);
    gr.add_edge(e, g, 9.);

    // add a disjoint part
    let h = gr.add_node("H");
    let i = gr.add_node("I");
    let j = gr.add_node("J");
    gr.add_edge(h, i, 1.);
    gr.add_edge(h, j, 3.);
    gr.add_edge(i, j, 1.);

    let mst = min_spanning_tree(&gr);
    println!("MST is:\n{:?}", mst);
    assert!(mst.node_count() == gr.node_count());
    // |E| = |N| - 2  because there are two disconnected components.
    assert!(mst.edge_count() == gr.node_count() - 2);

    // check the exact edges are there
    assert!(mst.find_edge(a, b).is_some());
    assert!(mst.find_edge(a, d).is_some());
    assert!(mst.find_edge(b, e).is_some());
    assert!(mst.find_edge(e, c).is_some());
    assert!(mst.find_edge(e, g).is_some());
    assert!(mst.find_edge(d, f).is_some());

    assert!(mst.find_edge(h, i).is_some());
    assert!(mst.find_edge(i, j).is_some());
    
    assert!(mst.find_edge(d, b).is_none());
    assert!(mst.find_edge(b, c).is_none());

}

#[test]
fn selfloop() {
    let mut gr = OGraph::new();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    gr.add_edge(a, b, 7.);
    gr.add_edge(c, a, 6.);
    let sed = gr.add_edge(a, a, 2.);

    assert!(gr.find_edge(a, b).is_some());
    assert!(gr.find_edge(b, a).is_none());
    assert!(gr.find_any_edge(b, a).is_some());
    assert!(gr.find_edge(a, a).is_some());
    println!("{:?}", gr);

    gr.remove_edge(sed);
    assert!(gr.find_edge(a, a).is_none());
    println!("{:?}", gr);
}

#[test]
fn cyclic() {
    let mut gr = OGraph::new();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");

    assert!(!is_cyclic(&gr));
    gr.add_edge(a, b, 7.);
    gr.add_edge(c, a, 6.);
    assert!(!is_cyclic(&gr));
    {
        let e = gr.add_edge(a, a, 0.);
        assert!(is_cyclic(&gr));
        gr.remove_edge(e);
        assert!(!is_cyclic(&gr));
    }

    {
        let e = gr.add_edge(b, c, 0.);
        assert!(is_cyclic(&gr));
        gr.remove_edge(e);
        assert!(!is_cyclic(&gr));
    }

    let d = gr.add_node("D");
    let e = gr.add_node("E");
    gr.add_edge(b, d, 0.);
    gr.add_edge(d, e, 0.);
    assert!(!is_cyclic(&gr));
    gr.add_edge(c, e, 0.);
    assert!(is_cyclic(&gr));
}

#[test]
fn multi() {
    let mut gr = OGraph::new();
    let a = gr.add_node("a");
    let b = gr.add_node("b");
    gr.add_edge(a, b, ());
    gr.add_edge(a, b, ());
    assert_eq!(gr.edge_count(), 2);

}
#[test]
fn update_edge()
{
    {
        let mut gr = OGraph::new();
        let a = gr.add_node("a");
        let b = gr.add_node("b");
        let e = gr.update_edge(a, b, 1);
        let f = gr.update_edge(a, b, 2);
        let _ = gr.update_edge(b, a, 3);
        assert_eq!(gr.edge_count(), 2);
        assert_eq!(e, f);
        assert_eq!(*gr.edge_weight(f).unwrap(), 2);
    }

    {
        let mut gr = OGraph::new_undirected();
        let a = gr.add_node("a");
        let b = gr.add_node("b");
        let e = gr.update_edge(a, b, 1);
        let f = gr.update_edge(b, a, 2);
        assert_eq!(gr.edge_count(), 1);
        assert_eq!(e, f);
        assert_eq!(*gr.edge_weight(f).unwrap(), 2);
    }
}

#[test]
fn dijk() {
    let mut g = OGraph::new_undirected();
    let a = g.add_node("A");
    let b = g.add_node("B");
    let c = g.add_node("C");
    let d = g.add_node("D");
    let e = g.add_node("E");
    let f = g.add_node("F");
    g.add_edge(a, b, 7.0_f32);
    g.add_edge(c, a, 9.);
    g.add_edge(a, d, 14.);
    g.add_edge(b, c, 10.);
    g.add_edge(d, c, 2.);
    g.add_edge(d, e, 9.);
    g.add_edge(b, f, 15.);
    g.add_edge(c, f, 11.);
    g.add_edge(e, f, 6.);
    println!("{:?}", g);
    for no in BreadthFirst::new(&g, a) {
        println!("Visit {:?} = {:?}", no, g.node_weight(no));
    }

    let scores = dijkstra(&g, a, None, |gr, n| gr.edges(n).map(|(n, &e)| (n, e)));
    println!("Scores= {:?}", scores);
    assert_eq!(scores[c], 9.);
    assert_eq!(scores[e], 20.);
    assert_eq!(scores[f], 20.);

    let scores = dijkstra(&g, a, Some(c), |gr, n| gr.edges(n).map(|(n, &e)| (n, e)));
    assert_eq!(scores[c], 9.);
}

#[test]
fn without()
{
    let mut og = OGraph::new_undirected();
    let a = og.add_node(0);
    let b = og.add_node(1);
    let c = og.add_node(2);
    let d = og.add_node(3);
    let _ = og.add_edge(a, b, 0);
    let _ = og.add_edge(a, c, 1);
    let v: Vec<NodeIndex> = og.without_edges(Outgoing).collect();
    assert_eq!(v, vec![d]);

    let mut og = OGraph::new();
    let a = og.add_node(0);
    let b = og.add_node(1);
    let c = og.add_node(2);
    let d = og.add_node(3);
    let _ = og.add_edge(a, b, 0);
    let _ = og.add_edge(a, c, 1);
    let init: Vec<NodeIndex> = og.without_edges(Incoming).collect();
    let term: Vec<NodeIndex> = og.without_edges(Outgoing).collect();
    assert_eq!(init, vec![a, d]);
    assert_eq!(term, vec![b, c, d]);
}
