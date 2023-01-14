/// Similar to the [`From`] trait, except `T` is treated as the complement of a set.
pub trait FromComplement<T>: Sized {
    /// Converts to this type from the input type, which is treated as the complement of a set.
    fn from_complement(value: T) -> Self;
}
