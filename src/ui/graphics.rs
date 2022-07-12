use std::path::{Path, PathBuf};
use std::fs::{read_dir, read_to_string};
use serde::{Deserialize};
use serde_json;

// TODO: In the source, this is a linked list
/// Describes a graphics mode which can be loaded as the active tile set
#[derive(Debug, Clone)]
pub struct GraphicsMode {
    /// Id of this mode (needs to be unique).
    ///  
    /// TODO: Isn't this just the index in the containing vector?
    pub graf_id: usize,
    /// Whether or not the tileset needs alpha blending.
    pub alphablend: bool,
    /// Row in the file where tiles in that row or lower draw the tile above as well.
    /// Together with `overdraw_max`, defines the set of double-height tiles.
    pub overdraw_row: usize,
    /// Row in the file where tiles in that row or above draw the tile above as well.
    /// Together with `overdraw_row`, defines the set of double-height tiles.
    pub overdraw_max: usize,
    /// Width of an individual tile in pixels.
    pub cell_width: usize,
    /// Height of an individual tile in pixels.
    pub cell_height: usize,
    /// Path to the PNG file to load.
    pub path: PathBuf,
    /// Player-visible name of the tileset.
    pub menuname: String
}

#[derive(Clone, Debug)]
pub enum FontType {
    Vector,
    Raster
}

#[derive(Clone, Debug)]
pub struct FontInfo {
    pub name: String,
    pub path: PathBuf,
    pub font_type: FontType
}

#[derive(Deserialize, Clone, Debug)]
pub struct SizeJson {
    pub width: usize,
    pub height: usize,
    pub file: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExtraJson {
    pub alpha: bool,
    pub start: usize,
    pub end: usize
}

#[derive(Deserialize, Clone, Debug)]
pub struct GraphicsModeJson {
    pub name: String,
    pub size: SizeJson,
    pub extra: ExtraJson
}

impl GraphicsMode {
    pub fn new(path: &Path, index: usize, json: GraphicsModeJson) -> GraphicsMode {
        GraphicsMode {
            graf_id: index,
            alphablend: json.extra.alpha,
            overdraw_row: json.extra.start,
            overdraw_max: json.extra.end,
            cell_width: json.size.width,
            cell_height: json.size.height,
            path: path.to_path_buf(),
            menuname: json.name.to_string()
        }
    }
}

/// Any Font (ttf) can be a Tileset but Tilesets cannot become Fonts
pub struct GraphicsModeService {
    pub graphics_modes: Vec<GraphicsMode>,
    pub current_graphics_mode: Option<usize>,
    pub current_font: usize,
    pub fonts: Vec<FontInfo>
}

impl GraphicsModeService {
    /// Read each subfolder under the given folder to see if there are Tile definitions there.
    /// If so, load them into the service
    pub fn from_folders(tiles_folder: &Path, fonts_folder: &Path) -> GraphicsModeService {
        let mut tiles_vec = vec!();
        for (idx, dir_read) in read_dir(tiles_folder).unwrap().enumerate() {
            let entry = dir_read.unwrap();
            let mut p = entry.path();
            if p.is_dir() {
                p.push("definition.json");
                let s = read_to_string(p).unwrap();
                let mode: GraphicsModeJson = serde_json::from_str(&s).unwrap();
                let gm = GraphicsMode::new(&entry.path(), idx, mode);
                tiles_vec.push(gm);
            }
        }

        let mut fonts_vec = vec!();
        for dir_read in read_dir(fonts_folder).unwrap() {
            let entry = dir_read.unwrap();
            let p = entry.path();
            if p.is_file() {
                match p.extension() {
                    None => { },
                    Some (os) => match os.to_str().unwrap() {
                        ".fon" =>{ 
                            fonts_vec.push(FontInfo {
                                font_type: FontType::Raster,
                                name: p.file_stem().unwrap().to_str().unwrap().to_string(),
                                path: p
                            })
                        },
                        _ => {
                            fonts_vec.push(FontInfo {
                                font_type: FontType::Vector,
                                name: p.file_stem().unwrap().to_str().unwrap().to_string(),
                                path: p
                            })
                        }
                    },

                }
            }
        }

        let found_font = fonts_vec.clone().into_iter().position(|f| f.name == "8x13x");

        GraphicsModeService {
            current_graphics_mode: None,
            graphics_modes: tiles_vec,
            current_font: found_font.unwrap_or(0),
            fonts: fonts_vec
        }
    }
}