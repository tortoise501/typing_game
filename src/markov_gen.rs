use markov_strings::*;
use std::fs;
pub fn generate(len: usize) -> String {
    let mut markov = Markov::new();

    // Optional: specify a state size
    markov.set_state_size(1).expect("idk error"); // Default: 2

    // Feed it data
    let data: Vec<InputData> = vec![match fs::read_to_string("text_for_markov.txt") {
        Ok(s) => s.to_string(),
        Err(_) => "The Elder Scrolls lore is an expansive and intricate tapestry of myth, magic, and history that stretches across the vast continent of Tamriel and beyond, weaving together a complex narrative that spans millennia and encompasses multiple planes of existence within the Aurbis, the cosmological framework of this fictional universe. This richly layered lore draws from a myriad of cultural influences and explores profound themes of destiny, power, and the very nature of reality itself. At the heart of this lore are the Elder Scrolls, enigmatic artifacts of immense power said to contain prophetic knowledge of past, present, and future events, revered by scholars and feared by those who seek to harness their power for their own ends. The history of Tamriel is a tapestry woven with threads of conflict and upheaval, shaped by the rise and fall of empires, the scheming of powerful sorcerers and daedric princes, and the heroic struggles of mortal champions caught in the currents of prophecy. From the legendary battles of the First Era to the political intrigues of the Third Era and beyond, every corner of the Elder Scrolls universe is steeped in lore and legend, with each installment in the series adding new layers of depth and complexity to this ever-expanding narrative. Whether exploring the frozen wastelands of Skyrim, navigating the labyrinthine streets of the Imperial City in Cyrodiil, or delving into the mystical realm of Morrowind, players are immersed in a world teeming with lore and history, where every artifact, every NPC, and every corner of the map holds the promise of discovery and adventure. With its rich lore, immersive world-building, and captivating storytelling, The Elder Scrolls universe stands as a testament to the enduring power of myth and imagination, inviting players to embark on an epic journey through a realm where the boundaries between reality and fantasy blur and anything is possible.
        The Witcher is an expansive and immersive fantasy universe that encompasses a rich tapestry of lore, mythology, and storytelling, spanning across novels, video games, and a highly acclaimed television series. Created by Polish author Andrzej Sapkowski, The Witcher universe is set in a world known as the Continent, a vast and diverse land inhabited by humans, elves, dwarves, and a myriad of other fantastical creatures. Central to this world is the figure of the Witcher himself, Geralt of Rivia, a genetically enhanced monster hunter imbued with superhuman abilities through a rigorous training regimen and alchemical mutations. Geralt roams the Continent, taking on contracts to slay dangerous beasts while navigating the complex political intrigues and moral dilemmas that define the world around him. The Witcher universe is characterized by its morally gray characters, intricate political machinations, and deep exploration of themes such as destiny, identity, and the nature of good and evil. From the bustling streets of Novigrad to the war-torn landscapes of Velen, players are transported to a world brimming with danger, intrigue, and adventure, where every decision has far-reaching consequences and every character has a story to tell. Through its captivating storytelling, richly detailed world-building, and compelling characters, The Witcher universe has captured the hearts and imaginations of fans around the globe, cementing its place as one of the most beloved and iconic fantasy franchises of all time.
        The Mass Effect universe is a sprawling and intricate science fiction setting that spans across multiple mediums, including video games, novels, and comic books. Created by BioWare, the Mass Effect series is set in the distant future, where humanity has ventured out into the galaxy and encountered a vast array of alien species, each with their own unique cultures, histories, and technologies. Central to the lore of Mass Effect is the concept of mass effect fields, which allow for faster-than-light travel and the manipulation of gravity, shaping the very fabric of galactic society. Players assume the role of Commander Shepard, a customizable protagonist who leads a diverse team of characters on a perilous journey to save the galaxy from ancient, malevolent forces known as the Reapers. Along the way, players navigate complex moral dilemmas, forge alliances with various alien species, and uncover the deep mysteries of the universe. The Mass Effect universe is renowned for its richly detailed world-building, which encompasses everything from the intricate politics of the galactic government to the diverse ecosystems of alien worlds. From the bustling metropolis of the Citadel to the harsh landscapes of Tuchanka, each location is meticulously crafted to immerse players in a vibrant and immersive sci-fi setting. Through its compelling storytelling, memorable characters, and thought-provoking themes, the Mass Effect universe has captivated audiences around the world, leaving an indelible mark on the landscape of science fiction entertainment.".to_string(),
    }]
    .iter()
    .map(|s| s.to_owned().into())
    .collect();
    markov.add_to_corpus(data);

    let mut res = String::new();

    while res.as_str().split(' ').count() < len {
        res.push_str(markov.generate().expect("err").text.as_str());
    }
    res.as_str()
        .split(' ')
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()[..len]
        .join(" ")
}
