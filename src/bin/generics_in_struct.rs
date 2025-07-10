#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}
fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold",
    };
    println!("{:?}", gold_chest);

    let silver_chest = TreasureChest {
        captain: String::from("Bloodsail"),
        treasure: String::from("Silver"),
    };
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{:?}", special_chest);
}