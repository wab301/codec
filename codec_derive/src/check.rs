use syn::Path;

pub fn is_number(path: &Path) -> bool {
    if path.is_ident("f32") ||
        path.is_ident("f64") ||
        path.is_ident("i16") ||
        path.is_ident("i32") ||
        path.is_ident("i64") ||
        path.is_ident("u16") ||
        path.is_ident("u32") ||
        path.is_ident("u64") {
       true
    } else {
        false
    }
}