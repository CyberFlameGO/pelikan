// Copyright 2020 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use serde::{Deserialize, Serialize};

// definitions
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TlsConfig {
    #[serde(default)]
    certificate_chain: Option<String>,
    #[serde(default)]
    private_key: Option<String>,
    #[serde(default)]
    certificate: Option<String>,
    #[serde(default)]
    ca_file: Option<String>,
}

// implementation
impl TlsConfig {
    pub fn certificate_chain(&self) -> Option<String> {
        self.certificate_chain.clone()
    }

    pub fn private_key(&self) -> Option<String> {
        self.private_key.clone()
    }

    pub fn certificate(&self) -> Option<String> {
        self.certificate.clone()
    }

    pub fn ca_file(&self) -> Option<String> {
        self.ca_file.clone()
    }
}
