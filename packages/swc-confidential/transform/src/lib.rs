#![feature(box_patterns)]

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::Deserialize;
use swc_common::{comments::Comments, Spanned};
use swc_ecma_visit::VisitMut;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub algorithm: Algorithm,
    pub encryption_key: String,
    #[serde(default)]
    pub prefix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Algorithm {
    #[serde(rename = "AES-128")]
    AES128,
    #[serde(rename = "AES-192")]
    AES192,
    #[serde(rename = "AES-256")]
    AES256,
}

fn encode_hex(bytes: Vec<u8>) -> String {
    hex::encode(bytes)
}

impl Algorithm {
    fn encrypt(&self, key: &str, value: &str) -> Result<String, String> {
        match self {
            Algorithm::AES128 => {
                let mc = new_magic_crypt!(key, 256);

                Ok(encode_hex(mc.encrypt_str_to_bytes(value)))
            }
            Algorithm::AES192 => {
                let mc = new_magic_crypt!(key, 192);

                Ok(encode_hex(mc.encrypt_str_to_bytes(value)))
            }
            Algorithm::AES256 => {
                let mc = new_magic_crypt!(key, 256);

                Ok(encode_hex(mc.encrypt_str_to_bytes(value)))
            }
        }
    }
}

pub fn swc_confidential<C>(config: Config, comments: C) -> impl VisitMut
where
    C: Comments,
{
    SwcConfidential { config, comments }
}

/// Handles functions from `@swc/magic`.
struct SwcConfidential<C>
where
    C: Comments,
{
    config: Config,
    comments: C,
}

impl<C> VisitMut for SwcConfidential<C>
where
    C: Comments,
{
    fn visit_mut_str(&mut self, n: &mut swc_ecma_ast::Str) {
        if self.comments.has_flag(n.span_lo(), "CONFIDENTIAL") {
            self.comments.take_leading(n.span_lo());

            let encrypted = self
                .config
                .algorithm
                .encrypt(&self.config.encryption_key, &n.value)
                .expect("failed to encrypt");

            n.raw = None;
            n.value = format!("{}{}", self.config.prefix, encrypted).into();
        }
    }
}
