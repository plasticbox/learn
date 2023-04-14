use bevy::prelude::*;
use bevy::hierarchy::Children;
use bevy::hierarchy::BuildChildren;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) enum Set {
    ASet,
    BSet,
    CSet,
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
    query: Query<(&TestParent, Ref<Children>), Changed<Children>>,
) {
    query.iter().for_each(|(parent, children)| {
        if children.is_added() {
            return;
        }

        println!("change_detect parent: {:?}", parent);
        println!("change_detect children: {:?}", children);
    });
}

fn main() {

    let mut app = App::new();

    app.configure_sets((Set::ASet, Set::BSet, Set::CSet).chain().in_base_set(CoreSet::Update));

    app.add_system(apply_system_buffers.after(Set::ASet));
    app.add_system(apply_system_buffers.after(Set::BSet));
    app.add_system(apply_system_buffers.after(Set::CSet));

    app.add_system(spawn.in_set(Set::ASet));
    app.add_system(add_detect_and_remove.in_set(Set::BSet));
    app.add_system(change_detect.in_set(Set::CSet));

    println!("update 1");
    app.update();

    println!("\nupdate 2");
    app.update();

    println!("\nupdate 3");
    app.update();
}