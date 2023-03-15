use syn::{Item, ForeignItem};

fn main() {
    let file_content = std::fs::read_to_string("../_bindings.rs").unwrap();
    let syntax = syn::parse_file(&file_content).unwrap();
    for i in syntax.items {
        if let Item::ForeignMod(m) = i {
            for i in m.items {
                match i {
                    ForeignItem::Fn(f) => {
                        println!("    fn {}();", f.sig.ident);
                    },
                    ForeignItem::Static(s) => {
                        println!("    static {};", s.ident);
                    }
                    _ => {},
                }
            }
        }
    }
}
