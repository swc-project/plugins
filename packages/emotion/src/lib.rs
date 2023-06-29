#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::path::Path;

use serde::Deserialize;
use swc_common::SourceMapper;
use swc_core::{
    common::Spanned,
    ecma::{ast::Program, visit::FoldWith},
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};
use swc_emotion::EmotionOptions;
pub struct TransformVisitor;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum EmotionJsAutoLabel {
    Never,
    DevOnly,
    Always,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmotionJsOptions {
    source_map: Option<bool>,
    auto_label: Option<EmotionJsAutoLabel>,
    label_format: Option<String>,
    #[serde(flatten)]
    extra: swc_emotion::EmotionOptions,
}

// This config transformation has to be the same as https://github.com/vercel/next.js/blob/9fe2f2637c8384ae7939d5a4a30f1557a4262acb/packages/next/build/swc/options.js#L115-L140
impl EmotionJsOptions {
    fn into_emotion_options(self, env_name: &str) -> EmotionOptions {
        EmotionOptions {
            enabled: Some(true),
            sourcemap: Some(match env_name {
                "development" => self.source_map.unwrap_or(true),
                _ => false,
            }),
            auto_label: Some(
                match self.auto_label.unwrap_or(EmotionJsAutoLabel::DevOnly) {
                    EmotionJsAutoLabel::Always => true,
                    EmotionJsAutoLabel::Never => false,
                    EmotionJsAutoLabel::DevOnly => matches!(env_name, "development"),
                },
            ),
            label_format: Some(self.label_format.unwrap_or_else(|| "[local]".to_string())),
            ..self.extra
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<EmotionJsOptions>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for emotion"),
    )
    .expect("invalid config for emotion");

    let config = config.into_emotion_options(
        &data
            .get_context(&TransformPluginMetadataContextKind::Env)
            .unwrap_or_default(),
    );
    let file_name = data
        .get_context(&TransformPluginMetadataContextKind::Filename)
        .unwrap_or_default();
    let path = Path::new(&file_name);
    let source_map = std::sync::Arc::new(data.source_map);
    let pos = source_map.lookup_char_pos(program.span().lo);
    let hash = pos.file.src_hash as u32;
    program.fold_with(&mut swc_emotion::emotion(
        config,
        path,
        hash,
        source_map,
        data.comments,
    ))
}
