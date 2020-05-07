/*!

This crate allows you to easily write text.

Usage:

```no_run
# extern crate glium;
# extern crate glium_text_rusttype as glium_text;
# extern crate cgmath;
# use std::fs::File;
# fn main() {
# let display: glium::Display = unsafe { std::mem::uninitialized() };
// The `TextSystem` contains the shaders and elements used for text display.
let system = glium_text::TextSystem::new(&display);

// Creating a `FontAtlas`, which a regular `Texture` which contains the font.
// Note that loading the systems fonts is not covered by this library.
let font = glium_text::FontAtlas::new(
    &display,
    File::open("font.ttf").unwrap(),
    32,
    glium_text::FontAtlas::ascii_character_list()
).unwrap();

// Creating a `TextDisplay` which contains the elements required to draw a specific sentence.
let text = glium_text::TextDisplay::new(&system, &font, "Hello world!");

// Finally, drawing the text is done like this:
let matrix = [[1.0, 0.0, 0.0, 0.0],
              [0.0, 1.0, 0.0, 0.0],
              [0.0, 0.0, 1.0, 0.0],
              [0.0, 0.0, 0.0, 1.0]];
glium_text::draw(&text, &system, &mut display.draw(), matrix, (1.0, 1.0, 0.0, 1.0));
# }
```

`FontAtlas` will rasterize exact characters it's told to.  It can be passed
anything that implements `IntoIterator<char>` trait.  It is possible to use
chained ranges with it like this:
`(0 .. 0x7f+1).chain(0x400 .. 0x4ff+1).filter_map(std::char::from_u32)`

This will rasterize font with characters from "Basic Latin" and "Cyrillic"
sets.  Here is a complete list of ranges with description.

*As of now, Rust doesn't provide a way to create inclusive ranges yet so thou
shall add +1 to the outer bound.*

| Start | End   | Description                             |
|-------+-------+-----------------------------------------|
|  0000 | 007F  | Basic Latin                             |
|  0080 | 00FF  | C1 Controls and Latin-1 Supplement      |
|  0100 | 017F  | Latin Extended-A                        |
|  0180 | 024F  | Latin Extended-B                        |
|  0250 | 02AF  | IPA Extensions                          |
|  02B0 | 02FF  | Spacing Modifier Letters                |
|  0300 | 036F  | Combining Diacritical Marks             |
|  0370 | 03FF  | Greek/Coptic                            |
|  0400 | 04FF  | Cyrillic                                |
|  0500 | 052F  | Cyrillic Supplement                     |
|  0530 | 058F  | Armenian                                |
|  0590 | 05FF  | Hebrew                                  |
|  0600 | 06FF  | Arabic                                  |
|  0700 | 074F  | Syriac                                  |
|  0780 | 07BF  | Thaana                                  |
|  0900 | 097F  | Devanagari                              |
|  0980 | 09FF  | Bengali/Assamese                        |
|  0A00 | 0A7F  | Gurmukhi                                |
|  0A80 | 0AFF  | Gujarati                                |
|  0B00 | 0B7F  | Oriya                                   |
|  0B80 | 0BFF  | Tamil                                   |
|  0C00 | 0C7F  | Telugu                                  |
|  0C80 | 0CFF  | Kannada                                 |
|  0D00 | 0DFF  | Malayalam                               |
|  0D80 | 0DFF  | Sinhala                                 |
|  0E00 | 0E7F  | Thai                                    |
|  0E80 | 0EFF  | Lao                                     |
|  0F00 | 0FFF  | Tibetan                                 |
|  1000 | 109F  | Myanmar                                 |
|  10A0 | 10FF  | Georgian                                |
|  1100 | 11FF  | Hangul Jamo                             |
|  1200 | 137F  | Ethiopic                                |
|  13A0 | 13FF  | Cherokee                                |
|  1400 | 167F  | Unified Canadian Aboriginal Syllabics   |
|  1680 | 169F  | Ogham                                   |
|  16A0 | 16FF  | Runic                                   |
|  1700 | 171F  | Tagalog                                 |
|  1720 | 173F  | Hanunoo                                 |
|  1740 | 175F  | Buhid                                   |
|  1760 | 177F  | Tagbanwa                                |
|  1780 | 17FF  | Khmer                                   |
|  1800 | 18AF  | Mongolian                               |
|  1900 | 194F  | Limbu                                   |
|  1950 | 197F  | Tai Le                                  |
|  19E0 | 19FF  | Khmer Symbols                           |
|  1D00 | 1D7F  | Phonetic Extensions                     |
|  1E00 | 1EFF  | Latin Extended Additional               |
|  1F00 | 1FFF  | Greek Extended                          |
|  2000 | 206F  | General Punctuation                     |
|  2070 | 209F  | Superscripts and Subscripts             |
|  20A0 | 20CF  | Currency Symbols                        |
|  20D0 | 20FF  | Combining Diacritical Marks for Symbols |
|  2100 | 214F  | Letterlike Symbols                      |
|  2150 | 218F  | Number Forms                            |
|  2190 | 21FF  | Arrows                                  |
|  2200 | 22FF  | Mathematical Operators                  |
|  2300 | 23FF  | Miscellaneous Technical                 |
|  2400 | 243F  | Control Pictures                        |
|  2440 | 245F  | Optical Character Recognition           |
|  2460 | 24FF  | Enclosed Alphanumerics                  |
|  2500 | 257F  | Box Drawing                             |
|  2580 | 259F  | Block Elements                          |
|  25A0 | 25FF  | Geometric Shapes                        |
|  2600 | 26FF  | Miscellaneous Symbols                   |
|  2700 | 27BF  | Dingbats                                |
|  27C0 | 27EF  | Miscellaneous Mathematical Symbols-A    |
|  27F0 | 27FF  | Supplemental Arrows-A                   |
|  2800 | 28FF  | Braille Patterns                        |
|  2900 | 297F  | Supplemental Arrows-B                   |
|  2980 | 29FF  | Miscellaneous Mathematical Symbols-B    |
|  2A00 | 2AFF  | Supplemental Mathematical Operators     |
|  2B00 | 2BFF  | Miscellaneous Symbols and Arrows        |
|  2E80 | 2EFF  | CJK Radicals Supplement                 |
|  2F00 | 2FDF  | Kangxi Radicals                         |
|  2FF0 | 2FFF  | Ideographic Description Characters      |
|  3000 | 303F  | CJK Symbols and Punctuation             |
|  3040 | 309F  | Hiragana                                |
|  30A0 | 30FF  | Katakana                                |
|  3100 | 312F  | Bopomofo                                |
|  3130 | 318F  | Hangul Compatibility Jamo               |
|  3190 | 319F  | Kanbun (Kunten)                         |
|  31A0 | 31BF  | Bopomofo Extended                       |
|  31F0 | 31FF  | Katakana Phonetic Extensions            |
|  3200 | 32FF  | Enclosed CJK Letters and Months         |
|  3300 | 33FF  | CJK Compatibility                       |
|  3400 | 4DBF  | CJK Unified Ideographs Extension A      |
|  4DC0 | 4DFF  | Yijing Hexagram Symbols                 |
|  4E00 | 9FAF  | CJK Unified Ideographs                  |
|  A000 | A48F  | Yi Syllables                            |
|  A490 | A4CF  | Yi Radicals                             |
|  AC00 | D7AF  | Hangul Syllables                        |
|  D800 | DBFF  | High Surrogate Area                     |
|  DC00 | DFFF  | Low Surrogate Area                      |
|  E000 | F8FF  | Private Use Area                        |
|  F900 | FAFF  | CJK Compatibility Ideographs            |
|  FB00 | FB4F  | Alphabetic Presentation Forms           |
|  FB50 | FDFF  | Arabic Presentation Forms-A             |
|  FE00 | FE0F  | Variation Selectors                     |
|  FE20 | FE2F  | Combining Half Marks                    |
|  FE30 | FE4F  | CJK Compatibility Forms                 |
|  FE50 | FE6F  | Small Form Variants                     |
|  FE70 | FEFF  | Arabic Presentation Forms-B             |
|  FF00 | FFEF  | Halfwidth and Fullwidth Forms           |
|  FFF0 | FFFF  | Specials                                |
| 10000 | 1007F | Linear B Syllabary                      |
| 10080 | 100FF | Linear B Ideograms                      |
| 10100 | 1013F | Aegean Numbers                          |
| 10300 | 1032F | Old Italic                              |
| 10330 | 1034F | Gothic                                  |
| 10380 | 1039F | Ugaritic                                |
| 10400 | 1044F | Deseret                                 |
| 10450 | 1047F | Shavian                                 |
| 10480 | 104AF | Osmanya                                 |
| 10800 | 1083F | Cypriot Syllabary                       |
| 1D000 | 1D0FF | Byzantine Musical Symbols               |
| 1D100 | 1D1FF | Musical Symbols                         |
| 1D300 | 1D35F | Tai Xuan Jing Symbols                   |
| 1D400 | 1D7FF | Mathematical Alphanumeric Symbols       |
| 20000 | 2A6DF | CJK Unified Ideographs Extension B      |
| 2F800 | 2FA1F | CJK Compatibility Ideographs Supplement |
| E0000 | E007F | Tags                                    |
| E0100 | E01EF | Variation Selectors Supplement          |

*/

