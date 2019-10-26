pub mod tokenized;
pub mod cmp;
#[macro_use]
pub mod contract;
pub mod ffi;

use std::fs::File;


/// Type of the formatted input reader used in `WaveTestLib`.
pub type JudgeReader = crate::contract::JudgeReader<crate::tokenized::TokenizedReader<File>>;

/// A [`Checker`] instance is the core object in a checker implemented using `WaveTestLib.`
pub struct Checker {
    input: JudgeReader,
    std_answer: JudgeReader,
    user_answer: JudgeReader
}

impl Checker {
    /// Create a new [`Checker`] instance. Information required to build the instance
    /// is collected in the command line arguments of the program.
    pub fn new() -> Checker {
        // TODO: Implement Checker::new
        unimplemented!()
    }

    /// Get the [`JudgeReader`] instance around the input file.
    pub fn input(&mut self) -> &mut JudgeReader {
        &mut self.input
    }

    /// Get the [`JudgeReader`] instance around the standard answer file.
    pub fn std_answer(&mut self) -> &mut JudgeReader {
        &mut self.std_answer
    }

    /// Get the [`JudgeReader`] instance around the user's output file.
    pub fn user_answer(&mut self) -> &mut JudgeReader {
        &mut self.user_answer
    }
}

/// An [`Interactor`] instance is the core object in an Interactor implemented using
/// `WaveTestLib`.
pub struct Interactor {
    input: JudgeReader,
    answer: JudgeReader,
    read_end: JudgeReader,
    write_end: File
}

impl Interactor {
    /// Create a new [`Interactor`] instance. Information required to build the instance
    /// is collected in the command line arguments of the program.
    pub fn new() -> Interactor {
        // TODO: Implement Interactor::new
        unimplemented!()
    }

    /// Get the [`JudgeReader`] instance around the input file.
    pub fn input(&mut self) -> &mut JudgeReader {
        &mut self.input
    }

    /// Get the [`JudgeReader`] instance around the answer file.
    pub fn answer(&mut self) -> &mut JudgeReader {
        &mut self.answer
    }

    /// Get the [`JudgeReader`] instance around the read end of the pipe that connects
    /// the interactor and the user's program.
    pub fn read_end(&mut self) -> &mut JudgeReader {
        &mut self.read_end
    }

    /// Get the [`File`] instance that represents the write end of the pipe that
    /// connects the interactor and the user's program.
    pub fn write_end(&mut self) -> &mut File {
        &mut self.write_end
    }
}
