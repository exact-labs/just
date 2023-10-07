#[macro_export]
macro_rules! inc {
    ($id:expr) => {{
        let _rv = $id;
        $id += 1;
        _rv
    }};
}

#[macro_export]
macro_rules! dec {
    ($id:expr) => {{
        let _rv = $id;
        $id -= 1;
        _rv
    }};
}

#[macro_export]
macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}

#[macro_export]
macro_rules! then {
    ($c:expr, $v:expr) => {
        if $c {
            $v
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! attempt {
   (@recurse ($a:expr) { } catch ($e:ident) $b:block) => {
      if let Err ($e) = $a $b
   };
   (@recurse ($a:expr) { $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      attempt!{@recurse ($a.and_then (|_| $e)) { $($tail)* } $($handler)*}
   };
   ({ $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      attempt!{@recurse ($e) { $($tail)* } $($handler)* }
   };
}
