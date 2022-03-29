# Fast Hash Map
## hashmap library for rust with xxHash3 algorithm
xxhash3 is a fast hash function for strings and byte arrays which makes it suitable for use in hash tables that are expected to have a large number of collisions (e.g. for a web server cache, database and ext).

## usage
```Rust
use fast-hashmap::{HashMap, xxHash3, HashmapType};
let mut map = HashMap<String,String,HashmapType.CooCoo, HashBuilder(xxHash3)>::new();

map.insert("key1".to_string(), "value1".to_string());
map.insert("key2".to_string(), "value2".to_string());

map.get("key1");

map.remove("key1");

```