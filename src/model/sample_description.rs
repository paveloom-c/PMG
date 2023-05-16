//! Sample description

use super::Model;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Result};
use indoc::formatdoc;

impl<F> Model<F> {
    /// Read the sample description from the input file
    pub fn try_read_sample_description_from(&mut self, path: &Path) -> Result<()> {
        let file =
            File::open(path).with_context(|| format!("Couldn't read from the file {path:?}"))?;
        let mut lines = BufReader::new(file).lines();
        self.sample_description = 'out: {
            let mut sample_description = Vec::<String>::new();
            'skip: loop {
                match lines.next() {
                    Some(Ok(line)) => {
                        if line.eq("# Sample:") {
                            sample_description.push(String::from("#"));
                            sample_description.push(line);
                            break 'skip;
                        }
                    }
                    _ => break 'out None,
                }
            }
            'take: loop {
                match lines.next() {
                    Some(Ok(line)) => {
                        if line.eq("# Descriptions:") {
                            break 'take;
                        }
                        sample_description.push(line);
                    }
                    _ => break 'out None,
                }
            }
            break 'out Some(sample_description.join("\n"));
        };
        Ok(())
    }
    /// Get the sample description as a formatted string
    pub fn format_sample_description(&self) -> String {
        let n = self.objects.borrow().len();
        let l_stroke_3_n = self
            .l_stroke_3_n
            .as_ref()
            .map_or("???".to_owned(), |v| format!("{v}"));
        let l_stroke_1_n = self
            .l_stroke_1_n
            .as_ref()
            .map_or("???".to_owned(), |v| format!("{v}"));
        match self.sample_description {
            Some(ref sample_description) => formatdoc!(
                "
                {sample_description}
                # Total number of the objects: {n}.
                # Number of the objects after the L' = 3 run: {l_stroke_3_n}.
                # Number of the objects after the L' = 1 run: {l_stroke_1_n}.
                #",
            ),
            None => String::from("#"),
        }
    }
}
