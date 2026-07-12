//! Path manipulation.
//!
//! This module provides types and utilities for parsing, manipulating, and
//! formatting file system paths. Paths may optionally begin with a root
//! predicate (e.g., `home:/Documents/report.docx`). If no predicate is present,
//! the path is considered relative to the current root.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use core::ops::Deref;

/// A slice of a path (analogous to `str`).
#[status(stable)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Path
{
    inner: str,
}

/// An owned, mutable path (analogous to `String`).
#[status(stable)]
#[derive(Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PathBuf
{
    inner: String,
}

/// A single component of a path.
#[status(stable)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component<'a>
{
    /// A root predicate (e.g., `home` from `home:/Documents`).
    Predicate(&'a str),
    /// The root directory separator `/`.
    RootDir,
    /// The current directory `.`.
    CurDir,
    /// The parent directory `..`.
    ParentDir,
    /// A normal path component.
    Normal(&'a str),
}

// Helper function to parse predicate from path
fn parse_predicate(s: &str) -> Option<(&str, &str)>
{
    if let Some(colon_pos) = s.find(':')
    {
        let predicate = &s[..colon_pos];
        let rest = &s[colon_pos + 1..];

        if !predicate.contains('/') && !predicate.is_empty()
        {
            return Some((predicate, rest));
        }
    }

    None
}

// Path implementations
impl Path
{
    /// Directly wraps a string slice as a `Path` slice.
    #[status(stable)]
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> &Path
    {
        unsafe
        {
            &*(s.as_ref() as *const str as *const Path)
        }
    }

    /// Returns the path as a string slice.
    #[status(stable)]
    pub fn as_str(&self) -> &str
    {
        &self.inner
    }

    /// Returns `true` if the path has a predicate.
    #[status(stable)]
    pub fn has_predicate(&self) -> bool
    {
        parse_predicate(&self.inner)
        .   is_some()
    }

    /// Returns the predicate of the path, if any.
    #[status(stable)]
    pub fn predicate(&self) -> Option<&str>
    {
        parse_predicate(&self.inner)
        .   map(|(p, _)| p)
    }

    /// Returns `true` if the path is absolute (has a root `/` after predicate or at start).
    #[status(stable)]
    pub fn is_absolute(&self) -> bool
    {
        let rest = match parse_predicate(&self.inner)
        {
            Some((_, r)) => r,
            None => &self.inner,
        };

        rest.starts_with('/')
    }

    /// Returns `true` if the path is relative.
    #[status(stable)]
    pub fn is_relative(&self) -> bool
    {
        !self.is_absolute()
    }

    /// Returns the parent directory of this path.
    #[status(stable)]
    pub fn parent(&self) -> Option<&Path>
    {
        let path = &self.inner;
        
        if path.is_empty()
        {
            return None
        }
        
        let (predicate, rest) = match parse_predicate(path)
        {
            Some((p, r)) => (Some(p), r),
            None => (None, path),
        };
        
        if rest.is_empty()
        || rest == "/"
        {
            return None
        }
        
        if let Some(last_slash) = rest.rfind('/')
        {
            let end = predicate.map(|p| p.len() + 1 + last_slash).unwrap_or(last_slash);

            if end == 0
            {
                return Some(Path::new("/"));
            }

            return Some(Path::new(&path[..end]))
        }
        
        if predicate.is_some()
        {
            let pred_len = predicate.unwrap().len();
            return Some(Path::new(&path[..pred_len + 1]))
        }
        
        None
    }

    /// Returns the final component of the path, if any.
    #[status(stable)]
    pub fn file_name(&self) -> Option<&str>
    {
        let path = &self.inner;
        
        let (_, rest) = match parse_predicate(path)
        {
            Some((p, r)) => (Some(p), r),
            None => (None, path),
        };
        
        if rest.is_empty()
        || rest == "/"
        {
            return None
        }
        
        let rest = rest.trim_end_matches('/');
        
        if let Some(last_slash) = rest.rfind('/')
        {
            let name = &rest[last_slash + 1..];

            if name == "." || name == ".."
            {
                None
            }
            else
            {
                Some(name)
            }
        }
        else
        {
            if rest == "."
            || rest == ".."
            {
                None
            }
            else
            {
                Some(rest)
            }
        }
    }

    /// Returns the file extension of the path.
    #[status(stable)]
    pub fn extension(&self) -> Option<&str>
    {
        self
        .   file_name()
        .   and_then(|name|
        {
            name
            .   rfind('.')
            .   map(|dot| &name[dot + 1..])
        })
    }

    /// Returns an iterator over the components of the path.
    #[status(stable)]
    pub fn components(&self) -> impl Iterator<Item = Component<'_>>
    {
        let path = &self.inner;
        let mut components = Vec::new();
        
        let rest = if let Some((predicate, rest)) = parse_predicate(path)
        {
            components.push(Component::Predicate(predicate));
            rest
        }
        else
        {
            path
        };
        
        if rest.starts_with('/')
        {
            components.push(Component::RootDir);
        }
        
        let mut i = if rest.starts_with('/') { 1 } else { 0 };
        let bytes = rest.as_bytes();
        let len = bytes.len();
        
        while i < len
        {
            if bytes[i] == b'/'
            {
                i += 1;
            }
            else
            {
                let start = i;

                while i < len && bytes[i] != b'/'
                {
                    i += 1;
                }

                let comp = &rest[start..i];

                match comp
                {
                    "." => components.push(Component::CurDir),
                    ".." => components.push(Component::ParentDir),
                    _ => components.push(Component::Normal(comp)),
                }
            }
        }
        
        components.into_iter()
    }

    /// Creates an owned `PathBuf` with `path` adjoined to `self`.
    #[status(stable)]
    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf
    {
        let mut buf = PathBuf::from(self.as_str());
        buf.push(path);
        buf
    }

    /// Returns the canonical form of the path.
    ///
    /// If no predicate is present, uses the current root from `raw::ExecRoot()`.
    /// The canonical form always includes a predicate.
    #[status(stable)]
    pub fn canonicalize(&self) -> Result<PathBuf, core::io::Error>
    {
        let predicate = match self.predicate()
        {
            Some(p) => p.to_string(),
            None => crate::raw::ExecRoot(),
        };
        
        let rest = match parse_predicate(&self.inner)
        {
            Some((_, r)) => r,
            None => &self.inner,
        };
        
        let is_absolute = rest.starts_with('/');

        let full_path = if is_absolute
        {
            rest.to_string()
        }
        else
        {
            let current = crate::raw::ExecPwd()
            .   unwrap_or_default();

            let (_, current_rest) = parse_predicate(&current)
            .   unwrap_or(("", &current));

            if current_rest.is_empty()
            {
                rest.to_string()
            }
            else
            {
                format!("{}/{}", current_rest, rest)
            }
        };
        
        let mut result = Vec::new();

        for comp in full_path
        .   split('/')
        .   filter(|s| !s.is_empty())
        {
            match comp
            {
                "." => {}
                
                ".." =>
                {
                    result.pop();
                }
                
                _ =>
                {
                    result.push(comp.to_string());
                }
            }
        }
        
        let mut canonical = String::new();
        canonical.push_str(&predicate);
        canonical.push(':');

        if !result.is_empty()
        {
            canonical.push('/');
            canonical.push_str(&result.join("/"));
        }
        
        Ok(PathBuf::from(canonical))
    }
}

// PathBuf implementations
impl PathBuf
{
    /// Creates a new empty `PathBuf`.
    #[status(stable)]
    pub fn new() -> PathBuf
    {
        PathBuf
        {
            inner: String::new()
        }
    }

    /// Returns the path as a `Path` slice.
    #[status(stable)]
    pub fn as_path(&self) -> &Path
    {
        Path::new(&self.inner)
    }

    /// Extends `self` with `path`.
    ///
    /// If `path` is absolute or has a predicate, it replaces `self`.
    #[status(stable)]
    pub fn push<P: AsRef<Path>>(&mut self, path: P)
    {
        let path = path.as_ref();
        let path_str = path.as_str();
        
        if path.has_predicate()
        || path_str.starts_with('/')
        {
            self.inner = path_str.to_string();
        }
        else
        {
            if !self.inner.is_empty()
            && !self.inner.ends_with('/')
            && !self.inner.ends_with(':')
            {
                self.inner.push('/');
            }

            self.inner.push_str(path_str);
        }
    }

    /// Truncates `self` to the parent directory.
    #[status(stable)]
    pub fn pop(&mut self) -> bool
    {
        let path = self.as_path();

        if let Some(parent) = path.parent()
        {
            self.inner = parent.as_str().to_string();
            true
        }
        else
        {
            false
        }
    }

    /// Updates the file name of the path.
    #[status(stable)]
    pub fn set_file_name<S: AsRef<str>>(&mut self, file_name: S)
    {
        if self.pop()
        {
            self.push(file_name.as_ref());
        }
    }

    /// Updates the extension of the path.
    #[status(stable)]
    pub fn set_extension<S: AsRef<str>>(&mut self, extension: S) -> bool
    {
        if let Some(file_name) = self.file_name()
        {
            let ext = extension.as_ref();

            let new_name = if let Some(dot) = file_name.rfind('.')
            {
                format!("{}.{}", &file_name[..dot], ext)
            }
            else
            {
                format!("{}.{}", file_name, ext)
            };

            self.set_file_name(&new_name);
            true
        }
        else
        {
            false
        }
    }
}

// Trait implementations
impl AsRef<str> for Path
{
    #[status(stable)]
    fn as_ref(&self) -> &str
    {
        &self.inner
    }
}

impl AsRef<Path> for str
{
    #[status(stable)]
    fn as_ref(&self) -> &Path
    {
        Path::new(self)
    }
}

impl AsRef<Path> for String
{
    #[status(stable)]
    fn as_ref(&self) -> &Path
    {
        Path::new(self)
    }
}

impl AsRef<Path> for &Path
{
    #[status(stable)]
    fn as_ref(&self) -> &Path
    {
        *self
    }
}

impl AsRef<Path> for PathBuf
{
    #[status(stable)]
    fn as_ref(&self) -> &Path
    {
        self.as_path()
    }
}

impl Deref for PathBuf
{
    type Target = Path;
    
    #[status(stable)]
    fn deref(&self) -> &Path
    {
        self.as_path()
    }
}

impl From<&str> for PathBuf
{
    #[status(stable)]
    fn from(s: &str) -> PathBuf
    {
        PathBuf
        {
            inner: s.to_string()
        }
    }
}

impl From<String> for PathBuf
{
    #[status(stable)]
    fn from(s: String) -> PathBuf
    {
        PathBuf { inner: s }
    }
}

impl<'a> From<&'a Path> for PathBuf
{
    #[status(stable)]
    fn from(path: &'a Path) -> PathBuf
    {
        PathBuf::from(path.as_str())
    }
}

impl fmt::Debug for Path
{
    #[status(stable)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "\"{}\"", &self.inner)
    }
}

impl fmt::Display for Path
{
    #[status(stable)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl fmt::Debug for PathBuf
{
    #[status(stable)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "\"{}\"", &self.inner)
    }
}

impl fmt::Display for PathBuf
{
    #[status(stable)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl<'a> Component<'a>
{
    /// Returns the string representation of the component.
    #[status(stable)]
    pub fn as_str(&self) -> &'a str
    {
        match self
        {
            Component::Predicate(s) => s,
            Component::RootDir => "/",
            Component::CurDir => ".",
            Component::ParentDir => "..",
            Component::Normal(s) => s,
        }
    }
}
