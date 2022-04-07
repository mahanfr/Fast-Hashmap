mod robinHood;
mod LinkedListHash;

#[cfg(test)]
mod tests {
    use crate::robinHood::RobinHoodHashMap;
    use crate::LinkedListHash::LinkedListHashMap;
    #[test]
    fn robin_hood_create_new_hashmap() {
        RobinHoodHashMap::<String,String>::new(10);
        assert!(true);
    }

    #[test]
    fn robin_hood_insert_into_hashmap() {
        let mut map = RobinHoodHashMap::<String,String>::new(10);
        map.insert("key".to_string(),"value".to_string());
        assert!(true);
    }

    fn linked_create_new_hashmap() {
        LinkedListHashMap::<String,String>::new(10);
        assert!(true);
    }

    #[test]
    fn linked_insert_into_hashmap() {
        let mut map = LinkedListHashMap::<String,String>::new(10);
        map.insert("key".to_string(),"value".to_string());
        assert!(true);
    }

}
