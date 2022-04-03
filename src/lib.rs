mod robinHood;


#[cfg(test)]
mod tests {
    use crate::robinHood::RobinHoodHashMap;
    
    #[test]
    fn create_new_hashmap() {
        RobinHoodHashMap::<String,String>::new(10);
        assert!(true);
    }

    #[test]
    fn insert_into_hashmap() {
        let mut map = RobinHoodHashMap::<String,String>::new(10);
        map.insert("key".to_string(),"value".to_string());
        assert!(true);
    }
}
