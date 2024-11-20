use crate::structures::common::{self, StructureError};

#[derive(Debug, Default, Clone)]
pub struct TTFTableEntry {
    pub tag: [u8; 4],
    pub checksum: u32,
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug, Default, Clone)]
pub struct TTFHeader {
    pub tables: Vec<TTFTableEntry>,
}

pub const TTF_HEADER_SIZE: usize = 4 + 2 + 2 + 2 + 2;
pub const REQUIRED_TABLES: &[[u8; 4]] = &[*b"head", *b"hhea", *b"maxp"];

// https://github.com/WerWolv/ImHex-Patterns/blob/master/patterns/ttf.hexpat
pub fn parse_ttf_header(data: &[u8]) -> Result<TTFHeader, StructureError> {
    let ttf_header_structure = vec![
        ("magic", "u32"),
        ("num_tables", "u16"),
        ("search_range", "u16"),
        ("entry_selector", "u16"),
        ("range_shift", "u16"),
    ];

    let ttf_table_structure = vec![
        ("tag", "u32"),
        ("checksum", "u32"),
        ("offset", "u32"),
        ("length", "u32"),
    ];

    // Parse the header
    if let Ok(header) = common::parse(data, &ttf_header_structure, "big") {
        if header["num_tables"] > 32 {
            return Err(StructureError);
        }

        let mut tables = vec![];
        let table_entry_size = common::size(&ttf_table_structure);
        for i in 0..header["num_tables"] {
            let entry_offset = TTF_HEADER_SIZE + i * table_entry_size;
            let table_entry = common::parse(
                data.get(entry_offset..entry_offset + table_entry_size)
                    .ok_or(StructureError)?,
                &ttf_table_structure,
                "big",
            )?;

            tables.push(TTFTableEntry {
                tag: (table_entry["tag"] as u32).to_be_bytes(),
                checksum: table_entry["checksum"] as u32,
                offset: table_entry["offset"],
                length: table_entry["length"],
            });
        }

        for required_tag in REQUIRED_TABLES {
            if !tables.iter().any(|t| &t.tag == required_tag) {
                return Err(StructureError);
            }
        }

        return Ok(TTFHeader { tables });
    }

    Err(StructureError)
}

pub fn calculate_ttf_file_size(header: &TTFHeader) -> Option<usize> {
    header
        .tables
        .iter()
        .map(|t| t.offset as usize + t.length as usize)
        .max()
}
