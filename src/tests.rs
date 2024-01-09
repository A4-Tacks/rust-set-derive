use super::*;
use std::collections::{HashSet,HashMap};

#[test]
fn iter_body_test() {
    let mut arr = vec![];
    iter_body!(@top {arr.push(i)} for i in 0..3);
    iter_body!(@top {arr.push(i)} for i in 0..3;);
    iter_body!(@top {arr.push(i)} for i in 0..3; if i != 2);
    iter_body!(@top {arr.push(i)} for i in 0..3; if i != 2;);
    iter_body!(@top {arr.push(j)} for i in 0..3; for j in 0..2; if i != 2);
    iter_body!(@top {arr.push(j)} for i in 0..3; for j in 0..2; if i != 2;);
    iter_body!(@top {arr.push(i)} for i in 0..3; if i != 2; for _ in 0..2);
    iter_body!(@top {arr.push(j)} for i in 0..3; for j in 0..2; if let 2 = i);
    iter_body!(@top {arr.push(j)} for i in 0..3; {assert!(i < 3)} for j in 0..2; if let 2 = i;);
    assert_eq!(arr, [
        0, 1, 2,
        0, 1, 2,
        0, 1,
        0, 1,
        0, 1, 0, 1,
        0, 1, 0, 1,
        0, 0, 1, 1,
        0, 1,
        0, 1,
    ]);
}

#[test]
fn hashmap_test() {
    let map: HashMap<(), ()>
        = hashmap_chain_list!(@top {});
    assert!(map.is_empty());

    let map = hashmap_chain_list!(@top <(), ()>{});
    assert!(map.is_empty());

    let map = hashmap_chain_list!(@top <(), (),>{});
    assert!(map.is_empty());

    let key_c = "c";
    let map = hashmap_chain_list!(@top {
        a: 2,
        b: 3,
        *key_c: 4,
    });
    assert_eq!(map, {
        let mut map = HashMap::new();
        map.insert("a", 2);
        map.insert("b", 3);
        map.insert(key_c, 4);
        map
    });

    let map = hashmap_chain_list!(@top {
        a: 2,
        b: 3,
    });
    assert_eq!(map, {
        let mut map = HashMap::new();
        map.insert("a", 2);
        map.insert("b", 3);
        map
    });

    let map = hashmap_chain_list!(@top {
        %a: 2,
        *(i.to_string() => i;
            for i in 0..10;
        )
    });
    assert_eq!(map, {
        let mut map = HashMap::new();
        map.insert("a".into(), 2);
        for i in 0..10 {
            map.insert(i.to_string(), i);
        }
        map
    });

    let map = hashmap_chain_list!(@top {
        %a: 2,
        *(i.to_string() => i;
            for i in 0..10;
        ),
        ["b".into()]: -1,
    });
    assert_eq!(map, {
        let mut map = HashMap::new();
        map.insert("a".into(), 2);
        for i in 0..10 {
            map.insert(i.to_string(), i);
        }
        map.insert("b".into(), -1);
        map
    });

    let map = hashmap_chain_list!(@top {
        %"a": 2,
        *[10](i.to_string() => i;
            for i in 0..10;
        ),
        %['b']: -1,
    });
    assert_eq!(map, {
        let mut map = HashMap::new();
        map.insert("a".into(), 2);
        for i in 0..10 {
            map.insert(i.to_string(), i);
        }
        map.insert("b".into(), -1);
        map
    });

    let arr: Vec<i32> = hashmap_chain_list!(@top []);
    assert_eq!(arr, vec![]);

    let arr: Vec<i32> = hashmap_chain_list!(@top [1, 2]);
    assert_eq!(arr, vec![1, 2]);

    let arr: Vec<i32> = hashmap_chain_list!(@top (10)[1, 2]);
    assert_eq!(arr, vec![1, 2]);

    let set: HashSet<i32> = hashmap_chain_list!(@top (10)!{1, 2});
    assert_eq!(set, HashSet::from([1, 2]));
}

#[test]
fn hashmap_elem_size_test() {
    let size = hashmap_elem_size!(@rest x);
    assert_eq!(size, 0);

    let size = hashmap_elem_size!(@rest x a: 2);
    assert_eq!(size, 1);

    let size = hashmap_elem_size!(@rest x a: 2,);
    assert_eq!(size, 1);

    let size = hashmap_elem_size!(@rest x a: 2, b: 3);
    assert_eq!(size, 2);

    let size = hashmap_elem_size!(@rest x *a: 2, %b: 3);
    assert_eq!(size, 2);

    let size = hashmap_elem_size!(@rest x *a: 2, %b: 3, *(x=>y;));
    assert_eq!(size, 2);

    let size = hashmap_elem_size!(@rest x *a: 2, %b: 3, *[3](x=>y;));
    assert_eq!(size, 5);
}

