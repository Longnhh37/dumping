 use std::fs::{File, OpenOptions};
 use std::io::{self, Read, Write};

 use crate::expander::redirect::ExpandedRedirect;

 pub struct WriterSet {
     pub stdin: Box<dyn Read>,
     pub stdout: Box<dyn Write>,
     pub stderr: Box<dyn Write>,
 }

 pub fn make_writer_set(redirect: &ExpandedRedirect) -> io::Result<WriterSet> {
     let stdin: Box<dyn  Read> = match redirect {
         ExpandedRedirect::Input(path) => Box::new(File::open(path)?),
         _ => Box::new(io::stdin()),
     };

     let stdout: Box<dyn Write> = match redirect {
         ExpandedRedirect::Output(path) => Box::new(File::create(path)?),
         ExpandedRedirect::Append(path) => Box::new(
             OpenOptions::new().append(true).create(true).open(path)?
         ),
         _ => Box::new(io::stdout()),
     };

     let stderr: Box<dyn Write> = match redirect {
         ExpandedRedirect::ErrorOutput(path) => Box::new(File::create(path)?),
         ExpandedRedirect::ErrorAppend(path) => Box::new(
             OpenOptions::new().append(true).create(true).open(path)?
         ),
         _ => Box::new(io::stderr()),
     };

     Ok(WriterSet {
         stdin,
         stdout,
         stderr,
     })
 }
