use std::hash::Hash;

use timely::dataflow::Scope;

use differential_dataflow::{ExchangeData, Collection};
use differential_dataflow::difference::{Semigroup, Multiply};
use differential_dataflow::lattice::Lattice;
use differential_dataflow::operators::arrange::Arranged;
use differential_dataflow::trace::{Cursor, TraceReader, BatchReader};

/// Proposes extensions to a stream of prefixes.
///
/// This method takes a stream of prefixes and for each determines a
/// key with `key_selector` and then proposes all pair af the prefix
/// and values associated with the key in `arrangement`.
pub fn validate<G, K, V, Tr, F, P>(
    extensions: &Collection<G, (P, V), Tr::R>,
    arrangement: Arranged<G, Tr>,
    key_selector: F,
) -> Collection<G, (P, V), Tr::R>
where
    G: Scope,
    G::Timestamp: Lattice,
    Tr: TraceReader<Key=(K,V), Val=(), Time=G::Timestamp>+Clone+'static,
    K: Ord+Hash+Clone+Default,
    V: ExchangeData+Hash+Default,
    Tr::Batch: BatchReader<Tr::Key, Tr::Val, Tr::Time, Tr::R>,
    Tr::Cursor: Cursor<Tr::Key, Tr::Val, Tr::Time, Tr::R>,
    Tr::R: Semigroup+Multiply<Output = Tr::R>+ExchangeData,
    F: Fn(&P)->K+Clone+'static,
    P: ExchangeData,
{
    crate::operators::lookup_map(
        extensions,
        arrangement,
        move |(pre,val),key| { *key = (key_selector(pre), val.clone()); },
        |(pre,val),r,&(),_| ((pre.clone(), val.clone()), r.clone()),
        Default::default(),
        Default::default(),
        Default::default(),
    )
}
