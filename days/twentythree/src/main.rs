use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

fn main() {
    println!("Hello, advent of code day twentythree!");
    let input = include_str!("../input.txt");
    // let input = TEST_ALIAS_LIST;
    let connections: Vec<ComputerConnection> = input
        .trim()
        .lines()
        .filter_map(|line| ComputerConnection::try_from(line.trim()).ok())
        .collect();
    println!("{}", connections.len());
    let map = NetworkMap::new(connections);
    println!("{}", map);
    // let mut triplets = map.find_triplets();
    // triplets.iter().for_each(|triplet| {
    //     println!("{}", triplet);
    // });
    // triplets.retain(|triplet| triplet.has_alias_starting_with_t());
    // println!("Possible games {}", triplets.len());
    let cliques = map.bron_kerbosh();
    let larget_clique = cliques.iter().max_by_key(|clique| clique.len()).unwrap();
    println!("Largest Clique: {:?}", larget_clique.len());
    let mut password = larget_clique.iter().fold(Vec::new(), |mut acc, alias| {
        acc.push(*alias);
        acc
    });
    password.sort();
    password.iter().for_each(|alias| {
        print!("{},", alias);
    });
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MultiplayerGame([ComputerAlias; 3]);
impl MultiplayerGame {
    fn has_alias_starting_with_t(&self) -> bool {
        self.0.iter().any(|alias| alias.0[0] == 't')
    }
}
impl Display for MultiplayerGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}
struct NetworkMap {
    connections: HashMap<ComputerAlias, HashSet<ComputerAlias>>,
}
impl NetworkMap {
    fn bron_kerbosh(&self) -> Vec<HashSet<ComputerAlias>> {
        let mut r: HashSet<ComputerAlias> = HashSet::new();
        let mut p: HashSet<ComputerAlias> = self.connections.keys().cloned().collect();
        let mut x: HashSet<ComputerAlias> = HashSet::new();
        let mut cliques: Vec<HashSet<ComputerAlias>> = Vec::new();
        self.bron_kerbosh_recursive(&mut r, &mut p, &mut x, &mut cliques);
        cliques
    }

    fn bron_kerbosh_recursive(
        &self,
        r: &mut HashSet<ComputerAlias>,
        p: &mut HashSet<ComputerAlias>,
        x: &mut HashSet<ComputerAlias>,
        cliques: &mut Vec<HashSet<ComputerAlias>>,
    ) {
        // Base case: when P and X are both empty, we found a maximal clique
        if p.is_empty() && x.is_empty() {
            cliques.push(r.clone());
            return;
        }

        let p_copy = p.clone();
        for v in p_copy.iter() {
            // Add v to the current clique R
            r.insert(*v);

            // Get the intersection of P and the neighbors of v (P_v)
            let p_v = &self.connections[v] & p;
            // Get the intersection of X and the neighbors of v (X_v)
            let x_v = &self.connections[v] & x;

            // Recurse with updated P_v and X_v
            self.bron_kerbosh_recursive(r, &mut p_v.clone(), &mut x_v.clone(), cliques);

            // Backtrack: Remove v from the current clique R
            r.remove(v);

            // Move v from P to X
            p.remove(v);
            x.insert(*v);
        }
    }
    fn new(connections: Vec<ComputerConnection>) -> Self {
        let mut map: HashMap<ComputerAlias, HashSet<ComputerAlias>> = HashMap::new();
        for conn in connections {
            map.entry(conn.0)
                .or_insert_with(HashSet::new)
                .insert(conn.1);
            map.entry(conn.1)
                .or_insert_with(HashSet::new)
                .insert(conn.0);
        }
        Self { connections: map }
    }
    // Function to find all interconnected triplets
    fn find_triplets(&self) -> Vec<MultiplayerGame> {
        let mut triplets: Vec<MultiplayerGame> = Vec::new();

        // Loop over all combinations of three aliases
        let aliases: Vec<&ComputerAlias> = self.connections.keys().collect();
        for i in 0..aliases.len() {
            for j in i + 1..aliases.len() {
                for k in j + 1..aliases.len() {
                    let a = aliases[i];
                    let b = aliases[j];
                    let c = aliases[k];
                    if self.is_connected(a, b) && self.is_connected(b, c) && self.is_connected(a, c)
                    {
                        // If all three are connected, it's a valid triplet
                        triplets.push(MultiplayerGame([*a, *b, *c]));
                    }
                }
            }
        }
        triplets.iter_mut().for_each(|v| v.0.sort());
        triplets.sort();
        triplets
    }

    // Helper function to check if two aliases are connected
    fn is_connected(&self, a: &ComputerAlias, b: &ComputerAlias) -> bool {
        if let Some(connections) = self.connections.get(a) {
            connections.contains(b)
        } else {
            false
        }
    }
}
impl Display for NetworkMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (alias, connections) in self.connections.iter() {
            write!(f, "{}: ", alias)?;
            for conn in connections {
                write!(f, "{} ", conn)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ComputerAlias([char; 2]);
impl Display for ComputerAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ComputerConnection(ComputerAlias, ComputerAlias);
impl Display for ComputerConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}
impl TryFrom<&str> for ComputerConnection {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (a_str, b_str) = value.split_once('-').ok_or("Invalid input")?;
        let a = a_str
            .chars()
            .take(2)
            .fold(ComputerAlias(['\0'; 2]), |mut acc, c| {
                acc.0[acc.0.iter().position(|&x| x == '\0').unwrap()] = c;
                acc
            });
        let b = b_str
            .chars()
            .take(2)
            .fold(ComputerAlias(['\0'; 2]), |mut acc, c| {
                acc.0[acc.0.iter().position(|&x| x == '\0').unwrap()] = c;
                acc
            });
        Ok(ComputerConnection(a, b))
    }
}

const TEST_ALIAS_LIST: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;
