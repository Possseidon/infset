use std::{
    borrow::Borrow,
    collections::BTreeSet,
    mem::swap,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

use crate::convert::FromComplement;

/// A set that can not only represent the union of elements, but also the complement of a
/// theoretical infinite set.
///
/// It is assumed, that the set of elements, representable by `T`, is infinite.
/// Say we choose `bool` for our element type which only has `true` and `false` as possible
/// elements:
/// Since we know, that the set of possible elements for `bool` is not infinite, one could assume
/// that `InvBTreeSet::from_complement([false, true])` should be equal to the empty set.
/// This however is not the case, as an empty complement is instead seen as containing literally
/// "everything", and not just everything representable by `T`.
///
/// With this in mind, it usually makes more sense to choose a type that actually does have an
/// infinite number of possible elements.
/// Examples for this would be a recursive structure or even just a [`Vec<T>`].
///
/// It can also make sense to use integers (or floats), which might not have an infinite number of
/// actually representable values, but are usually assumed to be a representation of the entire set
/// of all integers up to infinity.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InfBTreeSet<T> {
    /// Elements that are part of the set.
    Union(BTreeSet<T>),
    /// Elements that are *not* part of the set.
    Complement(BTreeSet<T>),
}

impl<T> InfBTreeSet<T> {
    /// Makes a new, empty [`InfBTreeSet`].
    ///
    /// Does not allocate anything on its own.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use infset::btree::InfBTreeSet;
    ///
    /// let mut set: InfBTreeSet<i32> = InfBTreeSet::new();
    /// ```
    pub const fn new() -> Self {
        Self::Union(BTreeSet::new())
    }

    /// Makes a new [`InfBTreeSet`] containing "all" values.
    ///
    /// Does not allocate anything on its own.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use infset::btree::InfBTreeSet;
    ///
    /// let mut set: InfBTreeSet<i32> = InfBTreeSet::new();
    /// ```
    pub const fn all() -> Self {
        Self::Complement(BTreeSet::new())
    }

