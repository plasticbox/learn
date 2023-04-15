use bevy::hierarchy::BuildChildren;
use bevy::hierarchy::Children;
use bevy::prelude::*;

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

#[derive(Component, Debug)]
struct TestChildC;

fn spawn(mut cmds: Commands) {
    println!("spawn!");
    cmds.spawn(TestParent).with_children(|parent| {
        parent.spawn(TestChildA { 0: 1 });
        parent.spawn(TestChildB {
            0: "test".to_string(),
        });
    });
}

fn added_detect_and_remove(
    mut cmds: Commands,
    query: Query<(Entity, &TestParent, &Children), Added<Children>>,
    a_query: Query<&TestChildA>,
) {
    query.iter().for_each(|(entity, parent, children)| {
        println!("added_detect parent: {:?}", parent);
        println!("added_detect children: {:?}", children);
        children.iter().for_each(|child_entity| {
            if a_query.get(*child_entity).ok().is_some() {
                println!("remove child: {:?}", child_entity);
                cmds.entity(entity).remove_children(&[*child_entity]);
                cmds.entity(*child_entity).despawn();
            }
        });
    });
}

fn change_detect(query: Query<(&TestParent, Ref<Children>), Changed<Children>>) {
    query.iter().for_each(|(parent, children)| {
        if children.is_changed() {
            println!("change_detect parent: {:?}", parent);
            println!("change_detect children: {:?}", children);
            return;
        }
    });
}

fn add_child(
    mut cmds: Commands,
    query: Query<(Entity, &TestParent, &Children)>,
    mut is_run: Local<bool>,
) {
    if *is_run {
        return;
    }

    *is_run = true;
    query.iter().for_each(|(entity, _, _)| {
        let id = cmds.spawn(TestChildC).id();
        cmds.entity(entity).add_child(id);

        println!("add_child {:?}", id);
    });
}

fn main() {
    let mut app = App::new();

    app.configure_sets(
        (Set::ASet, Set::BSet, Set::CSet)
            .chain()
            .in_base_set(CoreSet::Update),
    );

    app.add_system(apply_system_buffers.after(Set::ASet));
    app.add_system(apply_system_buffers.after(Set::BSet));
    app.add_system(apply_system_buffers.after(Set::CSet));

    app.add_startup_system(spawn);
    app.add_system(added_detect_and_remove.in_set(Set::ASet));
    app.add_system(change_detect.in_set(Set::BSet));
    app.add_system(add_child.in_set(Set::CSet));

    println!("update 1");
    app.update();

    println!("\nupdate 2");
    app.update();

    println!("\nupdate 3");
    app.update();
}
