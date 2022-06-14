macro_rules! impl_attrs {
    () => {
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs
                .extend(attrs.iter().map(|&(k, v)| (k.into(), v.into())));
            self
        }

        pub fn get_attr(&self, attr: &str) -> Option<&str> {
            self.attrs.get(attr).map(|x| x.as_str())
        }
    };
}

pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn get_node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == node_name)
        }

        impl_attrs!();
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Default, Clone, Debug, Eq, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        ..Self::default()
                    }
                }

                impl_attrs!();
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Default, Clone, Debug, Eq, PartialEq)]
            pub struct Edge {
                a: String,
                b: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {
                        a: a.to_string(),
                        b: b.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                impl_attrs!();
            }
        }
    }
}
