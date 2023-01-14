# infset

[![Crates.io](https://img.shields.io/crates/v/infset.svg)](https://crates.io/crates/infset)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Possseidon/infset/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/infset.svg)](https://crates.io/crates/infset)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/Possseidon/infset/main.svg)](https://results.pre-commit.ci/latest/github/Possseidon/infset/main)
[![CI status](https://github.com/Possseidon/infset/actions/workflows/ci.yml/badge.svg)](https://github.com/Possseidon/infset/actions/workflows/ci.yml?query=branch%3Amain+)

Infinite set types that can not only represent the union of elements, like a regular e.g. `BTreeSet<T>`, but also the complement of one.

If "set theory" scares you, you can think of it as a combination of both a whitelist and a blacklist, which is also a great real-world application of such a type.

## Examples

```rust
// TODO
```

## FAQ

### How does it work?

All `Inf*Set` types basically boil down to this:

```rust ignore
enum InfSet<T> {
    Union(Set<T>),
    Complement(Set<T>),
}
```

It contains either:

- A regular set; as in a `Union` of elements.
- A set of all elements that **aren't** in the set; as in the `Complement` of "all" elements.

### What about `Inf*Map`?

While sets and maps are similar in some aspects, it doesn't really translate over when you have an "infinite" set of possible elements.

The point of a map is, that every element in a set has some additional data attached. If you now have the complement of a set, how would you attach data to those missing elements? It just doesn't really make sense.

### Why does `insert` not insert elements into my `Complement` set?

A `Complement` set includes "all" elements in the whole world. If you insert something it will instead be removed from the internal representation. What you're looking for is `remove` which, in the case for `Complement` sets, inserts elements into the internal representation.

### Why does `remove` not remove elements from my `Complement` set?

Read the previous paragraph; you're looking for `insert`.

### Why is the function `foo` that exists on `std::collections::*Set` missing?

#### `range`, `difference`, `symmetric_difference`, `intersection`, `union`, `iter`

All iteration related functions cannot exist, as there is no way to iterate the "infinite" elements of a complement set.

One could in theory iterate the "missing" elements of a complement set, but that does not match the intended semantics of the type.

Note, that you can of course still manually unpack the internal (possibly complement) set and iterate over that, but at least your intent will be explicit then.

#### `get`

The `get` function should return `Some(_)` in case the given value is in the set. Keeping this semantic in mind, for complement sets this would mean, that a non-existent value would have to be returned.

One could argue, that it could just return a reference to the passed value as is, which I am still thinking about. What's currently stopping me, is the fact that the passed `value` type and actual set type do not have to match, which raises the question of what the return type should actually be. But even then, I don't see a reasonable use-case and `contains` is probably what you're looking for instead.

#### `first`, `last`, `pop_first`, `pop_last`

These have similar reasoning to iteration related functions. The "first" and "last" element of a complement set does not actually exist anywhere and thus can be returned nor "removed" (which would be translated to an insert for complement sets).

#### `replace`

In the case of complement sets, this would be more or less synonymous with `insert`, while discarding the passed value.

Additionally, the return value `Option<T>`, which returns the replaced elements, cannot exist elements in the case of a complement set, as what it would need to return doesn't actually exist in memory.

#### `take`

You cannot take elements out of a complement set, as they don't exist in memory.

#### `split_off`

You cannot split a set on an element that doesn't actually exist in memory.

#### `len`

What is the length of an infinite set? Well, infinite, which isn't representable with a `usize`. One could however introduce a new type that supports "all elements except `N`" semantics like so:

```rust
enum InfSetSize {
    Union(usize),
    Complement(usize),
}
```

You can however use either `union_len` or `complement_len`, which return `Option<usize>` instead.

### Why is `Inf*Set::from([false, true])` not equal to `Inf*Set::all()`?

`Inf*Set` types assume, that the set of possible elements is infinite. `Inf*Set::all()` doesn't mean "all possible `bool` values", but instead means "**all** values that could possibly exist in the entire universe". Of course, you can't actually add anything that isn't a `bool`, but as far as semantics go, that is how you have to look at it.
