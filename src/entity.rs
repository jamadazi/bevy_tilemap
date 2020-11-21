use crate::{chunk::ChunkDimensions, lib::*, render::CHUNK_PIPELINE_HANDLE, Tilemap};

/// A component bundle for `Chunk` entities.
#[derive(Bundle)]
pub(crate) struct ChunkBundle {
    /// The handle for a TextureAtlas which contains multiple textures.
    pub(crate) texture_atlas: Handle<TextureAtlas>,
    /// The chunk's dimensions which are passed to the renderer.
    pub(crate) chunk_dimensions: ChunkDimensions,
    /// A component that indicates how to draw a component.
    pub(crate) draw: Draw,
    /// The pipeline for the renderer.
    pub(crate) render_pipelines: RenderPipelines,
    /// A component that indicates that an entity should be drawn in the
    /// "main pass"
    pub(crate) main_pass: MainPass,
    /// A mesh of vertices for a component.
    pub(crate) mesh: Handle<Mesh>,
    /// The transform location in a space for a component.
    pub(crate) transform: Transform,
    /// The global transform location in a space for a component.
    pub(crate) global_transform: GlobalTransform,
}

impl Default for ChunkBundle {
    fn default() -> ChunkBundle {
        let pipeline = RenderPipeline::new(CHUNK_PIPELINE_HANDLE);
        ChunkBundle {
            texture_atlas: Default::default(),
            chunk_dimensions: Default::default(),
            mesh: Default::default(),
            transform: Default::default(),
            render_pipelines: RenderPipelines::from_pipelines(vec![pipeline]),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            main_pass: MainPass,
            global_transform: Default::default(),
        }
    }
}

/// A component bundle for `Tilemap` entities.
#[derive(Debug, Bundle)]
pub struct TilemapBundle {
    /// A `Tilemap` which maintains chunks and its tiles.
    pub tilemap: Tilemap,
    /// The transform location in a space for a component.
    pub transform: Transform,
    /// The global transform location in a space for a component.
    pub global_transform: GlobalTransform,
}
