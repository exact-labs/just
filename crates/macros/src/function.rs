#[macro_export]
macro_rules! function_path {
    () => {{
        fn f() {}
        fn t<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let n = t(f);
        &n[..n.len() - 3]
    }};
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn t<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let n = t(f);
        let v = &n[..n.len() - 3];

        match &v.strip_suffix("::call") {
            Some(d) => match &d.rfind(':') {
                Some(s) => &d[s + 1..d.len()],
                None => v,
            },
            None => match v.strip_suffix("::call::{{closure}}") {
                Some(f) => match &f.rfind(':') {
                    Some(s) => &f[s + 1..f.len()],
                    None => v,
                },
                None => match &v.rfind(':') {
                    Some(s) => &v[s + 1..v.len()],
                    None => v,
                },
            },
        }
    }};
}