#![allow(warnings)]
#![warn(missing_docs)]

use std::borrow::Cow;
use std::collections::HashMap;
use std::default::Default;
use std::io::Read;
use std::rc::Rc;

use rusttype::{Point, Rect};

#[cfg(feature = "render")]
mod render;

#[cfg(feature = "render")]
pub use render::*;

pub type AtlasCharacterInfos = HashMap<char, CharacterInfos>;

/// RGBA8 font texture
pub struct Texture {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// Texture which contains the characters of the font.
pub struct FontAtlas {
    pub texture: Texture,
    pub character_infos: AtlasCharacterInfos,
    pub font_size: u32,
}

///
#[derive(Debug)]
pub enum Error {
    /// A glyph for this character is not present in font.
    NoGlyph(char),
}

// structure containing informations about a character of a font
#[derive(Copy, Clone, Debug)]
pub struct CharacterInfos {
    // coordinates of the character top-left hand corner on the font's texture
    pub tex_coords: (f32, f32),

    // width and height of character in texture units
    pub tex_size: (f32, f32),

    // size of the character in EMs
    pub size: (f32, f32),

    // number of EMs between the bottom of the character and the base line of text
    pub height_over_line: f32,

    // number of EMs at the left of the character
    pub left_padding: f32,

    // number of EMs at the right of the character
    pub right_padding: f32,
}

impl CharacterInfos {
    pub fn scale(&self, font_size: f32) -> CharacterInfos {
        CharacterInfos {
            tex_coords: self.tex_coords,
            tex_size: self.tex_size,
            size: (self.size.0 * font_size, self.size.1 * font_size),
            height_over_line: self.height_over_line * font_size,
            left_padding: self.left_padding * font_size,
            right_padding: self.right_padding * font_size,
        }
    }
}

impl FontAtlas {
    /// Vec<char> of complete ASCII range (from 0 to 255 bytes)
    pub fn ascii_character_list() -> Vec<char> {
        (0..255).filter_map(::std::char::from_u32).collect()
    }

