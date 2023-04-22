use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CharacterPlugin)
        .add_startup_system(setup)
        .run();
}

struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(hello_world)
            .add_system(print_name)
            .add_system(character_with_role)
            .add_system(character_without_a_role)
            .add_system(print_character_role);
    }

    fn is_unique(&self) -> bool {
        true
    }

    fn name(&self) -> &str {
        "Character Plugin"
    }
}

fn hello_world() -> () {
    println!("Hello World!");
}

fn setup(mut commands: Commands) {
    let characters = vec![
        ("Melee", RoleEnum::Melee),
        ("Ranged", RoleEnum::Ranged),
        ("Tank", RoleEnum::Tank),
    ];

    characters.into_iter().for_each(|character| {
        let (name, role) = character;

        let character = Character {
            name: name.to_owned(),
        };

        let character_role = Role { role };

        commands.spawn((character, character_role));
    });

    commands.spawn(Character {
        name: "No Role".to_owned(),
    });
}

fn print_name(character_query: Query<&Character>) {
    for character in character_query.iter() {
        println!("Name: {}", character.name);
    }
}

fn character_with_role(character_query: Query<&Character, With<Role>>) {
    for character in character_query.iter() {
        println!("Character with role: {}", character.name)
    }
}

fn character_without_a_role(character_query: Query<&Character, Without<Role>>) {
    for character in character_query.iter() {
        println!("Character without a role: {}", character.name);
    }
}

fn print_character_role(character_query: Query<(&Character, &Role)>) {
    for (character, role) in character_query.iter() {
        let role_name = match role.role {
            RoleEnum::Tank => "tank",
            RoleEnum::Melee => "melee",
            RoleEnum::Ranged => "ranged",
        };

        println!(
            "The character {} has the role {}",
            character.name, role_name
        )
    }
}

#[derive(Component)]
struct Character {
    name: String,
}

#[derive(Component)]
struct Role {
    role: RoleEnum,
}

enum RoleEnum {
    Ranged,
    Melee,
    Tank,
}
