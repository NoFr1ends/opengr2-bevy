//
use std::path::Path;

//
use crate::Granny;

//
use anyhow::{bail, Context};

use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, Handle, LoadContext};
use bevy::hierarchy::BuildWorldChildren;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{SpatialBundle, World};
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::texture::{CompressedImageFormats, Image, ImageSampler, ImageType};
use bevy::scene::Scene;
use bevy::utils::default;

//
use opengr2::parser::{Element, ElementType};
use opengr2::{GrannyFile, GrannyResolve};

#[derive(Default)]
pub struct GrannyLoader;

impl AssetLoader for GrannyLoader {
    type Asset = Granny;
    type Settings = ();
    type Error = anyhow::Error;

    fn extensions(&self) -> &[&str] {
        &["gr2"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut buf = vec![];
            reader.read_to_end(&mut buf).await?;

            Ok(load_granny(&buf, load_context).await?)
        })
    }
}

struct GrannyMesh {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Option<Handle<StandardMaterial>>,
}

async fn load_granny_material(
    material: &Vec<Element>,
    i: usize,
    load_context: &mut LoadContext<'_>,
) -> anyhow::Result<Handle<StandardMaterial>> {
    let mut texture = None;

    if let Some(ElementType::Reference(map)) = material.resolve("Maps").map(|e| &e.element) {
        if let Some(ElementType::String(usage)) = map.resolve("Usage").map(|e| &e.element) {
            if usage == "Diffuse Color" {
                if let Some(ElementType::String(file_name)) =
                    map.resolve("Map.Texture.FromFileName").map(|e| &e.element)
                {
                    let parent = load_context.path().parent().unwrap();
                    let image_path = parent.join(file_name);
                    let bytes = load_context.read_asset_bytes(image_path.clone()).await?;

                    let extension = Path::new(file_name).extension().unwrap().to_str().unwrap();

                    let image = Image::from_buffer(
                        &bytes,
                        ImageType::Extension(extension),
                        CompressedImageFormats::default(),
                        true,
                        ImageSampler::nearest(),
                        RenderAssetUsages::default(),
                    )?;

                    texture = Some(load_context.add_labeled_asset(file_name.to_string(), image));
                }
            }
        }
    }

    let material_asset = //LoadedAsset::new_with_dependencies(
        StandardMaterial { base_color_texture: texture, ..default() }
    //)
    ;
    let material_handle = load_context.add_labeled_asset(format!("Material{i}"), material_asset);

    Ok(material_handle)
}

async fn load_granny<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> anyhow::Result<Granny> {
    let granny_file = GrannyFile::load_from_bytes(bytes).context("Malformed file provided")?;

    let mut default_scene = None;
    let mut scenes = Vec::new();

    if let Some(ElementType::ArrayOfReferences(models)) =
        granny_file.find_element("Models").map(|e| &e.element)
    {
        for (i, model) in models.iter().enumerate() {
            let mut meshes = Vec::new();

            if let Some(ElementType::Reference(mesh)) =
                model.resolve("MeshBindings.Mesh").map(|e| &e.element)
            {
                let mut bevy_mesh = Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::default(),
                );
                let mut bevy_material = None;

                if let Some(ElementType::ArrayOfReferences(vertices)) = mesh
                    .resolve("PrimaryVertexData.Vertices")
                    .map(|e| &e.element)
                {
                    let mut positions = Vec::<[f32; 3]>::new();
                    let mut normals = Vec::<[f32; 3]>::new();
                    let mut uvs = Vec::<[f32; 2]>::new();

                    for vertex in vertices {
                        if let Some(ElementType::Array(position)) =
                            &vertex.resolve("Position").map(|e| &e.element)
                        {
                            assert_eq!(position.len(), 3);
                            let position = position
                                .iter()
                                .map(|p| {
                                    if let ElementType::F32(pos) = p {
                                        *pos
                                    } else {
                                        0.0
                                    }
                                })
                                .collect::<Vec<_>>();
                            positions.push([position[0], position[1], position[2]]);
                        } else {
                            bail!("No position in vertex")
                        }

                        if let Some(ElementType::Array(normal)) =
                            &vertex.resolve("Normal").map(|e| &e.element)
                        {
                            assert_eq!(normal.len(), 3);
                            let normal = normal
                                .iter()
                                .map(|n| {
                                    if let ElementType::F32(nor) = n {
                                        *nor
                                    } else {
                                        0.0
                                    }
                                })
                                .collect::<Vec<_>>();
                            normals.push([normal[0], normal[1], normal[2]]);
                        } else {
                            bail!("No normal in vertex")
                        }

                        if let Some(ElementType::Array(uv)) =
                            &vertex.resolve("TextureCoordinates0").map(|e| &e.element)
                        {
                            assert_eq!(uv.len(), 2);
                            let uv = uv
                                .iter()
                                .map(|n| if let ElementType::F32(n) = n { *n } else { 0.0 })
                                .collect::<Vec<_>>();
                            uvs.push([uv[0], uv[1]]);
                        }
                    }

                    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                } else {
                    bail!("Missing Vertices")
                }

                if let Some(ElementType::Reference(indices)) =
                    mesh.resolve("PrimaryTopology.Indices").map(|e| &e.element)
                {
                    let indices = indices
                        .iter()
                        .map(|i| {
                            if let ElementType::I32(idx) = &i.element {
                                *idx as u32
                            } else {
                                0
                            }
                        })
                        .collect::<Vec<_>>();

                    bevy_mesh.insert_indices(Indices::U32(indices));
                } else {
                    bail!("Missing Indices")
                }

                if let Some(ElementType::Reference(material)) = mesh
                    .resolve("MaterialBindings.Material")
                    .map(|e| &e.element)
                {
                    bevy_material = load_granny_material(material, i, load_context).await.ok();
                }

                let bevy_mesh = load_context.add_labeled_asset(format!("Mesh{i}"), bevy_mesh);

                meshes.push(GrannyMesh {
                    mesh_handle: bevy_mesh,
                    material_handle: bevy_material,
                });
            } else {
                bail!("Missing MeshBindings")
            }

            let mut world = World::default();
            world
                .spawn(SpatialBundle::default())
                .with_children(|parent| {
                    for mesh in meshes {
                        parent.spawn(PbrBundle {
                            mesh: mesh.mesh_handle,
                            material: mesh.material_handle.unwrap_or_default(),
                            ..default()
                        });
                    }
                });

            let name = if let Some(ElementType::String(scene_name)) =
                model.resolve("Name").map(|e| &e.element)
            {
                scene_name.clone()
            } else {
                format!("Scene{i}")
            };

            println!("loaded scene {}", name);

            let scene_handle = load_context.add_labeled_asset(name, Scene::new(world));
            scenes.push(scene_handle);
        }
    } else {
        bail!("Missing Models")
    }

    if scenes.len() > 0 {
        default_scene = Some(scenes[0].clone());
    }

    Ok(Granny {
        default_scene,
        scenes,
    })
}
