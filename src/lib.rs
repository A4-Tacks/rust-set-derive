#![doc = include_str!("../README.md")]

#[doc = include_str!("../README.md")]
#[macro_export]
macro_rules! set_derive {
    ($($rest:tt)*) => {
        $crate::hashmap_chain_list!(@top $($rest)*)
    };
}

#[macro_export]
macro_rules! macro_util {
    (@if($($t:tt)+)else($($e:tt)*)) => {
        $($t)*
    };
    (@if else($($e:tt)*)) => {
        $($e)*
    };
}

#[macro_export]
macro_rules! iter_body {
    (@top $body:block for $pat:pat in $iter:expr $(; $($rest:tt)*)?) => {
        for $pat in $iter {
            $crate::iter_body!(@rest $body $($($rest)*)?);
        }
    };
    (@top $body:block $code:block $($rest:tt)*) => {
        $code
        $crate::iter_body!(@rest $body $($rest)*);
    };
    (@rest $body:block $code:block $($rest:tt)*) => {
        $code
        $crate::iter_body!(@rest $body $($rest)*);
    };
    (@rest $body:block for $pat:pat in $iter:expr $(; $($rest:tt)*)?) => {
        for $pat in $iter {
            $crate::iter_body!(@rest $body $($($rest)*)?);
        }
    };
    (@rest $body:block if $cond:expr $(; $($rest:tt)*)?) => {
        if $cond {
            $crate::iter_body!(@rest $body $($($rest)*)?);
        }
    };
    (@rest $body:block if let $pat:pat = $expr:expr $(; $($rest:tt)*)?) => {
        if let $pat = $expr {
            $crate::iter_body!(@rest $body $($($rest)*)?);
        }
    };
    (@rest $body:block) => {
        $body
    };
}

#[macro_export]
macro_rules! hashmap_elem_size {
    (@rest $map:ident) => {0usize};
    (@rest $map:ident *$([$cap:expr])?
        ($key:expr => $value:expr; $($body:tt)*)
        $(, $($rest:tt)*)?
    ) => {
        $crate::macro_util!(@if$(($cap))?else(0))
            + $crate::hashmap_elem_size!(@rest $map $($($rest)*)?)
    };
    (@rest $map:ident $key:ident : $value:expr $(, $($rest:tt)*)?) => {
        1 + $crate::hashmap_elem_size!(@rest $map $($($rest)*)?)
    };
    (@rest $map:ident %$key:ident : $value:expr $(, $($rest:tt)*)?) => {
        1 + $crate::hashmap_elem_size!(@rest $map $($($rest)*)?)
    };
    (@rest $map:ident *$key:ident : $value:expr $(, $($rest:tt)*)?) => {
        1 + $crate::hashmap_elem_size!(@rest $map $($($rest)*)?)
    };
    (@rest $map:ident $key:literal : $value:expr $(, $($rest:tt)*)?) => {
        1 + $crate::hashmap_elem_size!(@rest $map $($($rest)*)?)
    };
    (@rest $map:ident [$key:expr] : $value:expr $(, $($rest:tt)*)?) => {
        1 + $crate::hashmap_elem_size!(@rest $map $($($rest)*)?)
    };
    (@rest $map:ident $key:expr => $value:expr $(, $($rest:tt)*)?) => {
        1 + $crate::hashmap_elem_size!(@rest $map $($($rest)*)?)
    };
}

