use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Fasta {
    pub(crate) title: String,
    pub(crate) text: String,
}

impl Fasta {
    pub(crate) fn chars(&self) -> std::str::Chars<'_> {
        self.text.chars()
    }

    pub(crate) fn new(title: impl Into<String>, text: impl Into<String>) -> Self {
        Fasta {
            title: title.into(),
            text: text.into(),
        }
    }

    pub(crate) fn read(text: &str) -> Result<Self, Box<dyn Error>> {
        let mut lines = text.lines();
        let title = lines
            .next()
            .ok_or("No title line found")?
            .trim_start_matches('>')
            .to_string();
        let code = lines.collect::<String>();
        Ok(Self::new(title, code))
    }

    pub(crate) fn read_file_component(file: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let content = read_to_string(file)?;
        Self::read(content.trim()).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid FASTA format")
        })
    }

    pub(crate) fn read_file(file: impl AsRef<Path>) -> Result<Vec<Self>, std::io::Error> {
        let content = read_to_string(file)?;
        let fasta_segments = content.split('>');
        fasta_segments
            .into_iter()
            .filter(|&f| !f.is_empty())
            .map(|fasta_content| {
                Self::read(fasta_content.trim()).map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, fasta_content.trim())
                })
            })
            .collect::<Result<Vec<_>, std::io::Error>>()
    }

    pub(crate) fn len(&self) -> usize {
        self.text.len()
    }
}

pub(crate) type Dna = Fasta;
pub(crate) type Rna = Fasta;

impl Dna {
    pub(crate) fn dna(&self) -> &str {
        &self.text
    }
}

impl Rna {
    pub(crate) fn rna(&self) -> &str {
        &self.text
    }
}