    // TODO: glean chinese and japanese character lists from imgui

    pub fn cyrllic_character_list() -> Vec<char> {
        let ranges = [
            0x0020u32..0x00FF, // Basic Latin + Latin Supplement
            0x0400u32..0x052F, // Cyrillic + Cyrillic Supplement
            0x2DE0u32..0x2DFF, // Cyrillic Extended-A
            0xA640u32..0xA69F, // Cyrillic Extended-B
        ];

        flatten_ranges(ranges.iter())
    }

    pub fn thai_character_list() -> Vec<char> {
        let ranges = [
            0x0020u32..0x00FF, // Basic Latin
            0x2010u32..0x205E, // Punctuations
            0x0E00u32..0x0E7F, // Thai
        ];

        flatten_ranges(ranges.iter())
    }

    pub fn vietnamese_character_list() -> Vec<char> {
        let ranges = [
            0x0020u32..0x00FF, // Basic Latin
            0x0102u32..0x0103,
            0x0110u32..0x0111,
            0x0128u32..0x0129,
            0x0168u32..0x0169,
            0x01A0u32..0x01A1,
            0x01AFu32..0x01B0,
            0x1EA0u32..0x1EF9,
        ];

        flatten_ranges(ranges.iter())
    }

    /// Creates a new texture representing a font stored in a `FontAtlas`.
    /// This function is very expensive as it needs to rasterize font into a
    /// texture.  Complexity grows as `font_size**2 * characters_list.len()`.
    /// **Avoid rasterizing everything at once as it will be slow and end up in
    /// out of memory abort.**
    pub fn new<R, I>(font: R, font_size: u32, characters_list: I) -> Result<FontAtlas, Error>
    where
        R: Read,
        I: IntoIterator<Item = char>,
    {
        // building the freetype face object
        let font: Vec<u8> = font.bytes().map(|c| c.unwrap()).collect();

        let font = ::rusttype::Font::try_from_bytes(&font[..]).unwrap();

        // building the infos
        let (texture, chr_infos) = build_font_image(&font, characters_list.into_iter(), font_size)?;

        Ok(FontAtlas {
            texture,
            character_infos: chr_infos,
            font_size,
        })
    }
}

fn flatten_ranges<'a>(ranges: impl Iterator<Item = &'a std::ops::Range<u32>>) -> Vec<char> {
    ranges
        .cloned()
        .flatten()
        .map(|c| std::char::from_u32(c).unwrap())
        .collect()
}

