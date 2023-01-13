ghost_lite::ghost! {
    #![derive(Clone, Copy, Default, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
    /// A marker type which marks data is unspecified.
    pub struct Unspecified<T: ?Sized>
}

#[cfg(test)]
mod tests {
    use super::Unspecified;

    #[test]
    fn size() {
        let _: Unspecified<usize> = Unspecified;
        assert_eq!(std::mem::size_of::<Unspecified::<usize>>(), 0);
    }
}
