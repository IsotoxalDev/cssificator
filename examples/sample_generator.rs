use cssificator::{Style, CSS};

fn main() {
    let mut css = CSS::new();
    let mut st1 = Style::new("h1");
    st1.add_declaration("color", "red");
    st1.add_declaration("size", "20px");
    let mut st2 = Style::new("div");
    st2.add_declaration("background-color", "black");
    st2.add_declaration("height", "100vh");
    st2.add_declaration("width", "100vw");
    css.add_style(st1);
    css.add_style(st2);
    println!("{}", css)
}
