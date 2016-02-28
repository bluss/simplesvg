use std::fmt;
use std::fmt::Display;

#[test]
fn test() {
    let fig = Fig::Rect(10., 10., 200., 100.);
    let fig = Fig::Styled(Attr::default().fill(Color(0xff, 0, 0)), Box::new(fig));
    println!("{}", Svg(vec![fig], 1000, 1000));
}


const SVG_BEGIN: &'static str = r##"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg" >"##;
const SVG_END: &'static str = r##"</svg>"##;

fn replace(mut s: &str, f: &mut FnMut() -> String) -> String {
    let mut res = String::with_capacity(s.len());
    while let Some(i) = s.find("{}") {
        res.push_str(&s[..i]);
        res.push_str(&f());
        s = &s[i + 2..];
    }
    res.push_str(s);
    res
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Copy, Clone, Debug, Default)]
pub struct Attr {
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: Option<f32>,
    pub opacity: Option<f32>,
}

impl Attr {
    pub fn fill(mut self, c: Color) -> Self {
        self.fill = Some(c);
        self
    }
    pub fn stroke(mut self, c: Color) -> Self {
        self.stroke = Some(c);
        self
    }
    pub fn stroke_width(mut self, c: f32) -> Self {
        self.stroke_width = Some(c);
        self
    }
    pub fn opacity(mut self, c: f32) -> Self {
        self.opacity = Some(c);
        self
    }
}

pub struct Trans {
    pub rot: f32,
}

#[derive(Clone, Debug)]
pub enum Fig {
    Rect(f32, f32, f32, f32),
    Styled(Attr, Box<Fig>),
    Multiple(Vec<Fig>),
    //Transformed(Trans, Box<Fig>),
}

#[derive(Clone, Debug)]
pub struct Svg(pub Vec<Fig>, pub u32, pub u32);

impl Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        let head = replace(SVG_BEGIN, &mut || {
            i += 1;
            if i == 1 { self.1.to_string() } else { self.2.to_string() }
        });
        try!(writeln!(f, "{}", head));
        for elt in &self.0 {
            try!(write!(f, "{}", elt));
        }
        try!(writeln!(f, "{}", SVG_END));
        Ok(())
    }
}

impl Display for Fig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Fig::Styled(ref attr, ref fig) => {
                try!(writeln!(f, r##"<g style="{}">"##, attr));
                try!(write!(f, "{}", fig));
                writeln!(f, "</g>")
            }
            Fig::Rect(x, y, w, h) => {
                try!(writeln!(f, r#"<rect x="{}" y="{}" width="{}" height="{}" />"#,
                              x, y, w, h));
                Ok(())
            }
            Fig::Multiple(ref figs) => {
                for elt in figs {
                    try!(write!(f, "{}", elt));
                }
                Ok(())
            }
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

impl Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(c) = self.fill {
            try!(write!(f, "fill:{};", c));
        }
        if let Some(c) = self.stroke {
            try!(write!(f, "stroke:{};", c));
        }
        if let Some(v) = self.stroke_width {
            try!(write!(f, "stroke_width:{};", v));
        }
        if let Some(v) = self.opacity {
            try!(write!(f, "opacity:{};", v));
        }
        Ok(())
    }
}
