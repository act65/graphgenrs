Implementation of orderly enumeration of graphs.

Edges are given an ordered. This is used to order the graphs and allows us to define canonicity via the smallest graph (the 'minimal' graph).
The main insight in this algorithm is that a minimal graph with k edges will have a k-1 sub graph that is also minimal. Thus we can enumerate but adding edges to only the minimal graphs.

Limitations;

- Doesnt support any constraints.
- Tests all permutations to filter out isomers.
- Doesnt support parallel enumeration.
- Has a bug where some extra isomers are getting through.