/// Macro if
/// # Examples
/// ```
/// use set_derive::macro_if;
/// assert_eq!(macro_if!{if let a = t:(1) {1} else {2}}, 1);
/// assert_eq!(macro_if!{if let a = {1} else {2}}, 2);
///
/// assert_eq!(macro_if!{if {1} else {2}}, 1);
/// assert_eq!(macro_if!{if else {2}}, 2);
/// ```
#[macro_export]
macro_rules! macro_if {
    (if let $i:ident = t:($x:expr) $tb:block else $fb:block) => {{
        let $i = $x;
        $tb
    }};
    (if let $i:ident = $tb:block else $fb:block) => {{
        $fb
    }};
    (if $tb:block else $fb:block) => {{ $tb }};
    (if else $fb:block) => {{ $fb }};
}


/// Please do not use this macro. It is a dependency of other macros.
#[macro_export]
macro_rules! body {
    (($lab:tt => $res:tt.$f:tt($($exp:expr),+))) => ($res.$f($($exp),+));
    (($lab:tt => $res:tt.$f:tt($($exp:expr),+)) if( $($t:tt)+ ) $($other:tt)*) => {
        if $($t)+ {
            $crate::body!(($lab => $res.$f($($exp),+)) $($other)* );
        } else {
            continue $lab;
        }
    };
    (($lab:tt => $res:tt.$f:tt($($exp:expr),+)) for( $($t:tt)+ ) $($other:tt)*) => {
        for $($t)+ {
            $crate::body!(($lab => $res.$f($($exp),+)) $($other)* );
        }
    };
    (($lab:tt => $res:tt.$f:tt($($exp:expr),+)) $block:block $($other:tt)*) => {
        $block;
        $crate::body!(($lab => $res.$f($($exp),+)) $($other)* );
    };
}


/// You can use this macro to implement the comprehensions
/// # Examples
/// ```
/// # use set_derive::*;
/// let mut result = Vec::new();
/// head!((result.push(i)) for(i in 0..5) if(i & 1 == 0));
/// assert_eq!(result, vec![0, 2, 4]);
/// ```
#[macro_export]
macro_rules! head {
    (($res:ident.$f:tt($($exp:expr),+)) for( $($t:tt)+ ) $($other:tt)*) => {
        #[allow(unused_labels)]
        'set_derive_top: for $($t)+ {
            #[warn(unused_labels)]
            {
                $crate::body!(('set_derive_top => $res.$f($($exp),+)) $($other)* );
            }
        }
    };
}


/// Comprehensions encapsulation of `Vec [v]`, `HashMap {k=>v}`, `HashSet {k}`
/// # Examples
/// ```
/// use set_derive::*;
/// # use std::{iter::zip,collections::{HashMap,HashSet}};
///
/// assert_eq!(set_derive!((8)[1; for(_ in 0..1)]).capacity(), 8);
///
/// let a = set_derive!(<i64>[i; for(i in 0..5) if(i & 1 == 0)]);
/// assert_eq!(a, vec![0, 2, 4]);
///
/// let a = set_derive!(<i16>(30)[j; for(i in 0..5) if(i & 1 == 0)
///                             for(j in 0..=i)]);
/// assert_eq!(a.capacity(), 30);
/// assert_eq!(a, vec![0,0,1,2,0,1,2,3,4]);
///
/// let a = set_derive!({j; for(i in 0..5) if(i & 1 == 0)
///                             for(j in 0..=i)});
/// assert_eq!(a, HashSet::from_iter(0..5));
///
/// let a = set_derive!({j => j + 1; for(i in 0..5)
///                         if(i & 1 == 0) for(j in 0..=i)});
/// assert_eq!(a, HashMap::from_iter(zip(0..5, 1..6)));
///
/// let a = set_derive!([i; for(mut i in 0..5) { i += 1; }]);
/// assert_eq!(a, vec![1, 2, 3, 4, 5]);
/// ```
#[macro_export]
macro_rules! set_derive {
    ($(<$($ty:ty),+$(,)?>)? $(($cap:expr))?[$exp:expr; $($t:tt)+]) => {{
        let mut _macro_set_derive_result = $crate::macro_if!{
            if let cap = $(t:($cap))? {
                ::std::vec::Vec::$(<$($ty),+>::)?with_capacity(cap)
            } else {
                ::std::vec::Vec::$(<$($ty),+>::)?new()
            }
        };
        $crate::head!((_macro_set_derive_result.push($exp)) $($t)+);
        _macro_set_derive_result
    }};
    ($(<$($ty:ty),+$(,)?>)? $(($cap:expr))?{$exp:expr; $($t:tt)+}) => {{
        let mut _macro_set_derive_result = $crate::macro_if!{
            if let cap = $(t:($cap))? {
                ::std::collections::HashSet::$(<$($ty),+>::)?with_capacity(cap)
            } else {
                ::std::collections::HashSet::$(<$($ty),+>::)?new()
            }
        };
        $crate::head!((_macro_set_derive_result.insert($exp)) $($t)+);
        _macro_set_derive_result
    }};
    ($(<$($ty:ty),+$(,)?>)? $(($cap:expr))?{$k:expr => $v:expr; $($t:tt)+}) => {{
        let mut _macro_set_derive_result = $crate::macro_if!{
            if let cap = $(t:($cap))? {
                ::std::collections::HashMap::$(<$($ty),+>::)?with_capacity(cap)
            } else {
                ::std::collections::HashMap::$(<$($ty),+>::)?new()
            }
        };
        $crate::head!((_macro_set_derive_result.insert($k, $v)) $($t)+);
        _macro_set_derive_result
    }};
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, vec, collections::HashMap};

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        assert_eq!(set_derive!([i; for(i in 0..5)]), Vec::from_iter(0..5));

        let arr = set_derive!([i; for(mut i in 0..5) { i += 1; }]);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        let arr = set_derive!([i; for(mut i in 0..8) { i += 1; } if(i & 1 == 0)]);
        assert_eq!(arr, vec![2, 4, 6, 8]);

        let dict = set_derive!({i => i + 1; for(i in 0..6)});
        assert_eq!(dict, HashMap::from_iter((0..6).map(|i| (i, i + 1))));
    }
}
