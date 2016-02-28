//! Very simple drawing/diagramming library with svg output.
//!
//! Use `Fig` to build the figure and `Svg` to render the output to SVG.
//!
//! `Svg` implements `std::fmt::Display` for output purposes.
#![warn(variant_size_differences)]
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

#[test]
fn test() {
    let fig = Fig::Rect(10., 10., 200., 100.)
            .styled(Attr::default().fill(Color(0xff, 0, 0)));
    let text = Fig::Text(0., 20., "<XML & Stuff>".to_string());
    let c = Fig::Circle(20., 20., 100.);
    println!("{}", Svg(vec![fig, text, c], 500, 600));
}

#[test]
fn koch() {
    // Koch snowflake fractal
    let w = 500.;
    let mut fig = Fig::Line(0., 0., w, 0.);
    for _ in 0..5 {
        let f = fig.shared();
        let mut v = Vec::new();
        v.push(f.clone());
        v.push(f.clone().transformed(Trans::default().translate(w, 0.).rotate(60.)));
        v.push(f.clone().transformed(Trans::default().translate(2. * w, 0.).rotate(120.).scale_x_y(1., -1.)));
        v.push(f.clone().transformed(Trans::default().translate(2. * w, 0.)));
        fig = Fig::Multiple(v).transformed(Trans::default().scale(0.333));
    }
    fig = fig.styled(Attr::default().stroke(Color::default()).stroke_width(100.));
    println!("{}", Svg(vec![fig], w as u32, w as u32));
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
    pub font_family: Option<&'static str>,
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
    pub fn font_family(mut self, c: &'static str) -> Self {
        self.font_family = Some(c);
        self
    }
}

/// Transformations
#[derive(Clone, Debug, Default)]
pub struct Trans {
    pub translate: Option<(f32, f32)>,
    pub rotate: Option<f32>,
    pub scale: Option<(f32, f32)>,
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
    pub fn scale(mut self, x: f32) -> Self {
        self.scale = Some((x, x));
        self
    }

    pub fn scale_x_y(mut self, x: f32, y: f32) -> Self {
        self.scale = Some((x, y));
        self
    }
}

/// Figure parts
#[derive(Clone, Debug)]
pub enum Fig {
    /// `x`, `y`, `width`, `height`
    Rect(f32, f32, f32, f32),
    /// `cx`, `cy`, `radius`
    Circle(f32, f32, f32),
    /// `cx`, `cy`, `rx`, `ry`
    Ellipse(f32, f32, f32, f32),
    /// `x1`, `y1`, `x2`, `y2`
    Line(f32, f32, f32, f32),
    /// Text element, `x`, `y`, `text`
    Text(f32, f32, String),
    /// With style attributes
    Styled(Attr, Box<Fig>),
    /// With transformations
    Transformed(Trans, Box<Fig>),
    /// A bunch of figure parts
    Multiple(Vec<Fig>),
    /// Shared figure part.
    Shared(Rc<Fig>),
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

    pub fn shared(self) -> Self {
        if let Fig::Shared(_) = self {
            self
        } else {
            Fig::Shared(Rc::new(self))
        }
    }
}

/// SVG image object.
#[derive(Clone, Debug)]
pub struct Svg(pub Vec<Fig>, pub u32, pub u32);


impl Display for Svg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, r##"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"##,
                      self.1, self.2));
        for elt in &self.0 {
            try!(write!(f, "{}", elt));
        }
        try!(writeln!(f, r##"</svg>"##));
        Ok(())
    }
}

struct XmlEscape<'a>(&'a str);

impl<'a> Display for XmlEscape<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0;
        for ch in s.chars() {
            match ch {
                '<' => try!(write!(f, "&lt;")),
                '>' => try!(write!(f, "&gt;")),
                '&' => try!(write!(f, "&amp;")),
                c => try!(write!(f, "{}", c)),
            }
        }
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
                try!(writeln!(f, r#"<rect x="{}" y="{}" width="{}" height="{}"/>"#,
                              x, y, w, h));
            }
            Fig::Line(x1, y1, x2, y2) => {
                try!(writeln!(f, r#"<line x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
                              x1, y1, x2, y2));
            }
            Fig::Circle(x, y, r) => {
                try!(writeln!(f, r#"<circle x="{}" y="{}" r="{}"/>"#, x, y, r));
            }
            Fig::Ellipse(x, y, rx, ry) => {
                try!(writeln!(f, r#"<ellipse x="{}" y="{}" rx="{}" ry="{}"/>"#,
                              x, y, rx, ry));
            }
            Fig::Text(x, y, ref s) => {
                try!(writeln!(f, r#"<text x="{}" y="{}">{}</text>"#,
                              x, y, XmlEscape(s)));
            }
            Fig::Multiple(ref figs) => {
                for elt in figs {
                    try!(write!(f, "{}", elt));
                }
            }
            Fig::Shared(ref fig) => {
                try!(write!(f, "{}", **fig));
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
        if let Some(v) = self.font_family {
            try!(write!(f, "font-family:{};", v));
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
        if let Some((x, y)) = self.scale {
            if x == y {
                try!(write!(f, "scale({}) ", x));
            } else {
                try!(write!(f, "scale({}, {}) ", x, y));
            }
        }
        Ok(())
    }
}