fn build_font_image<I>(
    font: &rusttype::Font,
    characters_list: I,
    font_size: u32,
) -> Result<(Texture, HashMap<char, CharacterInfos>), Error>
where
    I: Iterator<Item = char>,
{
    use std::iter;

    // a margin around each character to prevent artifacts
    const MARGIN: u32 = 2;

    // glyph size for characters not presented in font.
    let invalid_character_width = font_size / 4;

    let size_estimation = characters_list.size_hint().1.unwrap_or(0);

    // this variable will store the texture data
    // we set an arbitrary capacity that we think will match what we will need
    let mut texture_data: Vec<u8> =
        Vec::with_capacity(size_estimation * font_size as usize * font_size as usize);

    // the width is chosen more or less arbitrarily, because we can store
    // everything as long as the texture is at least as wide as the widest
    // character we just try to estimate a width so that width ~= height
    let texture_width = get_nearest_po2(std::cmp::max(
        font_size * 2 as u32,
        ((((size_estimation as u32) * font_size * font_size) as f32).sqrt()) as u32,
    ));

    // we store the position of the "cursor" in the destination texture
    // this cursor points to the top-left pixel of the next character to write on the texture
    let mut cursor_offset = (0u32, 0u32);

    // number of rows to skip at next carriage return
    let mut rows_to_skip = 0u32;

    // now looping through the list of characters, filling the texture and returning the informations
    let em_pixels = font_size as f32;
    let characters_infos = characters_list
        .map(|character| {
            struct Bitmap {
                rows: i32,
                width: i32,
                buffer: Vec<u8>,
            }
            // loading wanted glyph in the font face
            // hope scale will set the right pixel size
            let scaled_glyph = font.glyph(character).scaled(::rusttype::Scale {
                x: font_size as f32,
                y: font_size as f32,
            });
            let h_metrics = scaled_glyph.h_metrics();
            let glyph = scaled_glyph.positioned(::rusttype::Point { x: 0.0, y: 0.0 });

            let bb = glyph.pixel_bounding_box();
            // if no bounding box - we suppose that its invalid character but want it to be draw as empty quad
            let bb = if let Some(bb) = bb {
                bb
            } else {
                Rect {
                    min: Point { x: 0, y: 0 },
                    max: Point {
                        x: invalid_character_width as i32,
                        y: 0,
                    },
                }
            };

            let mut buffer = vec![0; (bb.height() * bb.width()) as usize];

            glyph.draw(|x, y, v| {
                let x = x;
                let y = y;
                buffer[(y * bb.width() as u32 + x) as usize] = (v * 255.0) as u8;
            });
            let bitmap: Bitmap = Bitmap {
                rows: bb.height(),
                width: bb.width(),
                buffer,
            };

            // adding a left margin before our character to prevent artifacts
            cursor_offset.0 += MARGIN;

            // carriage return our cursor if we don't have enough room to write the next caracter
            // we add a margin to prevent artifacts
            if cursor_offset.0 + (bitmap.width as u32) + MARGIN >= texture_width {
                assert!(bitmap.width as u32 <= texture_width); // if this fails, we should increase texture_width
                cursor_offset.0 = 0;
                cursor_offset.1 += rows_to_skip;
                rows_to_skip = 0;
            }

            // if the texture data buffer has not enough lines, adding some
            if rows_to_skip < MARGIN + bitmap.rows as u32 {
                let diff = MARGIN + (bitmap.rows as u32) - rows_to_skip;
                rows_to_skip = MARGIN + bitmap.rows as u32;
                texture_data.extend(iter::repeat(0).take((diff * texture_width * 4) as usize));
            }

            // copying the data to the texture
            let offset_x_before_copy = cursor_offset.0;
            if bitmap.rows >= 1 {
                let destination = &mut texture_data
                    [(cursor_offset.0 * 4 + cursor_offset.1 * texture_width * 4) as usize..];
                let source = &bitmap.buffer;
                //ylet source = std::slice::from_raw_parts(source, destination.len());

                for y in 0..bitmap.rows as u32 {
                    let source = &source[(y * bitmap.width as u32) as usize..];
                    let destination = &mut destination[(y * texture_width * 4) as usize..];

                    for x in 0..bitmap.width {
                        for channel in 0..4 {
                            let val: u8 = source[x as usize];
                            let dest = &mut destination[x as usize * 4 + channel];

                            *dest = val;
                        }
                    }
                }

                cursor_offset.0 += bitmap.width as u32;
                debug_assert!(cursor_offset.0 <= texture_width);
            }

            // filling infos about that character
            // tex_size and tex_coords are in pixels for the moment ; they will be divided
            // by the texture dimensions later
            Ok((
                character,
                CharacterInfos {
                    tex_size: (bitmap.width as f32, bitmap.rows as f32),
                    tex_coords: (offset_x_before_copy as f32, cursor_offset.1 as f32),
                    size: (bitmap.width as f32, bitmap.rows as f32),
                    left_padding: h_metrics.left_side_bearing as f32,
                    right_padding: (h_metrics.advance_width
                        - bitmap.width as f32
                        - h_metrics.left_side_bearing as f32)
                        as f32,
                    height_over_line: -bb.min.y as f32,
                },
            ))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    // adding blank lines at the end until the height of the texture is a power of two
    {
        let current_height = texture_data.len() as u32 / texture_width;
        let requested_height = get_nearest_po2(current_height);
        texture_data.extend(
            iter::repeat(0)
                .take((texture_width * 4 * (requested_height - current_height)) as usize),
        );
    }

    // now our texture is finished
    // we know its final dimensions, so we can divide all the pixels values into (0,1) range
    assert!((texture_data.len() as u32 % (texture_width * 4)) == 0);
    let texture_height = (texture_data.len() as u32 / texture_width / 4) as f32;
    let float_texture_width = texture_width as f32;
    let mut characters_infos = characters_infos
        .into_iter()
        .map(|mut chr| {
            chr.1.tex_size.0 /= float_texture_width;
            chr.1.tex_size.1 /= texture_height;
            chr.1.tex_coords.0 /= float_texture_width;
            chr.1.tex_coords.1 /= texture_height;
            chr.1.size.0 /= em_pixels;
            chr.1.size.1 /= em_pixels;
            chr.1.left_padding /= em_pixels;
            chr.1.right_padding /= em_pixels;
            chr.1.height_over_line /= em_pixels;
            chr
        })
        .collect::<HashMap<_, _>>();

    // this HashMap will not be used mutably any more and it makes sense to
    // compact it
    characters_infos.shrink_to_fit();

    // returning
    Ok((
        Texture {
            data: texture_data,
            width: texture_width,
            height: texture_height as u32,
        },
        characters_infos,
    ))
}

/// Function that will calculate the nearest power of two.
fn get_nearest_po2(mut x: u32) -> u32 {
    assert!(x > 0);
    x -= 1;
    x = x | (x >> 1);
    x = x | (x >> 2);
    x = x | (x >> 4);
    x = x | (x >> 8);
    x = x | (x >> 16);
    x + 1
}
