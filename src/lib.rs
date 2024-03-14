pub mod app;
pub mod tui;

mod character;

pub mod utils {
    use std::ops::{Index, IndexMut};

    use enum_map::{EnumArray, EnumMap, Iter, IterMut};

    pub trait EnumMapWrapper<'a, K: EnumArray<V> + 'a, V> {
        fn map(&self) -> &EnumMap<K, V>;

        fn iter(&'a self) -> Iter<'a, K, V> {
            self.map().iter()
        }
    }

    #[derive(Debug)]
    pub struct MyEnumMap<K: EnumArray<V>, V>(EnumMap<K, V>);

    impl<K: EnumArray<V>, V: Default> Default for MyEnumMap<K, V> {
        fn default() -> Self {
            Self(Default::default())
        }
    }

    impl<K: EnumArray<V>, V> IndexMut<K> for MyEnumMap<K, V> {
        fn index_mut(&mut self, index: K) -> &mut Self::Output {
            &mut self.0[index]
        }
    }

    impl<K: EnumArray<V>, V> Index<K> for MyEnumMap<K, V> {
        type Output = V;

        fn index(&self, index: K) -> &Self::Output {
            &self.0[index]
        }
    }

    impl<K: EnumArray<V>, V> From<EnumMap<K, V>> for MyEnumMap<K, V> {
        fn from(value: EnumMap<K, V>) -> Self {
            Self(value)
        }
    }

    impl<'a, K: EnumArray<V>, V> MyEnumMap<K, V> {
        pub fn iter(&'a self) -> Iter<'a, K, V> {
            self.0.iter()
        }

        pub fn iter_mut(&'a mut self) -> IterMut<'a, K, V> {
            self.0.iter_mut()
        }
    }
}
