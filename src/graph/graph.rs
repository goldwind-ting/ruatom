use super::{edge::Edge, vertices::VertexIter};
use crate::error::RuatomError;
use hashbrown::HashMap;
use std::slice::Iter;

#[derive(Clone, Debug, Default)]
pub struct Graph<T, F> {
    vertices: HashMap<u8, T>,
    edges: HashMap<Edge, F>,
    bound_table: HashMap<u8, Vec<u8>>,
}

impl<T, F: Clone> Graph<T, F> {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            bound_table: HashMap::new(),
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
            bound_table: HashMap::with_capacity(capacity),
        }
    }

    #[inline]
    fn has_vertex(&self, v: &u8) -> bool {
        self.vertices.get(&v).is_some()
    }

    #[inline]
    pub fn add_vertex(&mut self, k: u8, v: T) -> Result<(), RuatomError> {
        if self.has_vertex(&k) {
            return Err(RuatomError::ExistedVertex(k));
        }
        self.bound_table.insert(k, Vec::new());
        self.vertices.insert(k, v);
        Ok(())
    }

    pub fn add_edge(&mut self, a: u8, b: u8, attr: F) -> Result<bool, RuatomError> {
        if self.has_edge(&a, &b) {
            return Err(RuatomError::ExistedEdge(a, b));
        }
        if a == b {
            return Err(RuatomError::InvalidEdge(a, b));
        }
        if let Err(e) = self.do_add_edge(a, b, attr, None) {
            return Err(e);
        };
        return Ok(true);
    }

    pub fn add_direction_edge(
        &mut self,
        a: u8,
        b: u8,
        attr_ab: F,
        attr_ba: F,
    ) -> Result<bool, RuatomError> {
        if self.has_edge(&a, &b) {
            return Err(RuatomError::ExistedEdge(a, b));
        }
        if a == b {
            return Err(RuatomError::InvalidEdge(a, b));
        }
        if let Err(e) = self.do_add_edge(a, b, attr_ab, Some(attr_ba)) {
            return Err(e);
        };
        return Ok(true);
    }

    #[inline]
    fn has_edge(&self, a: &u8, b: &u8) -> bool {
        match self.bound_table.get(a) {
            None => false,
            Some(bound) => bound.contains(b),
        }
    }

    fn do_add_edge(
        &mut self,
        a: u8,
        b: u8,
        attr: F,
        direction_attr: Option<F>,
    ) -> Result<(), RuatomError> {
        if !self.vertices.contains_key(&a) {
            return Err(RuatomError::NoSuchVertex(a));
        }
        if !self.vertices.contains_key(&b) {
            return Err(RuatomError::NoSuchVertex(b));
        }
        let edge_ab = Edge::new(a, b);
        self.edges.insert(edge_ab, attr.clone());
        match direction_attr {
            Some(attr_inner) => {
                let edge_ba = Edge::new(b, a);
                self.edges.insert(edge_ba, attr_inner);
            }
            None => {
                let edge_ba = Edge::new(b, a);
                self.edges.insert(edge_ba, attr);
            }
        }

        match self.bound_table.get_mut(&b) {
            None => {
                self.bound_table.insert(b, vec![a]);
            }
            Some(bound) => {
                bound.push(a);
                bound.sort();
            }
        }
        match self.bound_table.get_mut(&a) {
            None => {
                self.bound_table.insert(a, vec![b]);
            }
            Some(bound) => {
                bound.push(b);
                bound.sort();
            }
        }
        Ok(())
    }

    #[inline]
    pub fn bound_count(&self, v: &u8) -> Result<usize, RuatomError> {
        if !self.has_vertex(v) {
            return Err(RuatomError::NoSuchVertex(*v));
        }
        self.bound_table.get(v).map_or(Ok(0), |l| Ok(l.len()))
    }

    #[inline]
    pub fn adjancent(&self, a: u8, b: u8) -> bool {
        let e1 = Edge::new(a, b);
        let e2 = Edge::new(b, a);
        self.edges.contains_key(&e1) || self.edges.contains_key(&e2)
    }

    #[inline]
    pub fn neighbors(&self, v: &u8) -> Result<VertexIter<'_, Iter<'_, u8>>, RuatomError> {
        self.bound_table
            .get(v)
            .map_or(Err(RuatomError::NoSuchVertex(*v)), |l| {
                Ok(VertexIter::new(l.iter()))
            })
    }

    pub fn map_edge<Func>(&self, loc: &u8, mut f: Func) -> Result<(), RuatomError>
    where
        Func: FnMut(&F, &u8),
    {
        if !self.has_vertex(loc) {
            return Err(RuatomError::NoSuchVertex(*loc));
        }
        let _ = self.neighbors(loc).and_then(|vs| {
            for v in vs {
                let fd = self.edge_with_vertex(*v, *loc).unwrap();
                f(fd, v);
            }
            Ok(())
        });
        Ok(())
    }

    pub fn map_vertex<Func>(&self, loc: &u8, mut f: Func) -> Result<(), RuatomError>
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

    #[inline]
    pub fn vertex(&self, v: &u8) -> Result<&T, RuatomError> {
        self.vertices.get(v).ok_or(RuatomError::NoSuchVertex(*v))
    }

    pub fn update_vertex(&mut self, k: u8, v: T) {
        self.vertices.insert(k, v);
    }

    pub fn vertex_mut(&mut self, v: &u8) -> Result<&mut T, RuatomError> {
        self.vertices
            .get_mut(&v)
            .ok_or(RuatomError::NoSuchVertex(*v))
    }

    pub fn edge(&self, e: &Edge) -> Result<&F, RuatomError> {
        self.edges
            .get(&e)
            .ok_or(RuatomError::NoSuchEdge(*e.inbound(), *e.outbound()))
    }

    pub fn edge_mut(&mut self, e: &Edge) -> Result<&mut F, RuatomError> {
        self.edges
            .get_mut(&e)
            .ok_or(RuatomError::NoSuchEdge(*e.inbound(), *e.outbound()))
    }

    pub fn edge_with_vertex(&self, a: u8, b: u8) -> Result<&F, RuatomError> {
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
        self.edges.len() / 2
    }

    pub fn map_edges<Func>(&self, mut f: Func) -> Result<(), RuatomError>
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
    use crate::error::RuatomError;
    use crate::graph::Graph;

    #[test]
    fn test_has_vertex() {
        let mut g = Graph::new();
        g.add_vertex(0, "C").unwrap();
        g.add_vertex(1, "H").unwrap();
        assert!(g.has_vertex(&0));
        assert!(g.has_vertex(&1));
        assert!(!g.has_vertex(&3));
        assert_eq!(
            g.add_edge(0, 2, "Double"),
            Err(RuatomError::NoSuchVertex(2))
        );
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
        assert!(g.has_edge(&1, &0));
        assert!(g.has_edge(&0, &1));
        assert!(!g.has_edge(&1, &3));
        assert!(!g.has_edge(&1, &4));
        assert!(g.has_edge(&0, &3));
    }
}
