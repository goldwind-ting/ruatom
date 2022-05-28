use crate::{edge::Edge, error::GraphError, vertices::VertexIter};
use hashbrown::HashMap;
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Clone, Debug, Default)]
pub struct Graph<T, F> {
    vertices: HashMap<u8, T>,
    edges: HashMap<Edge, F>,
    inbound_table: HashMap<u8, Vec<u8>>,
    outbound_table: HashMap<u8, Vec<u8>>,
}

impl<T, F: Clone> Graph<T, F> {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            inbound_table: HashMap::new(),
            outbound_table: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let edges_capacity = if capacity < 100 {
            usize::pow(capacity, 2)
        } else {
            capacity
        };

        Self {
            vertices: HashMap::with_capacity(capacity),
            edges: HashMap::with_capacity(edges_capacity),
            inbound_table: HashMap::with_capacity(capacity),
            outbound_table: HashMap::with_capacity(capacity),
        }
    }

    fn has_vertex(&self, v: &u8) -> bool {
        self.vertices.get(&v).is_some()
    }

    pub fn add_vertex(&mut self, k: u8, v: T) -> Result<(), GraphError> {
        if self.has_vertex(&k) {
            return Err(GraphError::ExistedVertex(k));
        }
        self.vertices.insert(k, v);
        Ok(())
    }

    pub fn add_edge(&mut self, a: u8, b: u8, attr: F) -> Result<bool, GraphError> {
        if self.has_edge(&a, &b) {
            return Err(GraphError::ExistedEdge(a, b));
        }
        if a == b {
            return Err(GraphError::InvalidEdge(a, b));
        }
        if let Err(e) = self.do_add_edge(a, b, attr) {
            return Err(e);
        };
        return Ok(true);
    }

    fn has_edge(&self, a: &u8, b: &u8) -> bool {
        match self.outbound_table.get(a) {
            None => false,
            Some(bound) => bound.contains(b),
        }
    }

    fn do_add_edge(&mut self, a: u8, b: u8, attr: F) -> Result<(), GraphError> {
        if !self.vertices.contains_key(&a) {
            return Err(GraphError::NoSuchVertex(a));
        }
        if !self.vertices.contains_key(&b) {
            return Err(GraphError::NoSuchVertex(b));
        }
        let edge = Edge::new(a, b);
        self.edges.insert(edge, attr);
        match self.inbound_table.get_mut(&b) {
            None => {
                self.inbound_table.insert(b, vec![a]);
            }
            Some(bound) => {
                bound.push(a);
                bound.sort();
            }
        }

        match self.outbound_table.get_mut(&a) {
            None => {
                self.outbound_table.insert(a, vec![b]);
            }
            Some(bound) => {
                bound.push(b);
                bound.sort();
            }
        }
        Ok(())
    }

    pub fn outbound_count(&self, v: &u8) -> Result<usize, GraphError> {
        if !self.has_vertex(v) {
            return Err(GraphError::NoSuchVertex(*v));
        }
        self.outbound_table.get(v).map_or(Ok(0), |l| Ok(l.len()))
    }

    pub fn inbound_count(&self, v: &u8) -> Result<usize, GraphError> {
        if !self.has_vertex(v) {
            return Err(GraphError::NoSuchVertex(*v));
        }
        self.inbound_table.get(&v).map_or(Ok(0), |l| Ok(l.len()))
    }

    pub fn adjancent(&self, a: u8, b: u8) -> bool {
        let e1 = Edge::new(a, b);
        let e2 = Edge::new(b, a);
        self.edges.contains_key(&e1) || self.edges.contains_key(&e2)
    }

    pub fn in_neighbors(&self, v: &u8) -> Result<VertexIter<'_, Iter<'_, u8>>, GraphError> {
        self.inbound_table
            .get(v)
            .map_or(Err(GraphError::NoSuchVertex(*v)), |l| {
                Ok(VertexIter::new(l.iter()))
            })
    }

    pub fn out_neighbors(&self, v: &u8) -> Result<VertexIter<'_, Iter<'_, u8>>, GraphError> {
        self.outbound_table
            .get(v)
            .map_or(Err(GraphError::NoSuchVertex(*v)), |l| {
                Ok(VertexIter::new(l.iter()))
            })
    }

    pub fn neighbors(&self, v: &u8) -> Result<IntoIter<u8>, GraphError> {
        let mut inn: Vec<u8> = self
            .in_neighbors(v)
            .map_or(vec![], |it| it.cloned().collect());
        let outn: Vec<u8> = self
            .out_neighbors(v)
            .map_or(vec![], |it| it.cloned().collect());
        if !inn.is_empty() && !outn.is_empty() {
            inn.extend(outn);
            inn.sort();
            return Ok(inn.into_iter());
        } else if !inn.is_empty() {
            return Ok(inn.into_iter());
        } else if !outn.is_empty() {
            return Ok(outn.into_iter());
        }
        return Err(GraphError::NoSuchVertex(*v));
    }

    pub fn map_edge<Func>(&self, loc: &u8, mut f: Func) -> Result<(), GraphError>
    where
        Func: FnMut(&F, &u8),
    {
        if !self.has_vertex(loc) {
            return Err(GraphError::NoSuchVertex(*loc));
        }
        let _ = self.in_neighbors(loc).and_then(|vs| {
            for v in vs {
                let fd = self.edge_with_vertex(*v, *loc).unwrap();
                f(fd, v);
            }
            Ok(())
        });
        let _ = self.out_neighbors(loc).and_then(|vs| {
            for v in vs {
                let fd = self.edge_with_vertex(*loc, *v).unwrap();
                f(fd, v);
            }
            Ok(())
        });
        Ok(())
    }

    pub fn map_vertex<Func>(&self, loc: &u8, mut f: Func) -> Result<(), GraphError>
    where
        Func: FnMut(&T),
    {
        self.neighbors(loc).and_then(|vs| {
            for v in vs {
                let fd = self.vertex(&v)?;
                f(fd);
            }
            Ok(())
        })?;
        Ok(())
    }

    pub fn vertex(&self, v: &u8) -> Result<&T, GraphError> {
        self.vertices.get(v).ok_or(GraphError::NoSuchVertex(*v))
    }

    pub fn update_vertex(&mut self, k: u8, v: T) {
        self.vertices.insert(k, v);
    }

    pub fn vertex_mut(&mut self, v: &u8) -> Result<&mut T, GraphError> {
        self.vertices
            .get_mut(&v)
            .ok_or(GraphError::NoSuchVertex(*v))
    }

    pub fn edge(&self, e: &Edge) -> Result<&F, GraphError> {
        self.edges
            .get(&e)
            .ok_or(GraphError::NoSuchEdge(*e.inbound(), *e.outbound()))
    }

    pub fn edge_mut(&mut self, e: &Edge) -> Result<&mut F, GraphError> {
        self.edges
            .get_mut(&e)
            .ok_or(GraphError::NoSuchEdge(*e.inbound(), *e.outbound()))
    }

    pub fn edge_with_vertex(&self, a: u8, b: u8) -> Result<&F, GraphError> {
        let edge = Edge::new(a, b);
        return self.edge(&edge).and_then(|f| Ok(f));
    }

    pub fn replace(&mut self, e: Edge, desc: F) {
        self.edges.insert(e, desc);
    }

    #[inline]
    pub fn order(&self) -> usize {
        self.vertices.len()
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.edges.len()
    }

    pub fn map_edges<Func>(&self, mut f: Func) -> Result<(), GraphError>
    where
        Func: FnMut(&Edge, &F),
    {
        for (k, v) in self.edges.iter() {
            f(k, v)
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Graph, GraphError};

    #[test]
    fn test_has_vertex() {
        let mut g = Graph::new();
        g.add_vertex(0, "C").unwrap();
        g.add_vertex(1, "H").unwrap();
        assert!(g.has_vertex(&0));
        assert!(g.has_vertex(&1));
        assert!(!g.has_vertex(&3));
        assert_eq!(g.add_edge(0, 2, "Double"), Err(GraphError::NoSuchVertex(2)));
        g.add_vertex(3, "O").unwrap();
        assert!(g.has_vertex(&3));
    }

    #[test]
    fn test_has_edge() {
        let mut g = Graph::new();
        g.add_vertex(0, "C").unwrap();
        g.add_vertex(1, "H").unwrap();
        g.add_vertex(3, "O").unwrap();
        g.add_edge(0, 1, "-").unwrap();
        g.add_edge(0, 3, "=").unwrap();
        assert!(!g.has_edge(&1, &0));
        assert!(g.has_edge(&0, &1));
        assert!(!g.has_edge(&1, &3));
        assert!(!g.has_edge(&1, &4));
        assert!(g.has_edge(&0, &3));
    }
}
