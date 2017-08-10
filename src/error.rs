// Copyright 2017 Pablo Couto

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// version 3 as published by the Free Software Foundation.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License version 3 for more details.

// You should have received a copy of the GNU Lesser General Public
// License version 3 along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

use crest;
use hyper;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        Crest(crest::Error, crest::ErrorKind);
    }

    foreign_links {}

    errors {
        NoAuth {
            description("Missing authorization token")
        }
        Timeout {
            description("Operation timed out")
        }
        UnexpectedStatus(recv: hyper::StatusCode, req: hyper::StatusCode) {
            description("Unexpected status received")
            display("Expected status ‘{}’; received ‘{}’", req, recv)
        }
    }
}
