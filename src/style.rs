//! Collection of helper functions and classes related to colors
//! ```
//! use zenity::style::{Attribute, Attributes, combine_attributes, ContentStyle, StyledString};
//!
//! let attr1 = Attribute::Bold;
//! let attr2 = Attribute::Underlined;
//! let attr3 = Attribute::Italic;
//!
//! let combined_attr = combine_attributes(&[&attr1, &attr2, &attr3]);
//!
//! let style = ContentStyle {
//!     foreground_color: None,
//!     background_color: None,
//!     underline_color: None,
//!     attributes: combined_attr, // see docs on combine_attributes()
//! };
//!
//! let styled_string = StyledString {
//!     string: "example".to_string(),
//!     style,
//! };
//! # assert_eq!(styled_string.string, "example");
//! # assert_eq!(styled_string.style.attributes, combined_attr);
//! # assert_eq!(styled_string.style, style);
//! ```

pub use crossterm::style::*;

/// combines multiple attributes into a single `style::Attributes` instance
///
/// this function takes a slice of attribute references and combines them into a single
/// `style::Attributes` instance using bitwise OR (`|`) operation
///
/// # Arguments
///
/// * `attr_list` - a slice containing references to the attributes to be combined
///
/// # Returns
///
/// a `style::Attributes` instance representing the combined attributes
///
/// ```
/// use zenity::style::{Attribute, Attributes, combine_attributes};
///
/// let attr1 = Attribute::Bold;
/// let attr2 = Attribute::Underlined;
/// let attr3 = Attribute::Italic;
///
/// let combined_attr = combine_attributes(&[&attr1, &attr2, &attr3]);
/// #
/// # assert_eq!(combine_attributes(&[&attr1, &attr2, &attr3]), combined_attr);
/// ```
pub fn combine_attributes(attr_list: &[&Attribute]) -> Attributes {
    attr_list.iter().fold(Attributes::default(), |acc, &attr| {
        acc | Attributes::from(*attr)
    })
}

/// Represents the style of a string, including attributes and colors
///
/// Represents a string along with their style
///
/// The style is a representation of ``ContentStyle``
///
/// The ``ContentStyle`` is defined like this:
///
/// ```
/// # use zenity::style::{Attribute, combine_attributes};
/// # let combined_attr = combine_attributes(&[&Attribute::Bold, &Attribute::Underlined]);
/// use zenity::style::ContentStyle;
///
/// let style = ContentStyle {
///     foreground_color: None,
///     background_color: None,
///     underline_color: None,
///     attributes: combined_attr, // see docs on combine_attributes()
/// };
/// ```
///
/// for more info on combined attributes look up \[combine_attributes\]
///
///
///```
/// # use zenity::style::{Attribute, combine_attributes, ContentStyle};
/// # let combined_attr = combine_attributes(&[&Attribute::Bold, &Attribute::Underlined]);
/// # let style = ContentStyle {
/// #    foreground_color: None,
/// #    background_color: None,
/// #    underline_color: None,
/// #    attributes: combined_attr,
/// # };
/// use zenity::style::StyledString;
///
/// let styled = StyledString {
///    string: "example".to_string(),
///    style,
/// };
/// #
/// # assert_eq!(styled.string, "example");
/// # assert_eq!(styled.style.attributes, combined_attr);
/// # assert_eq!(styled.style, style);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct StyledString {
    /// the string
    pub string: String,
    /// the ContentStyle to apply to the string
    pub style: ContentStyle,
}

/// Example:
/// ```
/// use zenity::style::{ContentStyle,StyledString};
///
/// let styled_text = StyledString {
///     string: "Hello, ".to_string(),
///     style: ContentStyle::default(),
/// };
///
/// let repeated_styled_text = styled_text.repeat(3);
/// assert_eq!(repeated_styled_text.string, "Hello, Hello, Hello, ");
/// ```
impl StyledString {
    /// Repeats the StyledString `count` times
    ///
    /// Example:
    ///
    /// ```
    /// use zenity::style::{ContentStyle,StyledString};
    /// # let styled_text = StyledString {
    /// #     string: "Hello, ".to_string(),
    /// #     style: ContentStyle::default(),
    /// # };
    ///
    /// let repeated_styled_text = styled_text.repeat(3);
    /// ```
    pub fn repeat(&self, count: usize) -> StyledString {
        // repeat the string `count` times
        let repeated_string = self.string.repeat(count);
        // create a new StyledString with the repeated string and the same style
        StyledString {
            string: repeated_string,
            style: self.style, // cloning the style assuming it's a cheap operation
        }
    }
}

impl Default for StyledString {
    fn default() -> Self {
        StyledString::new("")
    }
}

/// ```
/// use zenity::style::StyledString;
///
/// let text = "Hello, world!";
/// let styled_text = StyledString::new(text);
/// ```
impl StyledString {
    /// creates a new StyledString with default arguments and parameters
    ///
    /// # Example
    ///
    /// ```
    /// use zenity::style::StyledString;
    ///
    /// let text = "Hello, world!";
    ///
    /// let styled_text = StyledString::new(text);
    /// ```
    pub fn new(string: &str) -> Self {
        StyledString {
            string: string.to_string(),
            style: ContentStyle {
                foreground_color: None,
                background_color: None,
                underline_color: None,
                attributes: combine_attributes(&[&Attribute::Reset]),
            },
        }
    }

    /// creates a new StyledString with:
    /// - foreground_color
    /// - background_color
    /// - underline_color
    /// # Example
    ///
    /// ```
    /// use zenity::style::{Color,StyledString};
    ///
    /// let text = "Hello, world!";
    /// let foreground_color = Some(Color::Red);
    /// let background_color = Some(Color::Blue);
    /// let underline_color = Some(Color::Green);
    ///
    /// let styled_text = StyledString::simple(text, foreground_color, background_color, underline_color);
    /// ```
    pub fn simple(
        string: &str,
        foreground_color: Option<Color>,
        background_color: Option<Color>,
        underline_color: Option<Color>,
    ) -> Self {
        StyledString {
            string: string.to_string(),
            style: ContentStyle {
                foreground_color,
                background_color,
                underline_color,
                attributes: combine_attributes(&[]),
            },
        }
    }
}

/// convert vec string into vec StyledString
#[macro_export]
macro_rules! styled_string {
    ($($string:expr), *) => {{
        vec![
            $(
               StyledString::new($string),
            )*
        ]
    }};
}
