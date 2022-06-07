#[cfg(test)]
mod test {
    use ruatom::error::RuatomError;
    use ruatom::graph::{Edge, Graph};

    fn create_graph() -> Graph<&'static str, &'static str> {
        let mut g = Graph::new();
        g.add_vertex(0, "C").unwrap();
        g.add_vertex(1, "H").unwrap();
        g.add_vertex(2, "O").unwrap();
        g.add_vertex(3, "N").unwrap();
        g.add_edge(0, 2, "Double").unwrap();
        g.add_edge(0, 1, "Double").unwrap();
        g.add_edge(2, 3, "Double").unwrap();
        g.add_edge(1, 3, "Double").unwrap();
        g
    }
    #[test]
    fn test_add_vertex() {
        let mut g = Graph::new();
        g.add_vertex(0, "C").unwrap();
        assert_eq!(g.add_vertex(0, "H"), Err(RuatomError::ExistedVertex(0)));
        g.add_vertex(2, "O").unwrap();
        g.add_edge(0, 2, "Single").unwrap();
    }

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new();
        g.add_vertex(0, "C").unwrap();
        g.add_vertex(1, "H").unwrap();
        g.add_vertex(2, "O").unwrap();
        g.add_vertex(3, "N").unwrap();
        g.add_edge(0, 2, "Double").unwrap();
        assert_eq!(
            g.add_edge(0, 2, "Double"),
            Err(RuatomError::ExistedEdge(0, 2))
        );
        assert_eq!(
            g.add_edge(4, 2, "Double"),
            Err(RuatomError::NoSuchVertex(4))
        );
        g.add_edge(3, 2, "Double").unwrap();
    }

    #[test]
    fn test_adjancent() {
        let g = create_graph();
        assert!(g.adjancent(0, 2));
        assert!(g.adjancent(2, 0));
        assert!(!g.adjancent(1, 2));
        assert!(!g.adjancent(1, 4));
    }

    #[test]
    fn test_out_neightors() {
        let g = create_graph();
        if let Err(e) = g.out_neighbors(&4) {
            assert!(e == RuatomError::NoSuchVertex(4));
        };
        if let Err(e) = g.out_neighbors(&3) {
            assert!(e == RuatomError::NoSuchVertex(3));
        };
        let mut it = g.in_neighbors(&3).unwrap();
        assert_eq!(&1, it.next().unwrap());
        assert_eq!(&2, it.next().unwrap());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test_in_neightors() {
        let g = create_graph();
        if let Err(e) = g.in_neighbors(&4) {
            assert!(e == RuatomError::NoSuchVertex(4));
        };
        if let Err(e) = g.in_neighbors(&0) {
            assert!(e == RuatomError::NoSuchVertex(0));
        };
        let mut it = g.in_neighbors(&3).unwrap();
        assert_eq!(&1, it.next().unwrap());
        assert_eq!(&2, it.next().unwrap());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test_outbound_count() {
        let mut g = create_graph();
        assert_eq!(g.outbound_count(&0), Ok(2));
        assert_eq!(g.outbound_count(&3), Ok(0));
        assert!(g.add_edge(3, 1, "Double").unwrap());
        assert_eq!(g.outbound_count(&3), Ok(1));
    }

    #[test]
    fn test_inbound_count() {
        let mut g = create_graph();
        assert_eq!(g.inbound_count(&0), Ok(0));
        assert_eq!(g.inbound_count(&3), Ok(2));
        assert!(g.add_edge(3, 1, "Double").unwrap());
        assert_eq!(g.inbound_count(&3), Ok(2));
        assert_eq!(g.inbound_count(&1), Ok(2));
    }

    #[test]
    fn test_neighbors() {
        let mut g = create_graph();
        let nei = g.neighbors(&0).unwrap();
        assert_eq!(vec![1, 2], nei.collect::<Vec<u8>>());
        g.add_edge(3, 0, "Double").unwrap();
        let nei = g.neighbors(&0).unwrap();
        assert_eq!(vec![1, 2, 3], nei.collect::<Vec<u8>>());
        let nei = g.neighbors(&2).unwrap();
        assert_eq!(vec![0, 3], nei.collect::<Vec<u8>>());
        if let Err(e) = g.neighbors(&4) {
            assert!(e == RuatomError::NoSuchVertex(4));
        };
    }

    #[test]
    fn test_map_edge() {
        let mut g = create_graph();
        assert_eq!(
            g.map_edge(&4, |_desc, _v| {}),
            Err(RuatomError::NoSuchVertex(4))
        );
        let mut edges = vec![];
        g.map_edge(&0, |_, v| {
            edges.push(*v);
        })
        .unwrap();
        assert_eq!(edges, vec![1, 2]);
        g.add_edge(3, 0, "D").unwrap();
        let mut edges = vec![];
        g.map_edge(&0, |_, v| {
            edges.push(*v);
        })
        .unwrap();
        edges.sort();
        assert_eq!(edges, vec![1, 2, 3]);
    }

    #[test]
    fn test_map_vertex() {
        let mut g = create_graph();
        assert_eq!(g.map_vertex(&4, |_| {}), Err(RuatomError::NoSuchVertex(4)));
        let mut edges = vec![];
        g.map_vertex(&0, |v| {
            edges.push(*v);
        })
        .unwrap();
        assert_eq!(edges, vec!["H", "O"]);
        g.add_edge(3, 0, "D").unwrap();
        let mut edges = vec![];
        g.map_vertex(&0, |v| {
            edges.push(*v);
        })
        .unwrap();
        assert_eq!(edges, vec!["H", "O", "N"]);
    }

    #[test]
    fn test_vertex() {
        let mut g = create_graph();
        assert_eq!(g.vertex(&0).unwrap(), &"C");
        assert_eq!(g.vertex(&4), Err(RuatomError::NoSuchVertex(4)));
        g.add_vertex(4, "S").unwrap();
        assert_eq!(g.vertex(&4), Ok(&"S"));
    }

    #[test]
    fn test_update_vertex() {
        let mut g = create_graph();
        assert_eq!(g.vertex(&0).unwrap(), &"C");
        g.update_vertex(0, "P");
        assert_eq!(g.vertex(&0).unwrap(), &"P");
    }

    #[test]
    fn test_vertex_mut() {
        let mut g = Graph::new();
        g.add_vertex(0, String::from("C")).unwrap();
        g.add_vertex(2, String::from("O")).unwrap();
        g.add_edge(0, 2, "Double").unwrap();
        let v = g.vertex_mut(&0).unwrap();
        v.push('H');
        assert_eq!(g.vertex(&0).unwrap(), &String::from("CH"));
    }

    #[test]
    fn test_edge() {
        let mut g = create_graph();
        assert_eq!(g.vertex(&0).unwrap(), &"C");
        assert_eq!(g.vertex(&4), Err(RuatomError::NoSuchVertex(4)));
        g.add_vertex(4, "S").unwrap();
        assert_eq!(g.vertex(&4), Ok(&"S"));
    }

    #[test]
    fn test_edge_mut() {
        let mut g = Graph::new();
        g.add_vertex(0, String::from("C")).unwrap();
        g.add_vertex(2, String::from("O")).unwrap();
        g.add_edge(0, 2, String::from("D")).unwrap();
        let e = Edge::new(0, 2);
        let v = g.edge_mut(&e).unwrap();
        v.push('b');
        assert_eq!(g.edge(&e).unwrap(), &String::from("Db"));

        let ne = Edge::new(0, 4);
        assert_eq!(g.edge_mut(&ne), Err(RuatomError::NoSuchEdge(4, 0)));
    }

    #[test]
    fn test_edge_with_vertex() {
        let mut g = Graph::new();
        g.add_vertex(0, String::from("C")).unwrap();
        g.add_vertex(2, String::from("O")).unwrap();
        g.add_edge(0, 2, String::from("D")).unwrap();
        assert_eq!(g.edge_with_vertex(0, 2).unwrap(), &String::from("D"));
        assert_eq!(g.edge_with_vertex(0, 4), Err(RuatomError::NoSuchEdge(4, 0)));
    }

    #[test]
    fn test_replace() {
        let mut g = create_graph();
        let e = Edge::new(0, 2);
        g.replace(e, "Si");
        assert_eq!(g.edge_with_vertex(0, 2).unwrap(), &"Si");
    }
}
