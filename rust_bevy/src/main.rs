use bevy::prelude::*;
use bevy::hierarchy::Children;
use bevy::hierarchy::BuildChildren;

#[derive(StageLabel)]
pub(crate) enum Stage {
    AStage,
    BStage,
}

#[derive(Component, Debug)]
struct TestParent;

#[derive(Component, Debug)]
struct TestChildA(i32);

#[derive(Component, Debug)]
struct TestChildB(String);

fn spawn(mut cmds: Commands, mut is_run: Local<bool>) {
    if !*is_run {
        println!("spawn!");
        cmds.spawn(TestParent)
            .with_children(|parent| {
                parent.spawn(TestChildA {0: 1});
                parent.spawn(TestChildB {0: "test".to_string()});
            });

        *is_run = true;
    }
}

fn add_detect_and_remove(mut cmds: Commands, query: Query<(Entity, &TestParent, &Children), Added<Children>>, a_query: Query<&TestChildA>) {
    query.iter().for_each(|(entity, parent, children)| {
        println!("add_detect parent: {:?}", parent);
        println!("add_detect children: {:?}", children);
        children.iter().for_each(|child_entity| {
            println!("add_detect child: {:?}", child_entity);
            if a_query.get(*child_entity).ok().is_some() {
                cmds.entity(entity).remove_children(&[*child_entity]);
                cmds.entity(*child_entity).despawn();
            }
        });
    });
}

fn change_detect(
    query: Query<(&TestParent, &Children, ChangeTrackers<Children>), Changed<Children>>,
) {
    query.iter().for_each(|(parent, children, tracker)| {
        if tracker.is_added() {
            return;
        }

        println!("change_detect parent: {:?}", parent);
        println!("change_detect children: {:?}", children);
    });
}

fn main() {

    let mut app = App::new();

    app.add_stage(Stage::AStage, SystemStage::parallel());
    app.add_stage(Stage::BStage, SystemStage::parallel());
    app.add_system_to_stage(Stage::AStage, spawn);
    app.add_system_to_stage(Stage::BStage, add_detect_and_remove);
    app.add_system_to_stage(Stage::BStage, change_detect);

    println!("update 1");
    app.update();

    println!("\nupdate 2");
    app.update();

    println!("\nupdate 3");
    app.update();
}