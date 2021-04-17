#[cfg(test)]
use crate::*;

#[cfg(test)]
pub fn make_small_table_collection() -> TableCollection {
    let mut tables = TableCollection::new(1000.).unwrap();
    tables.add_node(0, 1.0, TSK_NULL, TSK_NULL).unwrap();
    tables
        .add_node(TSK_NODE_IS_SAMPLE, 0.0, TSK_NULL, TSK_NULL)
        .unwrap();
    tables
        .add_node(TSK_NODE_IS_SAMPLE, 0.0, TSK_NULL, TSK_NULL)
        .unwrap();
    tables.add_edge(0., 1000., 0, 1).unwrap();
    tables.add_edge(0., 1000., 0, 2).unwrap();
    tables.build_index(0).unwrap();
    tables
}

#[cfg(test)]
pub fn treeseq_from_small_table_collection() -> TreeSequence {
    let tables = make_small_table_collection();
    tables.tree_sequence().unwrap()
}

#[cfg(test)]
pub fn make_small_table_collection_two_trees() -> TableCollection {
    // The two trees are:
    //  0
    // +++
    // | |  1
    // | | +++
    // 2 3 4 5

    //     0
    //   +-+-+
    //   1   |
    // +-+-+ |
    // 2 4 5 3

    let mut tables = TableCollection::new(1000.).unwrap();
    tables.add_node(0, 2.0, TSK_NULL, TSK_NULL).unwrap();
    tables.add_node(0, 1.0, TSK_NULL, TSK_NULL).unwrap();
    tables
        .add_node(TSK_NODE_IS_SAMPLE, 0.0, TSK_NULL, TSK_NULL)
        .unwrap();
    tables
        .add_node(TSK_NODE_IS_SAMPLE, 0.0, TSK_NULL, TSK_NULL)
        .unwrap();
    tables
        .add_node(TSK_NODE_IS_SAMPLE, 0.0, TSK_NULL, TSK_NULL)
        .unwrap();
    tables
        .add_node(TSK_NODE_IS_SAMPLE, 0.0, TSK_NULL, TSK_NULL)
        .unwrap();
    tables.add_edge(500., 1000., 0, 1).unwrap();
    tables.add_edge(0., 500., 0, 2).unwrap();
    tables.add_edge(0., 1000., 0, 3).unwrap();
    tables.add_edge(500., 1000., 1, 2).unwrap();
    tables.add_edge(0., 1000., 1, 4).unwrap();
    tables.add_edge(0., 1000., 1, 5).unwrap();
    tables.full_sort().unwrap();
    tables.build_index(0).unwrap();
    tables
}

#[cfg(test)]
pub fn treeseq_from_small_table_collection_two_trees() -> TreeSequence {
    let tables = make_small_table_collection_two_trees();
    tables.tree_sequence().unwrap()
}
