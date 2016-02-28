//! Very simple drawing/diagramming library with svg output.
//!
//! Use `Fig` to build the figure and `Svg` to render the output to SVG.
//!
//! `Svg` implements `std::fmt::Display` for output purposes.
use std::fmt;
use std::fmt::Display;

#[test]
fn test() {
    let fig = Fig::Rect(10., 10., 200., 100.);
    let fig = fig.styled(Attr::default().fill(Color(0xff, 0, 0)));
    println!("{}", Svg(vec![fig], 1000, 1000));
}


/// Color
#[derive(Copy, Clone, Debug, Default)]
pub struct Color(pub u8, pub u8, pub u8);

/// Style attributes
#[derive(Clone, Debug, Default)]
pub struct Attr {
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_width: Option<f32>,
    pub opacity: Option<f32>,
    _incomplete: (),
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

/// Transformations
#[derive(Clone, Debug, Default)]
pub struct Trans {
    pub translate: Option<(f32, f32)>,
    pub rotate: Option<f32>,
    _incomplete: (),
}

impl Trans {
    pub fn translate(mut self, x: f32, y: f32) -> Self {
        self.translate = Some((x, y));
        self
    }
    pub fn rotate(mut self, x: f32) -> Self {
        self.rotate = Some(x);
        self
    }
}

/// Figure parts
#[derive(Clone, Debug)]
pub enum Fig {
    /// `x`, `y`, `width`, `height`
    Rect(f32, f32, f32, f32),
    /// Text element
    Text(f32, f32, String),
    /// With style attributes
    Styled(Attr, Box<Fig>),
    /// With transformations
    Transformed(Trans, Box<Fig>),
    /// Bunch of figure children.
    Multiple(Vec<Fig>),
    #[doc(hidden)]
    __Incomplete(()),
}

impl Fig {
    /// Apply style from `attr`.
    pub fn styled(self, attr: Attr) -> Self {
        Fig::Styled(attr, Box::new(self))
    }
    pub fn transformed(self, trans: Trans) -> Self {
        Fig::Transformed(trans, Box::new(self))
    }
}

/// SVG image object.
#[derive(Clone, Debug)]
pub struct Svg(pub Vec<Fig>, pub u32, pub u32);


impl Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, r##"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg" >"##,
                      self.1, self.1));
        for elt in &self.0 {
            try!(write!(f, "{}", elt));
        }
        try!(writeln!(f, r##"</svg>"##));
        Ok(())
    }
}

impl Display for Fig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Fig::Styled(ref attr, ref fig) => {
                try!(writeln!(f, r##"<g style="{}">"##, attr));
                try!(write!(f, "{}", fig));
                try!(writeln!(f, "</g>"));
            }
            Fig::Transformed(ref trans, ref fig) => {
                try!(writeln!(f, r##"<g transform="{}">"##, trans));
                try!(write!(f, "{}", fig));
                try!(writeln!(f, "</g>"));
            }
            Fig::Rect(x, y, w, h) => {
                try!(writeln!(f, r#"<rect x="{}" y="{}" width="{}" height="{}" />"#,
                              x, y, w, h));
            }
            Fig::Text(x, y, ref s) => {
                // FIXME: XML escape
                try!(writeln!(f, r#"<text x="{}" y="{}">{}</text>"#, x, y, s));
            }
            Fig::Multiple(ref figs) => {
                for elt in figs {
                    try!(write!(f, "{}", elt));
                }
            }
            Fig::__Incomplete(..) => unreachable!()
        }
        Ok(())
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

impl Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `;` are separators
        if let Some(c) = self.fill {
            try!(write!(f, "fill:{};", c));
        }
        if let Some(c) = self.stroke {
            try!(write!(f, "stroke:{};", c));
        }
        if let Some(v) = self.stroke_width {
            try!(write!(f, "stroke-width:{};", v));
        }
        if let Some(v) = self.opacity {
            try!(write!(f, "opacity:{};", v));
        }
        Ok(())
    }
}

impl Display for Trans {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // spaces are separators
        if let Some((x, y)) = self.translate {
            try!(write!(f, "translate({}, {}) ", x, y));
        }
        if let Some(x) = self.rotate {
            try!(write!(f, "rotate({}) ", x));
        }
        Ok(())
    }
}
