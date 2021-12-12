use crate::{
    inspector::handlers::node::base::handle_base_property_changed, make_command,
    scene::commands::light::*, SceneCommand,
};
use rg3d::scene::light::Light;
use rg3d::{
    core::pool::Handle,
    gui::inspector::{CollectionChanged, FieldKind, PropertyChanged},
    scene::{
        light::{
            directional::{CsmOptions, DirectionalLight, FrustumSplitOptions},
            point::PointLight,
            spot::SpotLight,
            BaseLight,
        },
        node::Node,
    },
};

pub fn handle_base_light_property_changed(
    args: &PropertyChanged,
    handle: Handle<Node>,
    node: &Node,
) -> Option<SceneCommand> {
    match args.value {
        FieldKind::Object(ref value) => match args.name.as_ref() {
            BaseLight::COLOR => {
                make_command!(SetLightColorCommand, handle, value)
            }
            BaseLight::CAST_SHADOWS => {
                make_command!(SetLightCastShadowsCommand, handle, value)
            }
            BaseLight::SCATTER => {
                make_command!(SetLightScatterCommand, handle, value)
            }
            BaseLight::SCATTER_ENABLED => {
                make_command!(SetLightScatterEnabledCommand, handle, value)
            }
            BaseLight::INTENSITY => {
                make_command!(SetLightIntensityCommand, handle, value)
            }
            _ => None,
        },
        FieldKind::Inspectable(ref inner) => match args.name.as_ref() {
            BaseLight::BASE => handle_base_property_changed(inner, handle, node),
            _ => None,
        },
        _ => None,
    }
}

pub fn handle_spot_light_property_changed(
    args: &PropertyChanged,
    handle: Handle<Node>,
    node: &Node,
) -> Option<SceneCommand> {
    if let Node::Light(Light::Spot(_)) = node {
        match args.value {
            FieldKind::Object(ref value) => match args.name.as_ref() {
                SpotLight::HOTSPOT_CONE_ANGLE => {
                    make_command!(SetSpotLightHotspotCommand, handle, value)
                }
                SpotLight::FALLOFF_ANGLE_DELTA => {
                    make_command!(SetSpotLightFalloffAngleDeltaCommand, handle, value)
                }
                SpotLight::SHADOW_BIAS => {
                    make_command!(SetSpotLightShadowBiasCommand, handle, value)
                }
                SpotLight::DISTANCE => {
                    make_command!(SetSpotLightDistanceCommand, handle, value)
                }
                SpotLight::COOKIE_TEXTURE => {
                    make_command!(SetSpotLightCookieTextureCommand, handle, value)
                }
                _ => None,
            },
            FieldKind::Inspectable(ref inner) => match args.name.as_ref() {
                SpotLight::BASE_LIGHT => handle_base_light_property_changed(inner, handle, node),
                _ => None,
            },
            _ => None,
        }
    } else {
        None
    }
}

pub fn handle_point_light_property_changed(
    args: &PropertyChanged,
    handle: Handle<Node>,
    node: &Node,
) -> Option<SceneCommand> {
    if let Node::Light(Light::Point(_)) = node {
        match args.value {
            FieldKind::Object(ref value) => match args.name.as_ref() {
                PointLight::SHADOW_BIAS => {
                    make_command!(SetPointLightShadowBiasCommand, handle, value)
                }
                PointLight::RADIUS => {
                    make_command!(SetPointLightRadiusCommand, handle, value)
                }
                _ => None,
            },
            FieldKind::Inspectable(ref inner) => match args.name.as_ref() {
                PointLight::BASE_LIGHT => handle_base_light_property_changed(inner, handle, node),
                _ => None,
            },
            _ => None,
        }
    } else {
        None
    }
}

pub fn handle_directional_light_property_changed(
    args: &PropertyChanged,
    handle: Handle<Node>,
    node: &Node,
) -> Option<SceneCommand> {
    if let Node::Light(Light::Directional(_)) = node {
        match args.value {
            FieldKind::Inspectable(ref inner) => match args.name.as_ref() {
                DirectionalLight::BASE_LIGHT => {
                    handle_base_light_property_changed(inner, handle, node)
                }
                DirectionalLight::CSM_OPTIONS => match inner.name.as_ref() {
                    CsmOptions::SPLIT_OPTIONS => match inner.value {
                        FieldKind::Inspectable(ref split_options_value) => {
                            if let FieldKind::Collection(ref collection_changed) =
                                split_options_value.value
                            {
                                if let CollectionChanged::ItemChanged { .. } = **collection_changed
                                {
                                    match split_options_value.name.as_ref() {
                                        FrustumSplitOptions::ABSOLUTE_FAR_PLANES => None,
                                        FrustumSplitOptions::RELATIVE_FRACTIONS => None,
                                        _ => None,
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    } else {
        None
    }
}