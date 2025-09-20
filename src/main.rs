use std::collections::HashMap;

use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    color::{Color, Srgba},
    core_pipeline::core_2d::Camera2d,
    ecs::{children, component::Component, system::Commands},
    prelude::*,
    ui::{BackgroundColor, Node, Val},
    utils::default,
};
use strum::{EnumIter, IntoEnumIterator};

use crate::ui::{
    button::{ButtonState, button},
    label::label,
    ui_plugin,
};

mod ui;

struct ResourceDescription<'a> {
    resource: ResourceType,
    label: &'a str,
    button: &'a str,
    cost: Option<(ResourceType, f64)>,
}

fn description(resource_type: ResourceType) -> ResourceDescription<'static> {
    match resource_type {
        ResourceType::Thing => ResourceDescription {
            resource: ResourceType::Thing,
            label: "Things",
            button: "Make Thing!",
            cost: None,
        },
        ResourceType::ThingMaker => ResourceDescription {
            resource: ResourceType::ThingMaker,
            label: "Things Makers",
            button: "Make Thing Maker!",
            cost: Some((ResourceType::Thing, 10.0)),
        },
        ResourceType::Assembler => ResourceDescription {
            resource: ResourceType::Assembler,
            label: "Assemblers",
            button: "Assemble Assembler",
            cost: Some((ResourceType::Thing, 100.0)),
        },
        ResourceType::AssemblyLine => ResourceDescription {
            resource: ResourceType::AssemblyLine,
            label: "Assembly Lines",
            button: "Line up Assemblers",
            cost: Some((ResourceType::Thing, 1_000.0)),
        },
        ResourceType::Learning => ResourceDescription {
            resource: ResourceType::Learning,
            label: "Learning",
            button: "Think",
            cost: Some((ResourceType::Assembler, 100.0)),
        },
        ResourceType::AutomatedLearning => ResourceDescription {
            resource: ResourceType::AutomatedLearning,
            label: "Automated Learning",
            button: "Automate",
            cost: Some((ResourceType::Learning, 100.0)),
        },
        ResourceType::Ai => ResourceDescription {
            resource: ResourceType::Ai,
            label: "AI",
            button: "Automate more",
            cost: Some((ResourceType::Assembler, 1_000_000.0)),
        },
        ResourceType::Agi => ResourceDescription {
            resource: ResourceType::Agi,
            label: "AGI",
            button: "Give up contorl",
            cost: Some((ResourceType::Ai, 100.0)),
        },
    }
}

#[derive(Component, Debug)]
struct Resources {
    amounts: HashMap<ResourceType, f64>,
}

impl Resources {
    pub fn new() -> Self {
        let mut ret = Resources {
            amounts: HashMap::new(),
        };
        ret.reset();
        ret
    }

    pub fn get(&self, resource: ResourceType) -> f64 {
        *self.amounts.get(&resource).unwrap()
    }

    pub fn increase(&mut self, resource: ResourceType, amount: f64) {
        let current = self.amounts.get(&resource).unwrap();
        self.amounts.insert(resource, current + amount);
    }

    pub fn reset(&mut self) {
        for t in ResourceType::iter() {
            self.amounts.insert(t, 0.0);
        }
    }
}

#[derive(Component)]
struct ResourceRender(ResourceType);
#[derive(Component)]
struct ResourceUpdate(ResourceType);

#[derive(Component)]
struct IsEnabled {
    resource: ResourceType,
    enabled: bool,
}

#[derive(PartialEq, Eq, Hash, EnumIter, Clone, Copy, Debug)]
pub enum ResourceType {
    Thing,
    ThingMaker,
    Assembler,
    AssemblyLine,
    Learning,
    AutomatedLearning,
    Ai,
    Agi,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ui_plugin)
        .add_systems(Startup, (create_ui, create_resources))
        .add_systems(Update, (button_listener_system, render_resource_system))
        .add_systems(Update, (update_resource_system, update_button_state_system))
        .add_systems(Update, (enable_resources_system, update_enabled_visibility))
        .run();
}

fn create_resources(mut commands: Commands) {
    commands.spawn(Resources::new());
}

fn create_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                align_items: AlignItems::Start,
                align_content: AlignContent::FlexStart,
                padding: UiRect::all(Val::Px(30.0)),
                row_gap: Val::Px(40.0),
                column_gap: Val::Px(7.0),
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba::rgb(1.0, 1.0, 1.0))),
        ))
        .with_children(|parent| {
            for r in ResourceType::iter() {
                let description = description(r);
                parent.spawn(resource_tile(&description));
            }
        });
}

