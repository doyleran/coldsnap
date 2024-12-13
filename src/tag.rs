// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*!
Parse EBS tags from the command line
*/

use aws_sdk_ebs::types::Tag as EbsTag;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeyValueRecord {
    pub key: String,
    pub value: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct TagVec(Vec<KeyValueRecord>);

impl FromStr for TagVec {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str::<TagVec>(s)
    }
}

impl Display for TagVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or(format!("{:?}", self))
        )
    }
}

impl Deref for TagVec {
    type Target = Vec<KeyValueRecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TagVec {
    pub fn to_ebs_tags(&self) -> Vec<EbsTag> {
        self.iter()
            .map(|v| {
                EbsTag::builder()
                    .key(v.key.clone())
                    .value(v.value.clone())
                    .build()
            })
            .collect()
    }
}