    /// Clears the set, removing all elements.
    ///
    /// If the set is currently a [`Complement`], it will be changed to an empty [`Union`].
    ///
    /// # Examples
    ///
    /// ```
    /// use infset::btree::InfBTreeSet;
    ///
    /// let mut set = InfBTreeSet::from([1, 2, 3]);
    /// set.clear();
    /// assert!(set.is_empty());
    ///
    /// let mut set = InfBTreeSet::<i32>::all();
    /// set.clear();
    /// assert!(set.is_empty());
    /// ```
    ///
    /// [`Union`]: InfBTreeSet::Union
    /// [`Complement`]: InfBTreeSet::Complement
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Returns `true` if the set contains an element equal to the value.
    ///
    /// The value may be any borrowed form of the set's element type, but the ordering on the
    /// borrowed form *must* match the ordering on the element type.
    ///
    /// # Examples
    ///
    /// ```
    /// use infset::{btree::InfBTreeSet, convert::FromComplement};
    ///
    /// let union = InfBTreeSet::from([42]);
    /// assert!(union.contains(&42));
    /// assert!(!union.contains(&256));
    ///
    /// let complement = InfBTreeSet::from_complement([42]);
    /// assert!(!complement.contains(&42));
    /// assert!(complement.contains(&256));
    /// ```
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        match self {
            InfBTreeSet::Union(union) => union.contains(value),
            InfBTreeSet::Complement(complement) => !complement.contains(value),
        }
    }

    /// Returns `true` if [`self`] has no elements in common with `other`.
    ///
    /// A [`Union`] and [`Complement`] are disjoint, if the [`Union`] is a subset of the
    /// [`Complement`]'s elements.
    ///
    /// Two [`Complement`]s will never be disjoint, as they always have an overlap because of
    /// their "infinite" nature.
    ///
    /// # Examples
    ///
    /// ```
    /// use infset::{btree::InfBTreeSet, convert::FromComplement};
    ///
    /// // Unions are disjoint if there is no overlap:
    /// let union1 = InfBTreeSet::from([1]);
    /// assert!(!union1.is_disjoint(&union1));
    ///
    /// let union2 = InfBTreeSet::from([2]);
    /// assert!(union1.is_disjoint(&union2));
    ///
    /// // A union and a complement are disjoint, if the union is a subset of the complement's
    /// // values:
    /// let complement1 = InfBTreeSet::from_complement([1]);
    /// assert!(union1.is_disjoint(&complement1));
    ///
    /// let complement2 = InfBTreeSet::from_complement([2]);
    /// assert!(!union1.is_disjoint(&complement2));
    ///
    /// // Complements always overlap because of their "infinite" nature:
    /// let all = InfBTreeSet::<u32>::all();
    /// assert!(!all.is_disjoint(&all));
    /// ```
    ///
    /// [`Union`]: InfBTreeSet::Union
    /// [`Complement`]: InfBTreeSet::Complement
    pub fn is_disjoint(&self, other: &InfBTreeSet<T>) -> bool
    where
        T: Ord,
    {
        match (self, other) {
            (InfBTreeSet::Union(this), InfBTreeSet::Union(other)) => this.is_disjoint(other),
            (InfBTreeSet::Union(union), InfBTreeSet::Complement(complement))
            | (InfBTreeSet::Complement(complement), InfBTreeSet::Union(union)) => {
                union.is_subset(complement)
            }
            (InfBTreeSet::Complement(_), InfBTreeSet::Complement(_)) => false,
        }
    }

    /// Returns `true` if the set is a subset of another, i.e., `other` contains at least all the
    /// elements in `self`.
    pub fn is_subset(&self, other: &InfBTreeSet<T>) -> bool
    where
        T: Ord,
    {
        match (self, other) {
            (InfBTreeSet::Union(this), InfBTreeSet::Union(other)) => this.is_subset(other),
            (InfBTreeSet::Union(_), InfBTreeSet::Complement(_)) => todo!(),
            (InfBTreeSet::Complement(_), InfBTreeSet::Union(_)) => todo!(),
            (InfBTreeSet::Complement(this), InfBTreeSet::Complement(other)) => {
                other.is_subset(this)
            }
        }
    }

    /// Returns `true` if the set is a superset of another, i.e., `self` contains at least all the
    /// elements in `other`.
    pub fn is_superset(&self, other: &InfBTreeSet<T>) -> bool
    where
        T: Ord,
    {
        other.is_subset(self)
    }

    // ---

    pub fn is_empty(&self) -> bool {
        self.as_union().map_or(false, |union| union.is_empty())
    }

    pub fn is_all(&self) -> bool {
        self.as_complement()
            .map_or(false, |complement| complement.is_empty())
    }

    pub fn is_union(&self) -> bool {
        matches!(self, Self::Union(_))
    }

    pub fn as_union(&self) -> Option<&BTreeSet<T>> {
        if let Self::Union(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_union(self) -> Result<BTreeSet<T>, Self> {
        if let Self::Union(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn is_complement(&self) -> bool {
        matches!(self, Self::Complement(_))
    }

    pub fn as_complement(&self) -> Option<&BTreeSet<T>> {
        if let Self::Complement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_complement(self) -> Result<BTreeSet<T>, Self> {
        if let Self::Complement(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub fn as_storage(&self) -> &BTreeSet<T> {
        let (Self::Union(storage) | Self::Complement(storage)) = self;
        storage
    }

    pub fn into_storage(self) -> BTreeSet<T> {
        let (Self::Union(storage) | Self::Complement(storage)) = self;
        storage
    }

    pub fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        match self {
            InfBTreeSet::Union(set) => {
                set.insert(value);
            }
            InfBTreeSet::Complement(set) => {
                set.remove(&value);
            }
        }
    }
}

impl<T> From<BTreeSet<T>> for InfBTreeSet<T> {
    fn from(v: BTreeSet<T>) -> Self {
        Self::Union(v)
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for InfBTreeSet<T> {
    fn from(arr: [T; N]) -> Self {
        Self::from(BTreeSet::from(arr))
    }
}

impl<T: Ord> FromIterator<T> for InfBTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from(BTreeSet::from_iter(iter))
    }
}

impl<T> FromComplement<BTreeSet<T>> for InfBTreeSet<T> {
    fn from_complement(v: BTreeSet<T>) -> Self {
        Self::Complement(v)
    }
}

impl<T: Ord, const N: usize> FromComplement<[T; N]> for InfBTreeSet<T> {
    fn from_complement(arr: [T; N]) -> Self {
        Self::from_complement(BTreeSet::from(arr))
    }
}

impl<T> TryFrom<InfBTreeSet<T>> for BTreeSet<T> {
    type Error = InfBTreeSet<T>;

    fn try_from(value: InfBTreeSet<T>) -> Result<Self, Self::Error> {
        value.try_into_union()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for InfBTreeSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_complement() {
            write!(f, "!")?;
        }
        self.as_storage().fmt(f)
    }
}

impl<T: Default> Default for InfBTreeSet<T> {
    /// Creates an empty `InfBTreeSet`.
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> BitOr for InfBTreeSet<T> {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<T: Ord + Clone> BitOr<&InfBTreeSet<T>> for InfBTreeSet<T> {
    type Output = Self;

    fn bitor(mut self, rhs: &InfBTreeSet<T>) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<T: Ord + Clone> BitOr for &InfBTreeSet<T> {
    type Output = InfBTreeSet<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (InfBTreeSet::Union(lhs), InfBTreeSet::Union(rhs)) => InfBTreeSet::Union(lhs | rhs),
            (InfBTreeSet::Union(union), InfBTreeSet::Complement(complement))
            | (InfBTreeSet::Complement(complement), InfBTreeSet::Union(union)) => {
                InfBTreeSet::Complement(complement - union)
            }
            (InfBTreeSet::Complement(lhs), InfBTreeSet::Complement(rhs)) => {
                InfBTreeSet::Complement(lhs & rhs)
            }
        }
    }
}

impl<T: Ord + Clone> BitOr<InfBTreeSet<T>> for &InfBTreeSet<T> {
    type Output = InfBTreeSet<T>;

    fn bitor(self, rhs: InfBTreeSet<T>) -> Self::Output {
        rhs | self
    }
}

impl<T: Ord + Clone> BitOrAssign for InfBTreeSet<T> {
    fn bitor_assign(&mut self, mut rhs: Self) {
        if let (InfBTreeSet::Union(_), InfBTreeSet::Complement(_)) = (&self, &rhs) {
            swap(self, &mut rhs);
        }
        match (self, rhs) {
            (Self::Union(lhs), Self::Union(mut rhs)) => {
                lhs.append(&mut rhs);
            }
            (Self::Complement(complement), Self::Union(union)) => {
                complement.retain(|ty| !union.contains(ty));
            }
            (Self::Union(_), Self::Complement(_)) => unreachable!(),
            (Self::Complement(lhs), Self::Complement(rhs)) => {
                lhs.retain(|ty| rhs.contains(ty));
            }
        }
    }
}

impl<T: Ord + Clone> BitOrAssign<&InfBTreeSet<T>> for InfBTreeSet<T> {
    fn bitor_assign(&mut self, rhs: &InfBTreeSet<T>) {
        if let (InfBTreeSet::Union(union), InfBTreeSet::Complement(complement)) = (&self, rhs) {
            let mut complement = complement.clone();
            complement.retain(|ty| !union.contains(ty));
            *self = Self::Complement(complement);
            return;
        }
        match (self, rhs) {
            (Self::Union(lhs), Self::Union(rhs)) => {
                lhs.append(&mut rhs.clone());
            }
            (Self::Union(_), Self::Complement(_)) => unreachable!(),
            (Self::Complement(complement), Self::Union(union)) => {
                complement.retain(|ty| !union.contains(ty));
            }
            (Self::Complement(lhs), Self::Complement(rhs)) => {
                lhs.retain(|ty| rhs.contains(ty));
            }
        }
    }
}

impl<T: Ord + Clone> BitAnd for InfBTreeSet<T> {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<T: Ord + Clone> BitAnd<&InfBTreeSet<T>> for InfBTreeSet<T> {
    type Output = Self;

    fn bitand(mut self, rhs: &InfBTreeSet<T>) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<T: Ord + Clone> BitAnd for &InfBTreeSet<T> {
    type Output = InfBTreeSet<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (InfBTreeSet::Union(lhs), InfBTreeSet::Union(rhs)) => InfBTreeSet::Union(lhs & rhs),
            (InfBTreeSet::Union(union), InfBTreeSet::Complement(complement))
            | (InfBTreeSet::Complement(complement), InfBTreeSet::Union(union)) => {
                InfBTreeSet::Complement(union - complement)
            }
            (InfBTreeSet::Complement(lhs), InfBTreeSet::Complement(rhs)) => {
                InfBTreeSet::Complement(lhs | rhs)
            }
        }
    }
}

impl<T: Ord + Clone> BitAnd<InfBTreeSet<T>> for &InfBTreeSet<T> {
    type Output = InfBTreeSet<T>;

    fn bitand(self, rhs: InfBTreeSet<T>) -> Self::Output {
        rhs & self
    }
}

impl<T: Ord + Clone> BitAndAssign for InfBTreeSet<T> {
    fn bitand_assign(&mut self, mut rhs: Self) {
        if let (InfBTreeSet::Complement(_), InfBTreeSet::Union(_)) = (&self, &rhs) {
            swap(self, &mut rhs);
        }
        match (self, rhs) {
            (Self::Union(lhs), Self::Union(rhs)) => {
                lhs.retain(|ty| rhs.contains(ty));
            }
            (Self::Union(union), Self::Complement(complement)) => {
                union.retain(|ty| !complement.contains(ty));
            }
            (Self::Complement(_), Self::Union(_)) => unreachable!(),
            (Self::Complement(lhs), Self::Complement(mut rhs)) => {
                lhs.append(&mut rhs);
            }
        }
    }
}

impl<T: Ord + Clone> BitAndAssign<&InfBTreeSet<T>> for InfBTreeSet<T> {
    fn bitand_assign(&mut self, rhs: &InfBTreeSet<T>) {
        if let (InfBTreeSet::Complement(complement), InfBTreeSet::Union(union)) = (&self, rhs) {
            let mut union = union.clone();
            union.retain(|ty| !complement.contains(ty));
            *self = Self::Union(union);
            return;
        }
        match (self, rhs) {
            (Self::Union(lhs), Self::Union(rhs)) => {
                lhs.retain(|ty| rhs.contains(ty));
            }
            (Self::Union(union), Self::Complement(complement)) => {
                union.retain(|ty| !complement.contains(ty));
            }
            (Self::Complement(_), Self::Union(_)) => unreachable!(),
            (Self::Complement(lhs), Self::Complement(rhs)) => {
                lhs.append(&mut rhs.clone());
            }
        }
    }
}
