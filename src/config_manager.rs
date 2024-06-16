use std::{fs, io::Write};
///read text used for generation of markov chain
pub fn read_markov_text_file() ->  String {
    if let Ok(text) =  fs::read_to_string("text_gen.txt"){
        text
    }else{
        let file_in_superposition = fs::File::create_new("text_gen.txt");
        match file_in_superposition {
            Ok(mut file) => {
                if let Err(_) = file.write(default_text.as_bytes()){
                    return "error error".to_string()
                }
            },
            Err(_) => {return "error error".to_string()},
        }
        read_markov_text_file() //? possible infinite loop
    }
}

const default_text: &str = "Mordhau is a multiplayer medieval hack and slash game that has captivated players with its intense and immersive gameplay Developed by Triternion and released in 2019 the game has established a dedicated fanbase thanks to its intricate combat system large scale battles and impressive graphics The gameplay in Mordhau revolves around melee combat with a variety of weapons including swords axes and spears Players can also use ranged weapons like bows and crossbows as well as siege weapons such as catapults and ballistae The combat system is designed to be skill based with a focus on timing precision and strategy This makes the learning curve steep but also ensures that skilled players are rewarded for their efforts One of the standout features of Mordhau is its freeform melee combat system which allows players to control the direction and angle of their attacks and blocks This system is highly nuanced and gives players a lot of creative freedom in how they approach combat For example players can perform overhead swings horizontal slashes and thrusts each with varying degrees of effectiveness depending on the situation Blocking and parrying are also critical skills that require precise timing to be effective In addition to its combat system Mordhau offers a variety of game modes that cater to different playstyles and preferences Frontline is one of the most popular modes pitting two teams against each other in large scale battles where players must capture and hold strategic points on the map Another popular mode is Battle Royale where players fight to be the last one standing in a shrinking play area Horde mode allows players to team up and fight waves of increasingly difficult AI controlled enemies Invasion mode is another large scale mode where one team attacks while the other defends Players can also engage in smaller scale duels and skirmishes which offer a more focused and intense combat experience The customization options in Mordhau are extensive allowing players to create unique characters with a wide range of armor and weapon choices Players can mix and match different pieces of armor to create a look that suits their personal style and preferences The game also features a detailed character creator where players can adjust facial features body types and other attributes to create a truly unique warrior Additionally Mordhau includes a progression system where players can earn in game currency and experience points by participating in matches These can be used to unlock new weapons armor and cosmetic items enhancing the sense of achievement and progression The maps in Mordhau are diverse and meticulously designed ranging from open fields and dense forests to castles and fortresses Each map offers different tactical opportunities and challenges requiring players to adapt their strategies accordingly Some maps are designed for large scale battles with multiple capture points while others are more suited for smaller skirmishes and duels The environmental detail and realism in the maps contribute to the overall immersion of the game making each battle feel like a real medieval conflict The community aspect of Mordhau is also significant with players forming clans and groups to compete in organized matches and tournaments The game supports both official and community run servers giving players a variety of options for how they want to play and interact with others Triternion has also been active in supporting the game with regular updates and patches that address balance issues introduce new content and improve the overall gameplay experience This ongoing support has helped maintain a vibrant and engaged player base Despite its many strengths Mordhau is not without its challenges One of the primary criticisms of the game is its steep learning curve which can be daunting for new players The complexity of the combat system means that it can take a significant amount of time and practice to become proficient This can be discouraging for those who are looking for a more casual gaming experience Additionally while the game offers a variety of modes and maps some players feel that there could be more content and variety to keep the gameplay fresh and exciting over the long term Another issue that has been raised by the community is the presence of toxic behavior and harassment in online matches Like many multiplayer games Mordhau has had to deal with instances of griefing and unsportsmanlike conduct which can negatively impact the experience for some players Triternion has implemented measures to address these issues including reporting and banning systems but it remains an ongoing challenge to maintain a positive and inclusive community Overall Mordhau is a game that offers a deeply rewarding and immersive medieval combat experience Its intricate combat system large scale battles and extensive customization options make it a standout title in the genre While it does have its challenges particularly for new players its strengths in gameplay and community support make it a game worth investing time in Whether you are a fan of medieval history and combat or simply looking for a game that offers a high level of skill and strategy Mordhau has something to offer The games graphics and sound design are also noteworthy adding to the overall immersive experience The visual details of the armor weapons and environments are impressive creating a sense of realism that draws players into the medieval world The sound effects from the clash of swords to the thud of arrows hitting their targets further enhance the intensity of the battles The soundtrack is appropriately epic adding to the atmosphere and excitement of the game Mordhau also features modding support allowing the community to create and share custom content This has led to a wealth of user generated content including new maps game modes and cosmetic items The modding community has been very active and creative contributing to the longevity and replayability of the game Triternion has encouraged this aspect of the game providing tools and resources for modders to work with This has helped to keep the game fresh and engaging as players can continually discover and try out new content In terms of technical performance Mordhau runs smoothly on a variety of hardware configurations The developers have done a good job optimizing the game to ensure that it is accessible to players with different system specifications While there are occasional bugs and performance issues Triternion has been responsive in addressing these problems through regular patches and updates This commitment to improving the game has been appreciated by the player community and has contributed to the games positive reception Looking to the future Mordhau has the potential to continue growing and evolving as Triternion and the community work together to develop new content and features The developers have outlined plans for future updates including new maps weapons and game modes as well as improvements to the existing systems This ongoing development is a positive sign that Mordhau will remain a relevant and exciting game for years to come The competitive scene in Mordhau is another aspect that has gained traction over time With its skill based combat and strategic depth the game is well suited for competitive play Various tournaments and leagues have been organized by the community and Triternion itself offering players a platform to showcase their skills and compete for recognition and prizes This competitive aspect adds another layer of engagement to the game appealing to players who enjoy the thrill of high stakes matches and the camaraderie of team play In conclusion Mordhau is a game that stands out in the crowded field of multiplayer action games with its unique blend of skill based combat extensive customization options and large scale battles It offers a rich and immersive medieval experience that can be deeply rewarding for those willing to invest the time to master its complexities While it has its challenges particularly in terms of its learning curve and community management the ongoing support and development from Triternion and the active involvement of the player community bode well for its future Whether you are a seasoned veteran of melee combat games or a newcomer looking for a challenging and engaging experience Mordhau has something to offer and is well worth exploring";