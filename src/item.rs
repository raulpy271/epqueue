
use std::cmp;

#[derive(Debug, Clone)]
pub struct Item<'a, K: Copy + cmp::Ord + cmp::Eq, V: Clone> {
    pub key: K,
    pub value: &'a V,
}

impl<'a, K: Copy + cmp::Ord + cmp::Eq, V: Clone> cmp::Ord for Item<'a, K, V> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<'a, K: Copy + cmp::Ord + cmp::Eq, V: Clone> cmp::PartialOrd for Item<'a, K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<'a, K: Copy + cmp::Ord + cmp::Eq, V: Clone> cmp::PartialEq for Item<'a, K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<'a, K: Copy + cmp::Ord + cmp::Eq, V: Clone> cmp::Eq for Item<'a, K, V> { }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_ordering() {
        let value = String::from("item!");
        let item1 = Item {
            key: 2,
            value: &value,
        };
        let item2 = Item {
            key: 3,
            value: &value,
        };
        assert_eq!(item1.cmp(&item2), cmp::Ordering::Less);
        assert_eq!(item2.cmp(&item1), cmp::Ordering::Greater);
        assert_eq!(item2.cmp(&item2), cmp::Ordering::Equal);
    }

    #[test]
    fn item_equality() {
        let value1 = String::from("item1!");
        let value2 = String::from("item2!");
        let item1 = Item {
            key: 2,
            value: &value1,
        };
        let item2 = Item {
            key: 2,
            value: &value2,
        };
        assert_eq!(item1 == item2, true);
    }
}