#[macro_export]
macro_rules! hashmap_chain_list {
    (@top $(<$($ty:ty),+$(,)?>)? ($cap:expr) {$($rest:tt)*}) => {{
        let mut __map = ::std::collections
            ::HashMap::$(<$($ty),+>::)?with_capacity($cap);
        $crate::hashmap_chain_list!(@rest __map $($rest)*);
        __map
    }};
    (@top $(<$($ty:ty),+$(,)?>)? {$($rest:tt)*}) => {{
        let mut __map = ::std::collections
            ::HashMap::$(<$($ty),+>::)?with_capacity(
                $crate::hashmap_elem_size!(@rest __map $($rest)*)
            );
        $crate::hashmap_chain_list!(@rest __map $($rest)*);
        __map
    }};
    (@top $($rest:tt)*) => {{
        $crate::list_chain_hashset!(@top $($rest)*)
    }};
    (@rest $map:ident) => {};
    (@rest $map:ident *$([$cap:expr])?
        ($key:expr => $value:expr; $($body:tt)*)
        $(, $($rest:tt)*)?
    ) => {
        $($map.reserve($cap);)?
        $crate::iter_body!(@top {
            $map.insert($key, $value);
        } $($body)*);
        $crate::hashmap_chain_list!(@rest $map $($($rest)*)?);
    };
    (@rest $map:ident $key:ident : $value:expr $(, $($rest:tt)*)?) => {
        $map.insert(::std::stringify!($key), $value);
        $crate::hashmap_chain_list!(@rest $map $($($rest)*)?);
    };
    (@rest $map:ident %$key:ident : $value:expr $(, $($rest:tt)*)?) => {
        $map.insert(::std::string::String::from(::std::stringify!($key)), $value);
        $crate::hashmap_chain_list!(@rest $map $($($rest)*)?);
    };
    (@rest $map:ident *$key:ident : $value:expr $(, $($rest:tt)*)?) => {
        $map.insert($key, $value);
        $crate::hashmap_chain_list!(@rest $map $($($rest)*)?);
    };
    (@rest $map:ident $key:literal : $value:expr $(, $($rest:tt)*)?) => {
        $map.insert($key, $value);
        $crate::hashmap_chain_list!(@rest $map $($($rest)*)?);
    };
    (@rest $map:ident [$key:expr] : $value:expr $(, $($rest:tt)*)?) => {
        $map.insert($key, $value);
        $crate::hashmap_chain_list!(@rest $map $($($rest)*)?);
    };
    (@rest $map:ident $key:expr => $value:expr $(, $($rest:tt)*)?) => {
        $map.insert($key, $value);
        $crate::hashmap_chain_list!(@rest $map $($($rest)*)?);
    };
}

#[macro_export]
macro_rules! hashset_and_list_elem_size {
    (@rest $map:ident) => {0usize};
    (@rest $map:ident *$([$cap:expr])?
        ($value:expr; $($body:tt)*)
        $(, $($rest:tt)*)?
    ) => {
        $crate::macro_util!(@if$(($cap))?else(0))
            + $crate::hashset_and_list_elem_size!(@rest $map $($($rest)*)?)
    };
    (@rest $map:ident $value:expr $(, $($rest:tt)*)?) => {
        1 + $crate::hashset_and_list_elem_size!(@rest $map $($($rest)*)?)
    };
}

#[macro_export]
macro_rules! list_chain_hashset {
    (@top $(<$($ty:ty),+$(,)?>)? ($cap:expr) [$($rest:tt)*]) => {{
        let mut __map = ::std::vec::Vec::$(<$($ty),+>::)?with_capacity($cap);
        $crate::list_chain_hashset!(@rest __map $($rest)*);
        __map
    }};
    (@top $(<$($ty:ty),+$(,)?>)? [$($rest:tt)*]) => {{
        let mut __map = ::std::vec::Vec::$(<$($ty),+>::)?with_capacity(
            $crate::hashset_and_list_elem_size!(@rest __map $($rest)*)
        );
        $crate::list_chain_hashset!(@rest __map $($rest)*);
        __map
    }};
    (@top $($rest:tt)*) => {
        $crate::hashset!(@top $($rest)*)
    };
    (@rest $map:ident) => {};
    (@rest $map:ident *$([$cap:expr])?
        ($value:expr; $($body:tt)*)
        $(, $($rest:tt)*)?
    ) => {
        $($map.reserve($cap);)?
        $crate::iter_body!(@top {
            $map.push($value);
        } $($body)*);
        $crate::list_chain_hashset!(@rest $map $($($rest)*)?);
    };
    (@rest $map:ident $value:expr $(, $($rest:tt)*)?) => {
        $map.push($value);
        $crate::list_chain_hashset!(@rest $map $($($rest)*)?);
    };
}

#[macro_export]
macro_rules! hashset {
    (@top $(<$($ty:ty),+$(,)?>)? $(($cap:expr))? !{$($rest:tt)*}) => {{
        let mut __map = ::std::collections::HashSet::$(<$($ty),+>::)?with_capacity(
            $crate::macro_util!(@if$(($cap))?else(
                $crate::hashset_and_list_elem_size!(@rest __map $($rest)*)
            ))
        );
        $crate::hashset!(@rest __map $($rest)*);
        __map
    }};
    (@rest $map:ident) => {};
    (@rest $map:ident *$([$cap:expr])?
        ($value:expr; $($body:tt)*)
        $(, $($rest:tt)*)?
    ) => {
        $($map.reserve($cap);)?
        $crate::iter_body!(@top {
            $map.insert($value);
        } $($body)*);
        $crate::hashset!(@rest $map $($($rest)*)?);
    };
    (@rest $map:ident $value:expr $(, $($rest:tt)*)?) => {
        $map.insert($value);
        $crate::hashset!(@rest $map $($($rest)*)?);
    };
}

#[cfg(test)]
mod tests;