fn resource_tile(resource_type: &ResourceDescription) -> impl Bundle + use<> {
    let cost_label = resource_type
        .cost
        .map(|c| format!("{} {}", c.1, description(c.0).label))
        .unwrap_or("Free".into());
    (
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        IsEnabled {
            resource: resource_type.resource,
            enabled: false,
        },
        BackgroundColor(Color::Srgba(Srgba::rgb(0.95, 0.95, 0.95))),
        children![
            label(resource_type.label),
            (
                label("0"),
                ResourceRender(resource_type.resource),
                Node {
                    align_self: AlignSelf::FlexEnd,
                    ..default()
                }
            ),
            label("Cost:"),
            label(&cost_label),
            (
                ResourceUpdate(resource_type.resource),
                button(resource_type.button)
            )
        ],
    )
}

fn button_listener_system(
    button_query: Populated<(&ResourceUpdate, &Interaction, &ButtonState), Changed<Interaction>>,
    resources: Single<&mut Resources>,
) {
    let mut resources = resources;
    for (ResourceUpdate(resource_type), interaction, button_state) in button_query.iter() {
        if button_state.enabled {
            if matches!(interaction, Interaction::Pressed) {
                resources.increase(*resource_type, 1.0);
                if let Some((cost_type, amount)) = description(*resource_type).cost {
                    resources.increase(cost_type, -amount);
                }
            }
        }
    }
}

fn render_resource_system(
    entities: Query<(&mut Text, &ResourceRender)>,
    resources: Single<&mut Resources, Changed<Resources>>,
) {
    for (mut text, ResourceRender(resource)) in entities {
        **text = resources.get(*resource).floor().to_string();
    }
}

fn update_button_state_system(
    mut buttons: Populated<(&mut ButtonState, &ResourceUpdate), With<Button>>,
    resources: Single<&Resources>,
) {
    for (mut state, ResourceUpdate(resource_type)) in buttons.iter_mut() {
        state.enabled = description(*resource_type)
            .cost
            .map(|c| is_cost_met(&resources, c))
            .unwrap_or(true);
    }
}

fn enable_resources_system(
    panels: Query<&mut IsEnabled>,
    resources: Single<&Resources, Changed<Resources>>,
) {
    for mut panel in panels {
        let description = description(panel.resource);
        let met = description
            .cost
            .map(|c| is_cost_met(*resources, c))
            .unwrap_or(true);
        if met {
            panel.enabled = true;
        }
    }
}

fn update_enabled_visibility(panels: Query<(&mut Visibility, &IsEnabled), Changed<IsEnabled>>) {
    for (mut visibility, is_enabled) in panels {
        if is_enabled.enabled {
            *visibility = Visibility::Inherited
        } else {
            *visibility = Visibility::Hidden
        }
    }
}

fn update_resource_system(mut resources: Single<&mut Resources>, time: Res<Time>) {
    if resources.get(ResourceType::Agi) > 0.0 {
        resources.reset();
    }

    let seconds = time.delta_secs_f64();
    add_resources(
        ResourceType::ThingMaker,
        ResourceType::Thing,
        &mut resources,
        seconds,
    );
    add_resources(
        ResourceType::Assembler,
        ResourceType::ThingMaker,
        &mut resources,
        seconds,
    );
    add_resources(
        ResourceType::AssemblyLine,
        ResourceType::Assembler,
        &mut resources,
        seconds,
    );
    add_resources(
        ResourceType::Learning,
        ResourceType::AssemblyLine,
        &mut resources,
        seconds,
    );
    add_resources(
        ResourceType::AutomatedLearning,
        ResourceType::Learning,
        &mut resources,
        seconds,
    );
    add_resources(
        ResourceType::Ai,
        ResourceType::AutomatedLearning,
        &mut resources,
        seconds,
    );
}

fn add_resources(
    producer: ResourceType,
    produced: ResourceType,
    resources: &mut Single<&mut Resources>,
    seconds: f64,
) {
    let thing_makers = resources.get(producer);
    resources.increase(produced, thing_makers * seconds);
}

fn is_cost_met(resources: &Resources, (resource, amount): (ResourceType, f64)) -> bool {
    resources.get(resource) >= amount
}
