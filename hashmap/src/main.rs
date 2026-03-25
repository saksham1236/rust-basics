use std::collections::HashMap;

fn main() {
    let mut mission_flown = HashMap::new();
    mission_flown.insert("Hadfield", 3);
    mission_flown.insert("Hurley", 3);
    mission_flown.insert("Barron", 0);
    mission_flown.insert("Barron", 1);
    mission_flown.entry("Barron").or_insert(2);//Checks for entry in hashmap if it doesn't exist then add the value
    mission_flown.entry("Stone").or_insert(2);
    let layla = mission_flown.entry("layla").or_insert(0);
    *layla += 1;
    println!("mission_flown is {:?}", mission_flown);

    let barron_missions = mission_flown.get("Barron");
    println!("barron_mission is {:?}", barron_missions);
}
