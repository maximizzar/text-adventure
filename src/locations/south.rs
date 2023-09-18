use crate::{locations, Npc};

const CURRENT_CARDINAL_POINT: &str = "south";

pub(crate) fn forest() {
    const CURRENT_LOCATION: &str = "forest";

    println!("
    As you wander further into the dark and gloomy forest the trees begin to look even worse, like they've gotten there life sucked out of them and the mist seems to thicken.
	You wander until you can make out a single person standing in the midst of it all. The air seems to get heavier.
	!{:?}, {}, is here.", NPC_GHORKAN.name, NPC_GHORKAN.title);

    let mut player_action:String = String::new();

    loop {
        locations::map(CURRENT_LOCATION,CURRENT_CARDINAL_POINT);
        let ret: &str = forest_player_action(&*player_action);
        if ret.contains("break") {
            break;
        }
    }
}
fn forest_player_action(player_action: &str) -> &'static str {
    if player_action.contains("go.") {

        if player_action.eq("go.north") {
            return "break";
        }

    } else if player_action.contains("talk.") {

        if player_action.eq(&format!("talk.{}", NPC_GHORKAN.name)) {
            println!("... you try to reason with Ghorkan, he seems a bit irritated but ignores you.");
            return "";
        }

    } else if player_action.contains("attack.") {

        if player_action.eq(&format!("attack.{}", NPC_GHORKAN.name)) {
            println!("You decide to Attack {} the {}.", NPC_GHORKAN.name, NPC_GHORKAN.title);

            std::process::exit(0);
        }

    }


    return "";
}
fn attack_ghorkan() {
    //sword
    println!("...	your measly sword is useless against the evil Ghorkan and you are cursed by a ban.");
    //rare sword
    println!("
    ... You enter the foreboding forest...
    Your heartbeat is getting faster and faster as you start thinking of the last time you've been here.
	It is getting dark, the trees, stones and creatures within the forest cast wide shadows.
	And then you recognize the same sparkle in the darkness. Ghorkan, he is there.
	You take a deep breath, take your new sword to hand and walk towards him.

	\"You again\", he murmurs, \"was last time not enough for you?
			This time you won't get out of here alive.\"

	With a great thrust into Ghorkan's heart you achieved to kill the Master of Forest.
	Concurrently, the ban of Ghorkan that was layed upon you is now gone and the forest is once again full of life and the peaceful animals seem to return.
	")
}

static NPC_GHORKAN: Npc = Npc {
    name: "Ghorkan",
    title: "Master of the forest",
};