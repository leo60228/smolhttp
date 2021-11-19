use smolhttp_types::*;
use std::fmt;

mod sealed {
    use smolhttp_types::*;
    use std::fmt;

    pub trait Sealed {
        fn write_start_line(&self, w: &mut impl fmt::Write) -> fmt::Result;
        fn headers(&self) -> &[Header];
    }
}

use sealed::Sealed;

pub trait HttpMessage: Sealed {}

enum State<'a, T: HttpMessage> {
    StartLine(&'a T),
    Headers(&'a Header, &'a [Header]),
    Crlf,
    End,
}

impl<'a, T: HttpMessage> From<&'a [Header]> for State<'a, T> {
    fn from(headers: &'a [Header]) -> Self {
        if let Some((head, tail)) = headers.split_first() {
            Self::Headers(head, tail)
        } else {
            Self::Crlf
        }
    }
}

pub struct Writer<'a, T: HttpMessage> {
    state: State<'a, T>,
}

impl<'a, T: HttpMessage> Writer<'a, T> {
    pub fn new(message: &'a T) -> Self {
        Self {
            state: State::StartLine(message),
        }
    }

    pub fn write_line(&mut self, w: &mut impl fmt::Write) -> Result<Option<()>, fmt::Error> {
        let (new_state, elem) = match self.state {
            State::StartLine(message) => {
                message.write_start_line(w)?;

                (message.headers().into(), Some(()))
            }
            State::Headers(head, tail) => {
                write!(w, "{}\r\n", head)?;

                (tail.into(), Some(()))
            }
            State::Crlf => {
                write!(w, "\r\n")?;

                (State::End, None)
            }
            State::End => (State::End, None),
        };
        self.state = new_state;
        Ok(elem)
    }

    pub fn write_all(&mut self, w: &mut impl fmt::Write) -> fmt::Result {
        while let Some(()) = self.write_line(w)? {}

        Ok(())
    }
}

impl<'a, T: HttpMessage> Iterator for Writer<'a, T> {
    type Item = Result<String, fmt::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();

        self.write_line(&mut buf)
            .map(|opt| opt.map(|()| buf))
            .transpose()
    }
}
