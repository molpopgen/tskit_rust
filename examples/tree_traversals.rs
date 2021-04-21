use clap::{value_t_or_exit, App, Arg};
use streaming_iterator::StreamingIterator; // Required for tree iteration

// "Manual" traversal from samples to root
fn traverse_upwards(tree: &tskit::Tree) {
    let samples = tree.samples_to_vec();

    for s in samples.iter() {
        let mut u = *s;
        while u != tskit::TSK_NULL {
            u = tree.parent(u).unwrap();
        }
    }
}

// Iterate from each node up to its root.
fn traverse_upwards_with_iterator(tree: &tskit::Tree) {
    let samples = tree.samples_to_vec();

    for s in samples.iter() {
        // _steps_to_root counts the number of steps,
        // including the starting node s.
        for (_steps_to_root, _) in tree.path_to_root(*s).unwrap().enumerate() {}
    }
}

fn preorder_traversal(tree: &tskit::Tree) {
    for _ in tree.traverse_nodes(tskit::NodeTraversalOrder::Preorder) {}
}

fn main() {
    let matches = App::new("tree_traversals")
        .arg(
            Arg::with_name("treefile")
                .short("t")
                .long("treefile")
                .help("Tree file name")
                .takes_value(true),
        )
        .get_matches();

    let treefile = value_t_or_exit!(matches.value_of("treefile"), String);

    let treeseq = tskit::TreeSequence::load(&treefile).unwrap();

    let mut tree_iterator = treeseq.tree_iterator(tskit::TreeFlags::default()).unwrap();

    while let Some(tree) = tree_iterator.next() {
        traverse_upwards(&tree);
        traverse_upwards_with_iterator(&tree);
        preorder_traversal(&tree);
    }
}