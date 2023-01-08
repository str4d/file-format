//! Macros.

/// Generates `FileFormat` enum with methods.
macro_rules! formats {
    {
        $(
            format = $format:ident
            name = $name:literal
            $(short_name = $short_name:literal)?
            media_type = $media_type:literal
            extension = $extension:literal
            kind = $kind:ident
        )*
    } => {
        /// A file format.
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum FileFormat {
            $(
                #[doc=concat!($name, $(" (", $short_name, ")",)? ".")]
                #[doc=concat!("- Media type: `", $media_type, "`")]
                #[doc=concat!("- Extension: `", $extension, "`")]
                #[doc=concat!("- Kind: `", stringify!($kind), "`")]
                $format,
            )*
        }

        impl crate::FileFormat {
            /// Returns the name of the file format.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::Mpeg12AudioLayer3;
            /// assert_eq!(format.name(), "MPEG-1/2 Audio Layer 3");
            ///```
            pub const fn name(&self) -> &str {
                match self {
                    $(
                        Self::$format => $name,
                    )*
                }
            }

            /// Returns the short name of the file format.
            ///
            /// If there is none, the [name](crate::FileFormat::name) is returned.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::MusicalInstrumentDigitalInterface;
            /// assert_eq!(format.short_name(), "MIDI");
            ///```
            pub const fn short_name(&self) -> &str {
                match self {
                    $(
                        $(Self::$format => $short_name,)?
                    )*
                    _ => self.name(),
                }
            }

            /// Returns the [name](crate::FileFormat::name) of the file format followed by its
            /// [short name](crate::FileFormat::short_name) in parentheses.
            ///
            /// If there is no short name, the [name](crate::FileFormat::name) alone is returned.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::OfficeOpenXmlDocument;
            /// assert_eq!(format.display_name(), "Office Open XML Document (DOCX)");
            ///```
            pub const fn display_name(&self) -> &str {
                match self {
                    $(
                        $(Self::$format => concat!($name, " (", $short_name, ")"),)?
                    )*
                    _ => self.name(),
                }
            }

            /// Returns the media type (formerly known as MIME type) of the file format.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::Zstandard;
            /// assert_eq!(format.media_type(), "application/zstd");
            ///```
            pub const fn media_type(&self) -> &str {
                match self {
                    $(
                        Self::$format => $media_type,
                    )*
                }
            }

            /// Returns the common extension of the file format.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use file_format::FileFormat;
            ///
            /// let format = FileFormat::WindowsMediaVideo;
            /// assert_eq!(format.extension(), "wmv");
            ///```
            pub const fn extension(&self) -> &str {
                match self {
                    $(
                        Self::$format => $extension,
                    )*
                }
            }

            /// Returns the [Kind](crate::Kind) of the file format.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use file_format::{FileFormat, Kind};
            ///
            /// let format = FileFormat::FreeLosslessAudioCodec;
            /// assert_eq!(format.kind(), Kind::Audio);
            ///```
            pub const fn kind(&self) -> crate::Kind {
                match self {
                    $(
                        Self::$format => crate::Kind::$kind,
                    )*
                }
            }
        }
    };
}

/// Generates `FileFormat::from_signature` function.
macro_rules! signatures {
    {
        $(
            format = $format:ident
            $(value = $($value:literal $(offset = $offset:literal)?),+)+
        )*
    } => {
        impl crate::FileFormat {
            /// Determines file format by checking its signature.
            pub(crate) fn from_signature(bytes: &[u8]) -> Option<Self> {
                $(
                    if $($(bytes.len() >= $($offset +)? $value.len()
                        && &bytes[$($offset)?..$($offset +)? $value.len()] == $value)&&*)||* {
                        return Some(Self::$format);
                    }
                )*
                None
            }
        }
    };
}

/// Returns a limit value based on the `accuracy` feature flag.
macro_rules! limit {
    ($min:literal, $max:literal) => {
        if cfg!(feature = "accuracy") {
            $max
        } else {
            $min
        }
    };
}