#[test]
fn hashset_test() {
    let map: HashSet<i32> = hashset!(@top !{});
    assert!(map.is_empty());

    let map = hashset!(@top <i32>!{});
    assert!(map.is_empty());

    let map = hashset!(@top !{1});
    assert_eq!(map.len(), 1);
    assert!(map.contains(&1));

    let map = hashset!(@top !{1, 2, 3});
    assert_eq!(map.len(), 3);

    let map = hashset!(@top !{1, 2, 3,});
    assert_eq!(map, HashSet::from([1, 2, 3]));

    let map = hashset!(@top (100)!{1, 2, 3});
    assert!(map.capacity() > 100);

    let map = hashset!(@top <i32>(100)!{1, 2, 3});
    assert!(map.capacity() > 100);

    let map = hashset!(@top !{1, *[100](i; for i in 2..5), 5});
    assert_eq!(map, HashSet::from([1, 2, 3, 4, 5]));
    assert!(map.capacity() > 102);
}

#[test]
fn list_test() {
    let arr = list_chain_hashset!(@top <i32>[]);
    assert!(arr.is_empty());

    let arr = list_chain_hashset!(@top <i32>(10)[]);
    assert!(arr.is_empty());

    let arr: Vec<i32> = list_chain_hashset!(@top []);
    assert!(arr.is_empty());

    let arr: Vec<i32> = list_chain_hashset!(@top (10)[]);
    assert!(arr.capacity() >= 10);

    let arr = list_chain_hashset!(@top <i32>(10)[]);
    assert!(arr.is_empty());

    let arr = list_chain_hashset!(@top [1]);
    assert_eq!(arr, vec![1]);

    let arr = list_chain_hashset!(@top [1,]);
    assert_eq!(arr, vec![1]);

    let arr = list_chain_hashset!(@top [1, 2]);
    assert_eq!(arr, vec![1, 2]);

    let arr = list_chain_hashset!(@top [1, *(i; for i in 2..4), 4]);
    assert_eq!(arr, vec![1, 2, 3, 4]);

    // to hashset

    let map = list_chain_hashset!(@top !{1});
    assert_eq!(map.len(), 1);
    assert!(map.contains(&1));

    let map = list_chain_hashset!(@top !{1, 2, 3});
    assert_eq!(map.len(), 3);

    let map = list_chain_hashset!(@top !{1, 2, 3,});
    assert_eq!(map, HashSet::from([1, 2, 3]));

    let map = list_chain_hashset!(@top (100)!{1, 2, 3});
    assert!(map.capacity() > 100);

    let map = list_chain_hashset!(@top !{1, *[100](i; for i in 2..5), 5});
    assert_eq!(map, HashSet::from([1, 2, 3, 4, 5]));
    assert!(map.capacity() > 102);
}

#[test]
fn hashset_and_list_elem_size_test() {
    let size = hashset_and_list_elem_size!(@rest x);
    assert_eq!(size, 0);

    let size = hashset_and_list_elem_size!(@rest x a);
    assert_eq!(size, 1);

    let size = hashset_and_list_elem_size!(@rest x a,);
    assert_eq!(size, 1);

    let size = hashset_and_list_elem_size!(@rest x a, b);
    assert_eq!(size, 2);

    let size = hashset_and_list_elem_size!(@rest x a, b, *(x;));
    assert_eq!(size, 2);

    let size = hashset_and_list_elem_size!(@rest x a, b, *[3](x;));
    assert_eq!(size, 5);
}

#[test]
fn set_derive_test() {
    let arr = set_derive!([1, 2, 3, 4, 5, 6]);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

    let arr = set_derive!([1, *(i; for i in 2..6), 6]);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

    let set = set_derive!(!{1, *(i; for i in 2..6), 6});
    assert_eq!(set, HashSet::from([1, 2, 3, 4, 5, 6]));

    let map = set_derive!({
        name: "jack".into(),
        "age": 18.to_string(),
    });
    assert_eq!(map, HashMap::from([
            ("name", "jack".into()),
            ("age", 18.to_string()),
    ]));

    let a_key = "a".to_string();
    let b_key = "b".to_string();
    let c_key = "c".to_string();
    let map1 = {
        let mut map = ::std::collections::HashMap::with_capacity(4);
        map.insert(a_key.clone(), ());
        map.insert(b_key.clone(), ());
        map.insert(c_key.clone(), ());
        map.insert(String::from("d"), ());
        map
    };
    let map: HashMap<String, ()> = set_derive!({
        *a_key: (),
        [b_key]: (),
        c_key => (),
        %d: (),
    });
    assert_eq!(map, map1);

    let map = set_derive!({
        %fst: 0,
        *[4](format!("k{i}") => i; for i in 1..5),
    });
    assert_eq!(map, {
        let mut map = ::std::collections::HashMap::with_capacity(4);
        map.insert("fst".into(), 0);
        map.insert("k1".into(), 1);
        map.insert("k2".into(), 2);
        map.insert("k3".into(), 3);
        map.insert("k4".into(), 4);
        map
    });
}
