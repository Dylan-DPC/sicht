use sicht::birelational_map::BirelationalMap;

#[test]
pub fn inserting() {
    let mut map = BirelationalMap::new();
    map.insert(10, 20);

    let value = *map.get(10).unwrap()[0];
    assert_eq!(value, 20);

    let key = *map.get_value(20).unwrap()[0];
    assert_eq!(key, 10);
}

#[test]
pub fn inserting_twice() {
    let mut map = BirelationalMap::new();
    map.insert(10, 20);
    map.insert(10, 30);

    let value = map.get(10).unwrap();
    assert_eq!(*value[0], 20);
    assert_eq!(*value[1], 30);

    let key = *map.get_value(30).unwrap()[0];
    assert_eq!(key, 10);

    let key = *map.get_value(20).unwrap()[0];
    assert_eq!(key, 10);
}

#[test]
pub fn removal() {
    let mut map = BirelationalMap::new();
    map.insert(10, 20);
    map.insert(10, 30);

    map.remove(10, 30);

    let values = map.get(10).unwrap();
    assert_eq!(values.len(), 1);
    assert_eq!(*values[0], 20);
}
