use std::{collections::HashSet, sync::Arc};

use anyhow::Context;
use context::PipelineCtx;
use elements_asset_cache::SyncAssetKey;
use elements_std::{
    asset_cache::AssetCache, asset_url::{AbsAssetUrl, AssetType}
};
use futures::{future::BoxFuture, StreamExt};
use image::ImageFormat;
use out_asset::{OutAsset, OutAssetContent, OutAssetPreview};
use serde::{Deserialize, Serialize};

use self::{materials::MaterialsPipeline, models::ModelsPipeline, out_asset::asset_id_from_url};

pub mod audio;
pub mod context;
pub mod materials;
pub mod models;
pub mod out_asset;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PipelineConfig {
    Models(ModelsPipeline),
    ScriptBundles,
    Materials(MaterialsPipeline),
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Pipeline {
    pub pipeline: PipelineConfig,
    /// Filter sources; this is a list of glob patterns for accepted files
    /// All files are accepted if this is empty
    #[serde(default)]
    pub sources: Vec<String>,
    /// Tags to apply to the output resources
    #[serde(default)]
    pub tags: Vec<String>,
    /// Categories ot apply to the output resources
    #[serde(default)]
    pub categories: Vec<Vec<String>>,
}
impl Pipeline {
    pub async fn process(&self, ctx: PipelineCtx) -> Vec<OutAsset> {
        let mut assets = match &self.pipeline {
            PipelineConfig::Models(config) => models::pipeline(&ctx, config.clone()).await,
            PipelineConfig::ScriptBundles => {
                ctx.process_files(
                    |f| f.extension() == Some("script_bundle".to_string()),
                    |ctx, file| async move {
                        let bundle = file.download_bytes(&ctx.process_ctx.assets).await.unwrap();
                        let content =
                            ctx.write_file(ctx.in_root().relative_path(file.path()).with_extension("script_bundle"), bundle).await;

                        Ok(vec![OutAsset {
                            id: asset_id_from_url(&file),
                            type_: AssetType::ScriptBundle,
                            hidden: false,
                            name: file.path().file_name().unwrap().to_string(),
                            tags: Vec::new(),
                            categories: Default::default(),
                            preview: OutAssetPreview::None,
                            content: OutAssetContent::Content(content),
                            source: Some(file.clone()),
                        }])
                    },
                )
                .await
            }
            PipelineConfig::Materials(config) => materials::pipeline(&ctx, config.clone()).await,
            PipelineConfig::Audio => audio::pipeline(&ctx).await,
        };
        for asset in &mut assets {
            asset.tags.extend(self.tags.clone());
            for i in 0..asset.categories.len() {
                if let Some(cat) = self.categories.get(i) {
                    asset.categories[i].extend(cat.iter().cloned().collect::<HashSet<_>>());
                }
            }
        }
        assets
    }
}

pub async fn process_pipelines(ctx: &ProcessCtx) -> Vec<OutAsset> {
    log::info!("Processing pipeline with out_root={}", ctx.out_root);
    futures::stream::iter(ctx.files.iter())
        .filter_map(|file| async move {
            let pipelines: Vec<Pipeline> = if file.0.path().ends_with("pipeline.json") {
                file.download_json(&ctx.assets).await.unwrap()
            } else {
                return None;
            };
            Some((file, pipelines))
        })
        .flat_map(|(file, pipelines)| {
            futures::stream::iter(pipelines.into_iter().enumerate().map(|(i, pipeline)| {
                let mut file = file.clone();
                file.0.set_fragment(Some(&i.to_string()));
                (file, pipeline)
            }))
        })
        .map(|(pipeline_file, pipeline)| {
            let root = pipeline_file.join(".").unwrap();
            let ctx = PipelineCtx {
                process_ctx: ctx.clone(),
                pipeline: Arc::new(pipeline.clone()),
                pipeline_file,
                root_path: ctx.in_root.relative_path(root.path()),
            };
            tokio::spawn(async move { pipeline.process(ctx).await })
        })
        .buffered(30)
        .map(|x| x.unwrap())
        .flat_map(|out_assets| futures::stream::iter(out_assets.into_iter()))
        .collect::<Vec<_>>()
        .await
}

#[derive(Debug, Clone)]
pub struct ProcessCtxKey;
impl SyncAssetKey<ProcessCtx> for ProcessCtxKey {}

#[derive(Clone)]
pub struct ProcessCtx {
    pub assets: AssetCache,
    pub files: Arc<Vec<AbsAssetUrl>>,
    pub input_file_filter: Option<String>,
    pub package_name: String,
    pub in_root: AbsAssetUrl,
    pub out_root: AbsAssetUrl,
    pub write_file: Arc<dyn Fn(String, Vec<u8>) -> BoxFuture<'static, AbsAssetUrl> + Sync + Send>,
    pub on_status: Arc<dyn Fn(String) -> BoxFuture<'static, ()> + Sync + Send>,
    pub on_error: Arc<dyn Fn(anyhow::Error) -> BoxFuture<'static, ()> + Sync + Send>,
}
impl ProcessCtx {
    pub fn has_input_file(&self, url: &AbsAssetUrl) -> bool {
        self.files.iter().any(|x| x == url)
    }
    pub fn find_file_res(&self, glob_pattern: impl AsRef<str>) -> anyhow::Result<&AbsAssetUrl> {
        self.find_file(&glob_pattern).with_context(|| format!("Failed to find file with pattern {}", glob_pattern.as_ref()))
    }
    pub fn find_file(&self, glob_pattern: impl AsRef<str>) -> Option<&AbsAssetUrl> {
        let pattern = glob::Pattern::new(glob_pattern.as_ref()).unwrap();
        self.files.iter().find(|f| pattern.matches(f.path().as_str()))
    }
}

pub async fn download_image(assets: &AssetCache, url: &AbsAssetUrl) -> anyhow::Result<image::DynamicImage> {
    let data = url.download_bytes(assets).await?;
    if let Some(format) = url.extension().as_ref().and_then(|ext| ImageFormat::from_extension(ext)) {
        Ok(image::load_from_memory_with_format(&data, format).with_context(|| format!("Failed to load image {url}"))?)
    } else {
        Ok(image::load_from_memory(&data).with_context(|| format!("Failed to load image {url}"))?)
    }
}
