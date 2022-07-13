use std::path::Path;

use serde::Deserialize;
use swc_emotion::EmotionOptions;
use swc_plugin::{ast::*, plugin_transform, TransformPluginProgramMetadata};

pub struct TransformVisitor;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PluginContext {
    filename: Option<String>,
    env_name: String,
}

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
}

// This config transformation has to be the same as https://github.com/vercel/next.js/blob/9fe2f2637c8384ae7939d5a4a30f1557a4262acb/packages/next/build/swc/options.js#L115-L140
impl EmotionJsOptions {
    fn to_emotion_options(self, env_name: &str) -> EmotionOptions {
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
                    EmotionJsAutoLabel::DevOnly => match env_name {
                        "development" => true,
                        _ => false,
                    },
                },
            ),
            label_format: Some(self.label_format.unwrap_or("[local]".to_string())),
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<EmotionJsOptions>(&data.plugin_config)
        .expect("invalid config for emotion");

    let context = serde_json::from_str::<PluginContext>(&data.transform_context)
        .expect("Invalid plugin context");

    let config = config.to_emotion_options(&context.env_name);
    let file_name = context.filename.unwrap_or_else(String::new);
    let path = Path::new(&file_name);
    let source_map = std::sync::Arc::new(data.source_map);

    let program = program.fold_with(&mut swc_emotion::emotion(
        config,
        path,
        source_map,
        data.comments,
    ));

    program
}
