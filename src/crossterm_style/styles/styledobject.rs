use std;
use std::fmt;
use std::io::Write;
use super::super::Attribute;

use crossterm_style::{Color, ObjectStyle};

/// Struct that contains both the style and the content wits will be styled.
pub struct StyledObject<D> {
    pub object_style: ObjectStyle,
    pub content: D,
}

impl<D> StyledObject<D> {
    /// Sets the foreground of the styled object to the passed `Color`
    ///
    /// #Example
    ///
    /// ```rust
    ///    // create an styled object with the foreground color red.
    ///    let styledobject = paint("I am colored red").with(Color::Red);
    ///    // create an styled object with the foreground color blue.
    ///    let styledobject1 = paint("I am colored blue").with(Color::Blue);
    ///
    ///    // print the styledobject to see the result
    ///    println!("{}", styledobject);
    ///    println!("{}", styledobject1);
    ///    // print an styled object directly.
    ///    println!("{}", paint("I am colored green").with(Color::Green));
    /// 
    /// ```
    pub fn with(mut self, foreground_color: Color) -> StyledObject<D> {
        self.object_style = self.object_style.fg(foreground_color);
        self
    }

    /// Sets the background of the styled object to the passed `Color`
    ///
    /// #Example
    ///
    /// ```rust
    /// 
    ///    // create an styled object with the background color red.
    ///    let styledobject = paint("I am colored red").on(Color::Red);
    ///    // create an styled object with the background color blue.
    ///    let styledobject1 = paint("I am colored blue").on(Color::Blue);
    ///
    ///    // print the styledobjects
    ///    println!("{}", styledobject);
    ///    println!("{}", styledobject1);
    ///    // print an styled object directly.
    ///    println!("{}", paint("I am colored green").on(Color::Green))
    /// 
    /// ```
    pub fn on(mut self, background_color: Color) -> StyledObject<D> {
        self.object_style = self.object_style.bg(background_color);
        self
    }

    pub fn attrs(mut self, attrs: Vec<Attribute>) -> StyledObject<D>
    {
        for attr in attrs.iter() {
            self.attr(attr);
        }

        self
    }

    pub fn attr(mut self, attr: Attribute) -> StyledObject<D>
    {
        self.object_style.add_attr(attr);
        self
    }

    #[inline(always)] pub fn bold(self) -> StyledObject<D> { self.attr(Attribute::Bold) }
    #[inline(always)] pub fn dim(self) -> StyledObject<D> { self.attr(Attribute::Dim) }
    #[inline(always)] pub fn italic(self) -> StyledObject<D> { self.attr(Attribute::Italic) }
    #[inline(always)] pub fn underlined(self) -> StyledObject<D> { self.attr(Attribute::Underlined) }
    #[inline(always)] pub fn blink(self) -> StyledObject<D> { self.attr(Attribute::Blink) }
    #[inline(always)] pub fn reverse(self) -> StyledObject<D> { self.attr(Attribute::Reverse) }
    #[inline(always)] pub fn hidden(self) -> StyledObject<D> { self.attr(Attribute::Hidden) }
}

/// This is used to make StyledObject able to be displayed.
/// This macro will set the styles stored in Styled Object
macro_rules! impl_fmt
{
    ($name:ident) => {
        impl<D: fmt::$name> fmt::$name for StyledObject<D> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
            {
                let mut colored_terminal = super::super::get();
                let mut reset = true;

                if let Some(bg) = self.object_style.bg_color
                {
                    colored_terminal.set_bg(bg);
                    reset = true;
                }
                if let Some(fg) = self.object_style.fg_color
                {
                   colored_terminal.set_fg(fg);
                   reset = true;
                }

                #[cfg(unix)]
                 for attr in &self.object_style.attrs.iter() {
                    write!(f, csi!("{}m"), attr as i16);
                    reset = true;
                 }

                fmt::$name::fmt(&self.content, f)?;
                std::io::stdout().flush().expect("Flush stdout failed");

                if reset
                {
                    colored_terminal.reset();
                }

                Ok(())
            }
        }
    }
}

impl_fmt!(Debug);
impl_fmt!(Display);
