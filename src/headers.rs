// Copyright (c) the JPEG XL Project Authors. All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

extern crate jxl_headers_derive;

use jxl_headers_derive::JxlHeader;

pub mod encodings;
pub mod file_headers;

use crate::bit_reader::BitReader;
use crate::error::Error;

pub use encodings::JxlHeader;
pub use file_headers::*;

#[derive(JxlHeader)]
pub struct FileHeaders {
    #[allow(dead_code)]
    signature: Signature,
    pub size: Size,
    pub image_metadata: ImageMetadata,
}
