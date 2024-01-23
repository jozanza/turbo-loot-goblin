use super::*;

#[derive(Debug, Clone)]
pub struct EventLocationData {
    pub name: &'static str,
    pub images: &'static [&'static str],
    pub dialog: &'static [&'static str],
    pub description: &'static str,
    pub scenarios: &'static [EventScenario],
}
impl EventLocationData {
    pub fn get(i: usize) -> Self {
        ALL_EVENT_LOCATION_DATA[i % ALL_EVENT_LOCATION_DATA.len()].clone()
    }
}

#[derive(Debug, Clone)]
pub struct EventScenario {
    pub name: &'static str,
    pub description: &'static str,
    pub actions: &'static [EventScenarioAction],
}

#[derive(Debug, Clone)]
pub struct EventScenarioAction {
    pub dialog: &'static [&'static str],
    pub label: &'static str,
    pub outcomes: &'static [EventScenarioOutcome],
}

#[derive(Debug, Clone)]
pub struct EventScenarioOutcome {
    pub weight: u32,
    pub description: &'static str,
    pub dialog: &'static [&'static str],
    pub effect: EventResult,
}

pub const GOBLIN_RECRUITMENT_LETTER: &'static str = r#"Dear Esteemed Goblin,
We, the adventurers of the Gallant Guild, seek your unique expertise for an upcoming quest. Your renowned skills in navigating treacherous paths and handling precious artifacts are the talk of the realm, and we believe you would be the perfect addition to our expedition.
Our journey promises to be perilous, but with great risk comes great reward. We require someone of your particular talents to assist in carrying and safeguarding the treasures we aim to retrieve. While your primary role will be that of a bearer, your cunning and quick wits will undoubtedly prove invaluable in the challenges that lie ahead.
We offer fair compensation for your services, along with the opportunity to explore legendary locales. Should our venture prove successful, additional bonuses shall be considered.
Please rendezvous with our party at the dawn of the next full moon by the ancient oak at the crossroads to embark on this grand adventure with you at our side."#;

pub const UNREACHABLE_DIALOG: &'static str = "...";

pub const UNIMPLEMENTED_DIALOG: &[&'static str] =
    &["Heh, Jozanza's still working on that option. Let's try something else..."];

pub const ENTERING_CAMP_DIALOG: &[&'static str] = &[
    "Looks like someone set up camp. Snatchin' time!",
    "Shiny tents and snorin' snores... This is goblin paradise!",
    "Hope they got some good grub left over. Goblins gotta eat too!",
    "Gotta be sneaky as a shadow. One wrong move and they'll have me for target practice.",
    "Maybe I can distract them with some funny noises... or a well-placed stink bomb!",
    "Let's see if their pouches are as fat as their bellies.",
    "Treasure or trouble? Only one way to find out...",
    "Keep it cool, Snitch. Remember, you're here for a mission, not a snack break.",
    "Oooh, fancy weapons! Maybe I can trade some shiny rocks for that rusty sword.",
    "Wonder if they'd miss one little trinket...? Don't worry, it'll find a good home with me!",
    "Time to channel me inner ninja. Stealth mode activated!",
    "Let's hope their guard dog isn't as hungry as I am.",
    "Big camp, big loot, big risk. But a big reward for a clever goblin, eh?",
    "Think they'll notice if I borrow a few coins for me next game of goblin dice?",
    "Shhh, shhh, quiet feet and sneaky thoughts. They'll never know I was here.",
    "I bet their stories are full of juicy gossip and secret treasure maps.",
    "Gotta be careful not to wake the grumpy lookin' one with the axe.",
    "Maybe they left their loot unguarded? A goblin can dream, can't he?",
    "Let's see if I can find somethin' sparkly without causin' too much mayhem.",
    "Time to put my goblin intuition to the test. Can I sniff out the best loot in this camp?",
    "Don't get greedy, Snitch. Just enough loot to buy that new shiny helmet, and then outta here!",
    "Remember, patience is a goblin's secret weapon. Wait for the right moment, then strike like a shadow!",
    "If I get caught, just play dumb and blame it on the squirrels. They always get away with everything!",
    "This camp smells like adventure and campfire stew. My mouth is waterin' already!",
    "Wonder if they have any goblin games. I'd crush 'em all with my dice rolls!",
    "Just a friendly goblin visitor, here to admire the scenery and, uh, borrow some supplies.",
    "Maybe I can make friends with one of 'em. Goblins gotta stick together, right?",
    "Ooh, a camp fire! Perfect for roastin' some tasty mushrooms... or maybe a goblin's backside.",
    "They won't know what hit 'em! This goblin's on a mission, and nothin' can stop me!",
    "Time to put my thieving skills to the test. Let's see how rich these adventurers really are!",
    "Just gotta grab a few shinies and then it's outta here before the snoring stops.",
    "This is gonna be the greatest goblin heist ever! They'll be singin' songs about me in all the caves!",
];

pub const LOOT_RUMMAGE_DIALOG: &[&'static str] = &[
    "Time for a little 'unofficial inventory check'. Let's see what goodies won't be missed, shall we?",
    "Alright, pouches and packs, prepare to meet your new goblin owner!",
    "Sacks of loot, where ya hidin'? Let Uncle Snitch tickle your treasure strings!",
    "Time to unleash my inner magpie and gather all the shiny things!",
    "Shh, quiet fingers, let's see what secrets these pockets hold.",
    "Let's play a little game of loot hide-and-seek. I bet I win every time!",
    "Time to channel my inner dragon and hoard all the shinies for myself!",
    "Ooh, fancy trinkets and dusty coins, my goblin heart is singin'!",
    "Just a quick peek, just a little borrow... nobody will notice, right?",
    "Let's see if any of these pouches have a taste for goblin fingers.",
    "Gotta be careful, traps might be lurkin' in these packs. But a goblin gotta take risks!",
    "Wonder if they got any goblin-sized loot in here? A miniature sword, maybe?",
    "Remember, Snitch, gotta be selective. Only the best shinies are worthy of a goblin's pockets.",
    "Let's see if my nose can sniff out the juiciest treasure like a truffle-huntin' goblin!",
    "Gotta be quick and quiet, like a sneaky shadow. No alarms, no adventurers, just loot!",
    "Time to unleash my goblin charm and convince these pouches to open up for me.",
    "Maybe I should leave a little somethin' in return... like a muddy footprint or a goblin sneeze.",
    "Don't overstuff, Snitch! Remember, you gotta make a quick getaway when the snores stop.",
    "Let's hope they got some tasty snacks in here besides just boring gold coins.",
    "Gotta check for magical curses and booby traps before claimin' any loot. Safety first, even for goblins!",
    "Maybe I should leave a thank-you note? \"To the kind adventurers who gifted me these shinies\"!",
    "Shh, shh, pouches, don't make a sound. We wouldn't want to wake the grumpy ones with axes.",
    "Time to unleash my inner treasure goblin! May the shiniest loot find its way to my sticky fingers!",
    "One pouch at a time, Snitch. Don't get greedy, or you'll end up swimmin' in coins and regrets.",
    "Let's hope these adventurers have good taste in loot. No rusty daggers or moldy cheese for me!",
    "Remember, stealth is key! Leave no goblin fingerprints, no sparkly trails, just empty pouches.",
    "This is like Christmas come early! Except with more risk and less singing elves.",
    "Maybe I should share some of this loot with the other goblins back home. Nah, just kidding!",
    "Let's hope my luck holds out. One wrong move and it's goblin kabobs for breakfast!",
    "Remember, Snitch, you're on a mission, not a goblin shopping spree. Focus on the shiny essentials!",
    "Every coin, every trinket, whispers a story. Let's see what these treasures have to tell me.",
    "Gotta be careful not to wake the guard dog. One bark and it's all over for this little goblin.",
    "Time to put my years of practice at \"borrowin' things\" to good use!",
];

pub const LOOT_RUMMAGE_FAIL_ACCEPT_DIALOG: &[&'static str] = &[
    "J-Just makin' sure everything's accounted for, I swear!",
    "Hey, I was just returnin' this loot after borrowin' it for, uh, safekeepin'.",
    "What loot? I only see shiny souvenirs for me treasure collection!",
    "I'm just a lost little goblin lookin' for me way out! These shiny things keep distractin' me.",
    "Snatchin' loot? No way! I was simply offerin' a free cleanin' service for these dusty pouches.",
    "Those coins were singin' to me! I couldn't resist their sweet, jinglin' melody.",
    "Wait, are these not goblin taxes? I thought we had a new agreement with the adventurer guild!",
    "Borrowing, barterin', what's the difference? Goblins gotta live too, ya know?",
    "I'm just an innocent bystander, framed by these fancy adventurers for their dirty work!",
    "Oops, butterfingers strikes again! These clumsy goblin digits can't hold onto nothin'.",
    "Squirrel! Did you see a squirrel? Must've been it runnin' off with that bag!",
    "I was just performin' an advanced goblin trust exercise. See, you trust me now, right?",
    "These pockets? Nah, they're magic portals to the goblin dimension. Just sendin' some loot home.",
    "Stealth training exercise! Gotta practice my sneaky skills, even if it involves shiny distractions.",
    "Fine, you caught me! But hey, look on the bright side, at least you have some entertainment for the night.",
    "I'm actually a secret agent here to test your security. And you failed! Abysmally!",
    "Don't worry, I'm just borrowin' this loot for your own good. Trust me, you'll thank me later.",
    "Who says stealin'? I'm just returnin' these cursed artifacts to their rightful owners... in my pocket.",
    "Look, if you let me keep this sparkly thing, I'll tell you a secret about trolls. They like belly rubs, it's true!",
    "Maybe these items just wanted to come with me? Goblins have a natural magnetic pull for shinies.",
    "I'm not stealin', I'm just... redistributin' wealth! Spreading the loot more evenly, goblin style.",
    "Don't be mad, be impressed! You gotta admit, my sneaky skills are top-notch. Almost too good, even.",
    "Hey, at least I made you look! Now you can check all your stuff and feel extra secure. You're welcome!",
    "Okay, okay, you got me. But hey, consider it an early tip for savin' your life from that trap over there!",
    "Fine, take your loot back. But remember, a goblin never forgets... and I might borrow it again someday!",
    "This is outrageous! I demand a trial by combat! One rusty dagger versus your fancy swords. Winner takes the loot!",
    "Don't judge a goblin by his pockets! Maybe I'm just a philanthropist, anonymously donating treasure to the needy.",
    "These hands? They were made for holdin' shiny things! It's just a natural goblin instinct, I can't help it.",
    "Alright, you win. But just for today. Tomorrow, this camp will be known as Fort Goblinshield! Mark my words!",
    "Fine, take your loot. But remember, you just interrupted the greatest goblin heist of the century! History will never forgive you!",
    "Don't worry, I'll only be borrowin' this for, uh, inspiration for my next goblin masterpiece. A sculpture made entirely of loot, wouldn't that be somethin'?",
    "You may have won this battle, but the war for loot is far from over! Mark my words, adventurer, I'll be back... with reinforcements!"
];

pub const LOOT_RUMMAGE_TAKE_LOOT_DIALOG: &[&'static str] = &[
    "Heh, what have we here? A few sparklies for me pockets! No one's the wiser, right?",
    "Score! Goblin pockets are officially happy!",
    "Look at all this loot! I'm richer than a dragon with a gold mine!",
    "Hehe, these adventurers won't even miss this stuff. They have too much anyway!",
    "Time to retire and live the good life! Sun, snacks, and endless shinies!",
    "Maybe I'll open a goblin boutique with all this treasure. \"Snitch's Shiny Shack: Come get your bling!\"",
    "Shhh, keep it quiet, but I think I found a magical trinket! Time to experiment... carefully.",
    "This loot will buy me the biggest, shiniest helmet a goblin ever wore! Watch out, world, here comes Gobzilla!",
    "Just enough loot for a feast fit for a king... a goblin king, that is!",
    "I could buy a whole cave with this much gold! Or maybe just a bigger pouch...",
    "I bet all the other goblins will be green with envy when they see my haul!",
    "This is what I get for bein' sneaky and smart! Goblins rule!",
    "Maybe I should share a little... Nah, just kidding! This loot is all mine!",
    "Let's see what secrets these magical doodads hold. Goblin tech time!",
    "Time to vanish like a puff of smoke and leave these adventurers none the wiser!",
    "I'm the luckiest goblin alive! Or maybe the most skilled... probably both!",
    "This just proves it: goblins are the real masters of treasure!",
    "Now I can finally afford that goblin-sized rocking chair I've always wanted. Snoozin' in style!",
    "I'm gonna write a song about this heist! The Ballad of Snitch the Sly and the Sucker Adventurers!",
    "Maybe I should write a thank-you note... just to mess with their minds later!",
    "This calls for a celebration! Fireworks, goblin jigs, and maybe a bit of accidental fire-breathing...",
    "I'm practically royalty now! Bow down before your glorious leader, Snitch the Magnificent!",
    "Who needs adventurers when you have goblin ingenuity? I am unstoppable!",
    "This loot smells like adventure... but also like stale bread. Maybe I should leave a snack as a thank you.",
    "Never underestimate the power of a clever goblin! Tonight, the world is mine!",
    "I could build a fortress of gold with this much loot! Or maybe just a bigger pile to sleep on...",
    "Time to tell tall tales and impress the other goblins with my daring heist!",
    "This is the greatest day of my goblin life! And it's only just begun!",
    "I bet those adventurers are still snorin', oblivious to their shiny loss. Muhahahaha!",
    "Remember, Snitch, stay cool, stay quiet, and enjoy the sweet, sweet taste of victory... and maybe some goblin cake.",
    "This loot is so good, I might even share a little with the guard dog... as long as he doesn't bark!",
    "I feel lucky, I feel rich, I feel like a legend! Today, the world belongs to goblins... and me!",
];

pub const LOOT_RUMMAGE_LEAVE_LOOT_DIALOG: &[&'static str] =
    &["Maybe not this time... Better not push me luck too far, eh?"];

pub const KEEP_GOING_DIALOG: &[&'static str] = &[
    "Off we go! More shiny trinkets waitin' for me sticky fingers!",
    "Shiny loot, here I come! Time to make these pockets jingle like a goblin chorus!",
    "Adventure calls, and me pockets answer! Let's grab some treasure that'll make even Grobnack jealous.",
    "Snatchin' shinies is what I do! Next stop, riches and maybe a nap on a pile of gold.",
    "Sniffin' out sparklies like a truffle pig! Time to find somethin' worth squealin' about.",
    "Goblins gonna goblin! Time to raid and ravage for somethin' shiny enough to blind a troll!",
    "If I steal enough shinies, I'll feast like a king (or at least sneaky, loot-lovin' goblin).",
    "This goblin's on the prowl for somethin' that sparkles more than a dragon's sneeze.",
    "Shiny fortune awaits, and me fingers are itchin' to snag a souvenir (or two, or ten).",
    "Belly full o' ale. Head full o' mischief. Time to go find somethin' worth hoardin'!",
    "Time to dig up some buried treasure and make the ground cry glitter.",
    "I hear the sound of opportunity tappin' its wee feet! Let's go dance with some loot, goblin style!",
    "Sniff, sniff, I smell somethin' richer than a troll's armpit! Let's follow the scent.",
    "These pockets are about to overflow with shinies brighter than a firefly's rump.",
    "Goblins never say no to an opportunity for treasure. Follow the glint, my brothers!",
    "Off we go, to a land where loot runs like rivers and jewels grow on trees!",
    "I got a nose for gold like a dragon for sheep! It leads the way to a fortune fit for a goblin king!"
];

pub const TAKE_A_BREAK_DIALOG: &[&'static str] = &[
    "Gonna let the moss be my mattress for a bit. Time to find a shady spot and let me bones sigh.",
    "Think I'll take a goblin siesta before round two of shiny snatchin'.",
    "Nap time for the weary! Gotta recharge so I can pilfer mountains of treasure.",
    "Time to count sparkly dreams until me claws are ready for more action.",
    "This goblin needs a breather before the next shiny stampede.",
    "Snoozin' ain't lazy, it's strategic! A well-rested goblin is a loot-magnet.",
    "Diamonds on hold, eyelids gettin' heavy. Time to trade treasure huntin' for a treasure nap.",
    "Adventure can wait, me eyes can't. Catchin' some Zzz's before I dive back into the goblin gold rush.",
    "World ain't goin' anywhere, but this comfy rock sure is. Nap time, fellow treasure goblins!",
    "Snoring symphonies soon, shiny dreams to follow. This goblin's takin' a break from the loot marathon.",
    "My pockets are full, my head is hazy. Time to let the sun lull me into a goblin slumber.",
    "Mushrooms for dinner, nap for dessert. This goblin's got his priorities straight!",
    "Gonna recharge me goblin engine by snoozin' under the sun. Treasure can wait, sleep can't.",
    "Diamonds ain't worth nothin' compared to a good nap. Sweet dreams, shiny dreams, here I come!",
    "Snuggle time for this loot-lovin' gremlin. Sleepin' off the rush of all that treasure snatchin'.",
    "Adventure's a buffet, and naps are the dessert. Gonna savor me meal for now.",
];

#[rustfmt::skip]
pub const DEFAULT_RISKY_ACTION_OUTCOMES: &[EventScenarioOutcome] = &[
    EventScenarioOutcome { effect: EventResult::GetLoot, description: "Your gamble pays off, revealing a hidden cache of riches! Fortune favors the bold!", dialog: &["Look at all this loot! I knew takin' a risk would pay off!"], weight: 5 },
    EventScenarioOutcome { effect: EventResult::GetItem, description: "You uncover a precious trinket overlooked by others. A risky move rewarded!", dialog: &["Ooh, shiny! Hidden just for me! I'm a lucky goblin!"], weight: 8 },
    EventScenarioOutcome { effect: EventResult::StealLoot, description: "The chaos you caused distracts everyone, allowing you to swipe a pouch unseen.", dialog: &["Heh, they're all too busy with the mess I made. More loot for me!"], weight: 7 },
    EventScenarioOutcome { effect: EventResult::StealItem, description: "Amidst the confusion, you deftly pluck a trinket from a distracted hero's belt.", dialog: &["Ooh, shiny! They're too busy tryin' to figure out what happened to notice!"], weight: 4 },
    EventScenarioOutcome { effect: EventResult::Heal, description: "The adrenaline rush of taking a risk activates a healing surge within you.", dialog: &["Risk takin' always gets me blood pumpin'! Feelin' better already!"], weight: 3 },
    EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Fortune favors the bold! Your daring action increases your luck.", dialog: &["I feel luckier already! Risk takin' is the best way to get ahead!"], weight: 2 },
    EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "The potential consequences of your actions make you re-evaluate your priorities.", dialog: &["Maybe loot ain't worth the risk. Maybe there's more to life than shinies? Nah!"], weight: 1 },
    EventScenarioOutcome { effect: EventResult::LoseLoot, description: "Your risky move backfires, and you lose your pouch! A costly lesson learned.", dialog: &["Nooo! Me shinies! Where'd they go? I'll find whoever took 'em!"], weight: 6 },
    EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket falls victim to the chaos you unleashed. It's gone for good.", dialog: &["Me trinket! It's gone! Stupid risk takin'! I'll get revenge!"], weight: 4 },
    EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot vanishes amidst the turmoil! Panic and accusations erupt.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
    EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets disappear! The consequences of your actions are dire.", dialog: &["Me trinkets! They're gone! I'll never forgive myself for this!"], weight: 3 },
    EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, angered by your recklessness, starts a slapfight!", dialog: &["You think you're better than me, takin' all these risks? Fight me!"], weight: 2 },
    EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Your actions trigger a hidden trap! Enemies emerge, ready to fight.", dialog: &["Ambush! They were waitin' for someone to make a move! Get ready to fight!"], weight: 1 },
    EventScenarioOutcome { effect: EventResult::OK, description: "Things settle down, but the tension remains. Your actions have consequences.", dialog: &["That was a close one. Maybe I should be more careful next time..."], weight: 5 },
];

#[rustfmt::skip]
pub const DEFAULT_SAFE_ACTION_OUTCOMES: &[EventScenarioOutcome] = &[
    EventScenarioOutcome { effect: EventResult::GetLoot, description: "Your meticulous search uncovers a hidden cache! Patience and attention to detail are rewarded!", dialog: &["Look what I found by checkin' every nook and cranny! Slow and steady wins the shinies!"], weight: 4 },
    EventScenarioOutcome { effect: EventResult::GetItem, description: "Your keen eye spots a precious trinket overlooked in the shadows. Careful observation pays off!", dialog: &["Ooh, shiny! I'm glad I took the time to look closer. Sometimes the best treasures are hidden in plain sight."], weight: 7 },
    EventScenarioOutcome { effect: EventResult::StealLoot, description: "While others rush ahead, you notice a forgotten pouch and secure it safely. Discretion is its own reward.", dialog: &["Heh, they're all too busy runnin' around. Now I got a little somethin' extra for me troubles."], weight: 5 },
    EventScenarioOutcome { effect: EventResult::StealItem, description: "Amidst the chaos, you carefully retrieve a trinket abandoned in haste. A cautious acquisition!", dialog: &["Ooh, shiny! It's amazing what people leave behind when they're not thinkin' clearly."], weight: 3 },
    EventScenarioOutcome { effect: EventResult::Heal, description: "The relief of avoiding danger allows you to rest and recover. Taking it slow has its benefits!", dialog: &["Phew, that was a close one. Glad I didn't jump into the fray. Feelin' better already!"], weight: 4 },
    EventScenarioOutcome { effect: EventResult::BoostLuck, description: "The spirits favor those who act with wisdom! Your careful choices increase your fortune.", dialog: &["I feel luckier already! Maybe takin' things slow is the best way to get ahead!"], weight: 3 },
    EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "Avoiding needless risks helps you focus on what truly matters.", dialog: &["Maybe loot ain't worth the trouble. There's more to life than shinies."], weight: 2 },
    EventScenarioOutcome { effect: EventResult::LoseLoot, description: "Your caution couldn't prevent a mishap! You misplace your pouch while carefully navigating obstacles.", dialog: &["Nooo! Me shinies! Where'd they go? I musta dropped 'em while I was bein' so careful..."], weight: 5 },
    EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket slips away as you cautiously maneuver through the environment. A regrettable loss.", dialog: &["Me trinket! It's gone! I musta been too focused on avoidin' trouble."], weight: 3 },
    EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "While you prioritize safety, someone else seizes the loot! A reminder that even caution has risks.", dialog: &["The loot's gone! Someone musta snuck in while I was bein' careful. I'll get 'em next time!"], weight: 3 },
    EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your precious trinket vanishes! It seems even cautious actions can have unforeseen consequences.", dialog: &["Me trinkets! They're gone! I thought I was bein' so careful."], weight: 2 },
    EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, frustrated by your hesitation, starts a slapfight! Misunderstandings arise even in cautious moments.", dialog: &["Hey, you think you're better than me, just standin' around? Fight me!"], weight: 3 },
    EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Despite your careful approach, a hidden threat emerges! Danger can lurk even in the shadows.", dialog: &["Ambush! They were waitin' for someone to let their guard down! Get ready to fight!"], weight: 1 },
    EventScenarioOutcome { effect: EventResult::OK, description: "Things remain calm, and you feel a sense of peace. Your cautious choices have paid off.", dialog: &["That went well. Takin' things slow was the right call."], weight: 6 },
];

#[rustfmt::skip]
pub const CAMP_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Adventurers' Camp",
    images: &["camp", "camp"],
    dialog: &[
        "Ain't me fault they left their sack of loot unattended. A goblin must do what a goblin must do!",
        "Hmm, maybe I could trade 'em some shiny rocks for that fancy sword. Goblins are good at barterin', right?",
        "Look at 'em, all snorin' and droolin'. Makes it almost too easy to snatch their stuff.",
        "Ooh, shiny! I spy somethin' sparkly in that tent. Time for a closer look...",
        "Gotta be quiet as a mouse... or maybe a sneaky goblin! Hehehe.",
        "Wish I had some of that tasty stew they're eatin'. Maybe I'll just \"borrow\" a bowlful.",
        "Don't mind me, just a harmless little goblin... admirin' the scenery.",
        "Is it just me, or do adventurers always have the best loot? Must be nice!",
        "Wonder if they'd notice if I snagged a few coins for me collection...?",
        "Patience, patience... wait for the right moment, then grab the shinies and run!",
        "If I had a big sword like that, I'd be the toughest goblin in the caves!",
        "Think they'd fall for a distraction? I could make some funny noises over there...",
        "Ooh, maybe I could swipe that map while they're not lookin'. Goblins love treasure maps!",
        "Should I sneak the loot now, or wait till they're asleep? Decisions, decisions...",
        "I'm so good at sneakin', they'll never even know I was here. Goblin stealth is the best!",
        "Wonder what they're talkin' about... probably how awesome goblins are.",
        "If they catch me, I'll just tell 'em I'm lost. Works every time!",
        "Perfect setup for a quick pinch! They left their loot unguarded, the fools!",
        "Look at all those bags and pouches... must be full of loot! Goblin senses are tinglin'.",
        "Maybe I could make friends with one of 'em. Goblins can be friendly, right?",
        "Ooh, a shiny helmet! I bet it would make me look real important.",
        "If I get enough loot, I could buy me own camp! With snacks and everything!",
        "Just a few more minutes, then I'll be rich! Well, rich for a goblin, anyway.",
        "Adventurers are always so busy, they never notice a goblin under their noses.",
        "I'm startin' to get hungry... maybe I should just eat some of their food instead of lootin'.",
        "This is way more excitin' than sittin' in a cave all day. Goblins need adventure too!",
        "I wonder if adventurers like goblin jokes? I know a real good one about a troll and a bucket...",
        "If I get caught, I'll just run real fast. Goblins are good at runnin'!",
        "Almost there... just gotta reach out and grab that loot... almost...",
        "This is gonna be the best goblin heist ever! They'll be singin' songs about me!",
    ],
    description: "The flickering campfire casts a warm glow, offering a brief respite from the adventurers' relentless journey...",
    scenarios: &[]
};

#[rustfmt::skip]
pub const DEFAULT_SCENARIOS: &[EventScenario] = &[
    EventScenario {
        name: "Default scenario",
        description: "You notice something that may be good or bad. You have two choices.",
        actions: &[
            EventScenarioAction {
                label: "> Risk it",
                dialog: &["Let's risk it!"],
                outcomes: &[
                    EventScenarioOutcome { effect: EventResult::GetLoot, description: "Hidden riches! Enough gold, gems, and artifacts to ditch the heroes and become a king!", dialog: &["Shinies galore! Me found it first, me keeps it!"], weight: 10, },
                ]
            },
            EventScenarioAction {
                label: "> Play it safe",
                dialog: &["Staying safe!"],
                outcomes: &[
                    EventScenarioOutcome { effect: EventResult::GetLoot, description: "Hidden riches! Enough gold, gems, and artifacts to ditch the heroes and become a king!", dialog: &["Shinies galore! Me found it first, me keeps it!"], weight: 10, },
                ]
            }
        ],
    }
];

pub const ALL_EVENT_LOCATION_DATA: &[EventLocationData] = &[
    FORGOTTEN_RUINS_LOCATION_DATA,
    WHISPERING_WOODS_LOCATION_DATA,
    SUNKEN_CITY_LOCATION_DATA,
    DESERTED_VILLAGE_LOCATION_DATA,
    FROZEN_WASTELAND_LOCATION_DATA,
    VOLCANIC_RIDGE_LOCATION_DATA,
    GRAND_HALL_LOCATION_DATA,
    CRYSTAL_CAVERN_LOCATION_DATA,
    MYSTERIOUS_CAVE_LOCATION_DATA,
    LUMINOUS_PASSAGEWAY_LOCATION_DATA,
    HAUNTED_CAVE_LOCATION_DATA,
    OMINOUS_CORRRIDOR_LOCATION_DATA,
    LUSH_CAVERN_LOCATION_DATA,
    THRONE_ROOM_LOCATION_DATA,
    CURSED_VAULT_LOCATION_DATA,
];

#[rustfmt::skip]
pub const FORGOTTEN_RUINS_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Forgotten Ruins",
    images: &["forgotten_ruins", "forgotten_ruins"],
    dialog: &["???"],
    description: "Ancient ruins overgrown with vegetation, hiding secrets of a lost civilization.",
    scenarios: DEFAULT_SCENARIOS,
};

#[rustfmt::skip]
pub const WHISPERING_WOODS_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Whispering Woods",
    images: &["whispering_woods", "whispering_woods"],
    dialog: &["???"],
    description: "A mysterious forest where the trees seem to whisper secrets of the past.",
    scenarios: DEFAULT_SCENARIOS,
};

#[rustfmt::skip]
pub const SUNKEN_CITY_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Sunken City",
    images: &["sunken_city", "sunken_city"],
    dialog: &["???"],
    description: "A city submerged underwater, with remnants of a once thriving civilization.",
    scenarios: DEFAULT_SCENARIOS,
};

#[rustfmt::skip]
pub const DESERTED_VILLAGE_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Deserted Village",
    images: &["deserted_village", "deserted_village"],
    dialog: &["???"],
    description: "An abandoned village with empty houses and untold stories.",
    scenarios: DEFAULT_SCENARIOS,
};

#[rustfmt::skip]
pub const FROZEN_WASTELAND_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Frozen Wasteland",
    images: &["frozen_wasteland", "frozen_wasteland"],
    dialog: &["???"],
    description: "A harsh, icy landscape where survival is a constant challenge.",
    scenarios: DEFAULT_SCENARIOS,
};

#[rustfmt::skip]
pub const VOLCANIC_RIDGE_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Volcanic Ridge",
    images: &["volcanic_ridge", "volcanic_ridge"],
    dialog: &["???"],
    description: "A perilous ridge near an active volcano, with rivers of lava and ash.",
    scenarios: DEFAULT_SCENARIOS,
};

#[rustfmt::skip]
pub const GRAND_HALL_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Grand Hall",
    images: &["grand_hall", "grand_hall"],
    dialog: &["Must've been a great place for a party. Still reeks of stale mead."],
    description: "An elegant arched hall, echoing memories of grand feasts.",
    scenarios: &[
        EventScenario {
            name: "Echoing Voices",
            description: "You hear distant voices echoing. Could be other adventurers... or something worse.",
            actions: &[
                EventScenarioAction {
                    label: "> Investigate Voices",
                    dialog: &["Hey, do you hear that? Let's check it out!"],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "Hidden riches! Enough gold, gems, and artifacts to ditch the heroes and become a king!", dialog: &["Shinies galore! Me found it first, me keeps it!"], weight: 10, },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "Dusty pouch bulging with coins and trinkets! Not enough to retire, but perfect for a goblin feast!", dialog: &["Ooh, this could fetch a pretty penny! Into me pockets it goes!"], weight: 8, },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "Heroes distracted by voices. You nimbly swipe a hefty pouch! Teamwork? Who needs it with goblin cunning?", dialog: &["They're too busy listening. Time to lighten their pockets, one coin at a time!"], weight: 7, },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "Gleaming ring on a hero's finger. A quick flick, and it's yours! Finders keepers, losers weepers!", dialog: &["Ooh, shiny! They won't miss this little trinket, right? Finders keepers!"], weight: 5, },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "Echoes mend your cuts and bruises. Back to business!", dialog: &["That voice makes me feel strong! Back to lootin' we go!"], weight: 4, },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Tingling with good fortune! Maybe today's the day for a dragon's hoard!", dialog: &["Whoa, feelin' lucky! Maybe I'll find a whole dragon hoard next!"], weight: 3, },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "Strange whispers fill you with uncharacteristic camaraderie. Teamwork with these heroes?", dialog: &["Huh, not feelin' so greedy anymore. Weird. Now let's smash things!"], weight: 2, },
                        EventScenarioOutcome { effect: EventResult::OK, description: "Nothing but dust and echoes. Disappointing, but at least safe.", dialog: &["Just a dead end and spooky whispers. Not worth the scare, lads."], weight: 6, },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "Pouch empty! Curses on those deceiving voices!", dialog: &["Nooo! Me shinies! Where'd they go?! Curse those voices!"], weight: 6, },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Favorite trinket gone! Those heroes better not have anything to do with this!", dialog: &["Me favorite trinket! Gone! Those voices are gonna pay for this!"], weight: 5, },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "Loot vanished! Heroes blame you. Time to run!", dialog: &["Those voices were a distraction! The loot's gone! The boss is gonna have our heads!"], weight: 4, },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Trinkets gone! How will you impress the goblin queen now?", dialog: &["The shiny trinkets are gone! How're we gonna fight now? Stupid voices!"], weight: 3, },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin bumps you, accuses you of stealing. Time for a quick slapfight!", dialog: &["You wanna piece of me, bug-face?! Get over here and I'll show ya who's boss!"], weight: 2, },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Ambush from the shadows! Time to fight for your life and your loot!", dialog: &["Get em! We ain't goin' down without a fight!"], weight: 1, },
                    ],
                },                
                EventScenarioAction {
                    label: "> Keep Quiet",
                    dialog: &["Shh... Better not make a sound."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "Silently observing reveals a hidden cache! You snatch it before anyone notices.", dialog: &["Look what I found while you were all yappin'! All mine!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "You spot a valuable trinket overlooked by others and pocket it discreetly.", dialog: &["Ooh, shiny! And nobody saw me take it. Into me pockets it goes!"], weight: 8 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "While others argue, you swipe a pouch from the loot pile! Easy pickings.", dialog: &["Heh, they're too busy squawkin' to notice me lootin'. Thanks, voices!"], weight: 7 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "You deftly snag a ring off a distracted hero's finger. Finders keepers!", dialog: &["Ooh, shiny! They won't miss this little trinket, right? Finders keepers!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "The quiet allows you to focus on patching your wounds. Back to fighting fit!", dialog: &["That silence was just what I needed to feel strong again! Back to lootin' we go!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "The whispers bless you with good fortune! Maybe today's the day for a dragon hoard!", dialog: &["Whoa, feelin' lucky! Maybe I'll find a whole dragon hoard next!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "The voices fill you with a strange sense of contentment. Sharing might be okay.", dialog: &["Huh, not feelin' so greedy anymore. Weird. Now let's just smash some things!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "While lost in thought, someone pilfers your pouch! Curse those voices!", dialog: &["Nooo! Me shinies! Where'd they go?! Curse those voices!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket vanishes! You'll have to find the culprit.", dialog: &["Me favorite trinket! Gone! Those voices are gonna pay for this!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot disappears while everyone's distracted! Blame and chaos erupt.", dialog: &["Those voices were a distraction! The loot's gone! The boss is gonna have our heads!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets vanish! How will you impress the goblin queen now?", dialog: &["The shiny trinkets are gone! How're we gonna fight now? Stupid voices!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin accuses you of stealing. Time for a slapfight to settle it!", dialog: &["You wanna piece of me, bug-face?! Get over here and I'll show ya who's boss!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Enemies emerge from the shadows, sensing your divided attention! Fight back!", dialog: &["Get em! We ain't goin' down without a fight!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "The voices fade, leaving only silence. Nothing gained, but nothing lost.", dialog: &["Phew, that was close. Glad those voices are gone."], weight: 6 },
                    ],
                },
                
            ]
        },
        EventScenario {
            name: "Flickering Shadows",
            description: "Shadows dance along the walls, cast by the flickering torchlight.",
            actions: &[
                EventScenarioAction {
                    label: "> Examine Shadows",
                    dialog: &["Those shadows are tricky... Let's see what they're hiding."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You spot a hidden cache within the shadows! Quick, grab it before they shift!", dialog: &["Look, loot! The shadows hid it well, but I'm too sharp for 'em!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A glimmer catches your eye in the gloom. A valuable trinket, now yours!", dialog: &["Ooh, shiny! The shadows almost fooled me, but I saw it twinklin'!"], weight: 7 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "The shadows provide cover as you swipe a pouch from the loot pile! Sneaky!", dialog: &["Heh, the shadows are me best friends sometimes. Thanks for the loot!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "You deftly pluck a ring from a hero's belt while the shadows dance. Unseen!", dialog: &["Ooh, shiny! The shadows made me do it! They're the real thieves!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "The shadows seem to soothe your wounds, leaving you feeling refreshed.", dialog: &["That darkness felt good! All patched up and ready for more!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "The shadows whisper secrets of fortune! Luck is on your side now.", dialog: &["The shadows told me a secret... today's me lucky day!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "The dancing shadows remind you of life's fleeting nature. Sharing feels right.", dialog: &["The shadows showed me... maybe loot ain't everything. Weird."], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "Darkness covers you for a moment... and your pouch is lighter! Curses!", dialog: &["Nooo! Me shinies! The shadows took 'em! I'll get you back!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket slips into a patch of impenetrable darkness! Lost!", dialog: &["Me trinket! Gone into the shadows! I'll never see it again!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The shadows swirl and the loot vanishes! Panic and accusations fly.", dialog: &["The shadows stole the loot! Or maybe it was you? I'll smash ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets disappear into the gloom! The shadows mock your loss.", dialog: &["The shadows took me trinkets! I'll never impress the queen now!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin bumps you in the darkness. Time for a slapfight!", dialog: &["You wanna piece of me? Can't even see in the dark, can ya? Fight!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Enemies emerge from the shadows, blades drawn! Defend yourself!", dialog: &["Ambush! The shadows hid 'em well!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "The shadows dance and play, but reveal nothing of consequence.", dialog: &["Just shadows bein' shadows. Nothin' to see here."], weight: 6 },
                    ],
                },                
                EventScenarioAction {
                    label: "> Ignore Shadows",
                    dialog: &["Just some shadows. Keep moving, nothing to see here."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You stumble upon a hidden cache while ignoring the distractions! Lucky find!", dialog: &["Wasn't even lookin' for loot, but there it was! Score!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A shiny trinket catches your eye despite your focus elsewhere. Finders keepers!", dialog: &["Ooh, shiny! Almost missed you, but I'm always on the lookout for loot!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "While others fret over shadows, you swipe a pouch unnoticed. Easy pickings!", dialog: &["Heh, the shadows are distractin' everyone else. More for me!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "A hero's trinket slips into your pocket while their gaze is elsewhere. Score!", dialog: &["Ooh, shiny! They're too busy watchin' shadows to notice me lootin'!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "Brushing off the shadows clears your mind, allowing you to bandage your wounds.", dialog: &["No time for spooky stuff! Gotta patch meself up and keep movin'!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Your determination to ignore the shadows attracts a stroke of good fortune!", dialog: &["Who needs shadows when you got luck on your side? Bring on the loot!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "Dismissing the shadows reminds you of the value of teamwork. Sharing is caring?", dialog: &["Maybe loot ain't everything. Maybe friends are the real treasure? Nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "While you're not looking, a sneaky thief pilfers your pouch! Curse them!", dialog: &["Nooo! Me shinies! Where'd they go? I'll find the culprit and smash 'em!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket vanishes! The shadows must have swallowed it.", dialog: &["Me trinket! It's gone! Those shadows are gonna pay for this!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot disappears while everyone's distracted! Blame and chaos erupt.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets vanish! You should have kept an eye on them.", dialog: &["Me trinkets! They're gone! I'll never impress the queen now!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, annoyed by your dismissiveness, starts a slapfight!", dialog: &["You think you're better than me, ignorin' the shadows? Fight me!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Enemies hidden in the shadows ambush you! You should have been more cautious.", dialog: &["Ambush! They got the jump on us!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "The shadows dissipate, leaving you unharmed but with a sense of unease.", dialog: &["The shadows are gone. Good riddance. But I got a bad feelin' about this..."], weight: 6 },                        
                    ],
                },
            ],
        },
        EventScenario {
            name: "Mysterious Statue",
            description: "An ancient statue stands here, its eyes seeming to follow you.",
            actions: &[
                EventScenarioAction {
                    label: "> Inspect Statue",
                    dialog: &["That statue looks important, might hold a secret!"],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You discover a hidden compartment in the statue, filled with riches!", dialog: &["Look what I found in the statue's nose! Shiny!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A gem falls from the statue, unnoticed by others. Quick, pocket it!", dialog: &["Ooh, shiny! The statue dropped a present just for me!"], weight: 8 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "The statue's imposing presence distracts everyone, allowing you to swipe loot.", dialog: &["Heh, everyone's so busy gawkin' at the statue, they didn't see me lootin'!"], weight: 7 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "You deftly pluck a trinket from a hero's belt while they admire the statue.", dialog: &["Ooh, shiny! They're too distracted by the statue to notice me sticky fingers!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "The statue emanates an aura of healing energy, mending your wounds.", dialog: &["That statue's magic feels good! All patched up and ready for more!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "The statue's eyes glow, bestowing good fortune upon those who gaze upon it.", dialog: &["The statue winked at me! I feel luckier already! Time to find some loot!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "The statue's serene expression reminds you of the value of sharing.", dialog: &["Huh, that statue's got a point. Maybe I don't need all the shinies for myself."], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "The statue's eyes flash, and your pouch feels lighter! It's a thief!", dialog: &["Nooo! Me shinies! The statue took 'em! I'll smash it to bits!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket slips from your grasp and shatters against the statue's base!", dialog: &["Me trinket! It's broken! Stupid statue! I'll get revenge!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The statue's gaze mesmerizes everyone as the loot vanishes! Panic ensues.", dialog: &["The statue stole the loot! Or maybe it was you? I'll clobber ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets disappear! The statue's eyes gleam with mischief.", dialog: &["Me trinkets! They're gone! That statue's gonna pay for this!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin bumps into you while admiring the statue. Time for a slapfight!", dialog: &["Watch where you're goin', bug-face! You wanna fight about it?"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "The statue animates and attacks! Its stone fists pack a punch!", dialog: &["The statue's alive! And it's angry!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "The statue remains silent and still, revealing nothing of consequence.", dialog: &["Just a boring old statue. Nothin' to see here."], weight: 6 },                        
                    ],
                },
                EventScenarioAction {
                    label: "> Walk Past",
                    dialog: &["Creepy statue... Let's not stick around."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You stumble upon a hidden cache while walking past! Lucky break!", dialog: &["Wasn't even lookin' for loot, but there it was! Score!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A glint in the shadows catches your eye. A valuable trinket, all yours!", dialog: &["Ooh, shiny! Almost missed you, but I always keep me eyes peeled!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "Others are distracted by the statue, allowing you to swipe a pouch unseen.", dialog: &["Heh, that statue's a great distraction. More loot for me!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "A hero's trinket slips into your pocket while they gaze at the statue. Easy!", dialog: &["Ooh, shiny! They're too busy admirin' the statue to notice me lootin'!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "Focusing on your path clears your mind, allowing you to bandage your wounds.", dialog: &["No time for statues! Gotta patch meself up and keep movin'!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Your determination to ignore the statue attracts a stroke of good fortune!", dialog: &["Who needs statues when you got luck on your side? Bring on the loot!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "Dismissing the statue reminds you of the value of teamwork. Maybe sharing is good?", dialog: &["Maybe loot ain't everything. Maybe friends are the real treasure? Nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "While you're not looking, a sneaky thief pilfers your pouch! Curse them!", dialog: &["Nooo! Me shinies! Where'd they go? I'll find the culprit and smash 'em!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket vanishes! The statue must have cursed you somehow.", dialog: &["Me trinket! It's gone! That statue's gonna pay for this!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot disappears while everyone's focused on the statue! Blame flies.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets vanish! You should have kept a closer eye on them.", dialog: &["Me trinkets! They're gone! I'll never impress the queen now!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, annoyed by your dismissiveness, starts a slapfight!", dialog: &["You think you're better than me, ignorin' the statue? Fight me!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "The statue animates and attacks from behind! You should have paid attention.", dialog: &["The statue's alive! And it's angry!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "You pass the statue without incident, but a sense of unease lingers.", dialog: &["The statue didn't do nothin'. But I got a bad feelin' about this..."], weight: 6 },
                    ],
                },
            ],
        },
        EventScenario {
            name: "Hidden Door",
            description: "A section of the wall seems out of place. Could there be a secret door?",
            actions: &[
                EventScenarioAction {
                    label: "> Search for Door",
                    dialog: &["This wall looks odd. Help me push it!"],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You discover a secret cache behind a loose stone! Hidden treasures!", dialog: &["Found a secret stash while lookin' for the door! Even better!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A glimmer catches your eye in the shadows. A lost trinket, now yours!", dialog: &["Ooh, shiny! Found it tucked behind a rock! Finders keepers!"], weight: 7 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "Searching keeps you out of sight, allowing you to swipe a pouch unnoticed.", dialog: &["Heh, they're all busy lookin' for the door. More loot for me!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "A hero's ring slips off as they push on a wall. You pocket it with a grin.", dialog: &["Ooh, shiny! They're too distracted to notice me sticky fingers!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "You find a healing potion stashed behind a loose brick! A welcome surprise.", dialog: &["Found some healin' juice! All patched up and ready to keep searchin'!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Your persistence in searching activates a hidden luck rune! Fortune smiles.", dialog: &["Feelin' lucky all of a sudden! Maybe I'll find the door and a dragon hoard!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "The quiet contemplation of searching reminds you of the value of sharing.", dialog: &["Maybe loot ain't everything. Maybe helpin' each other is better? Nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "A trapdoor opens beneath you, dropping your pouch into a dark abyss! Curses!", dialog: &["Nooo! Me shinies! Fell into a hole! I'll find a way to get 'em back!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket gets stuck in a crevice and snaps in two! Disaster!", dialog: &["Me trinket! It's broken! This door better be worth it!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "While everyone's distracted, the loot vanishes! Panic and accusations erupt.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets disappear from your pocket! How could you not have noticed?", dialog: &["Me trinkets! They're gone! I'll never impress the queen now!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin bumps into you in the cramped space. Time for a slapfight!", dialog: &["Watch where you're steppin', bug-face! You wanna fight about it?"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Your search triggers a hidden guardian! It's not happy to be disturbed.", dialog: &["Ambush! The door was guarded!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "Despite careful searching, you find no hidden door. But you'll keep trying.", dialog: &["No door here. But I know it's around here somewhere. I'll find it!"], weight: 5 },
                    ],
                },
                EventScenarioAction {
                    label: "> Move On",
                    dialog: &["No time for walls. Let's keep moving."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You stumble upon a hidden cache while exploring a different path! Lucky!", dialog: &["Wasn't even lookin' for loot, but there it was! Score!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A glint in the rubble catches your eye. A valuable trinket, all yours!", dialog: &["Ooh, shiny! Almost missed you, but I always keep me eyes peeled!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "Focusing on progress allows you to swipe a pouch unnoticed. Easy pickings!", dialog: &["Heh, they're all stuck back there lookin' for a door. More loot for me!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "A hero's trinket slips into your pocket as you brush past them. Score!", dialog: &["Ooh, shiny! They're too busy worryin' about doors to notice me lootin'!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "Finding a new path clears your mind, allowing you to bandage your wounds.", dialog: &["No time for hidden doors! Gotta patch meself up and keep movin'!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Your determination to move forward attracts a stroke of good fortune!", dialog: &["Who needs hidden doors when you got luck on your side? Bring on the loot!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "Leaving the door behind reminds you of the value of shared experiences.", dialog: &["Maybe loot ain't everything. Maybe the real treasure is the friends we-- nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "While you're not looking, a sneaky thief pilfers your pouch! Curse them!", dialog: &["Nooo! Me shinies! Where'd they go? I'll find the culprit and smash 'em!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket slips from your grasp and tumbles into a chasm! Lost!", dialog: &["Me trinket! It's gone! That door was bad luck! I knew we shouldn'ta left!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot disappears while everyone's moving forward! Blame and chaos erupt.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets vanish! Maybe you should have stayed to find the door?", dialog: &["Me trinkets! They're gone! I'll never impress the queen now!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, frustrated by the lack of progress, starts a slapfight!", dialog: &["You think you're better than me, just walkin' away? Fight me!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Moving on leads you straight into an ambush! You should have been more cautious.", dialog: &["Ambush! They were waitin' for us to leave!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "You leave the hidden door behind, but a sense of unease lingers.", dialog: &["Maybe there wasn't even a door. But I got a bad feelin' about this..."], weight: 6 },
                    ],
                },
            ],
        },
        EventScenario {
            name: "Old Painting",
            description: "An old painting hangs here, looking out of place and valuable.",
            actions: &[
                EventScenarioAction {
                    label: "> Check Painting",
                    dialog: &["Hmm, this painting could be worth something. Let's take a closer look."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You find a hidden compartment behind the painting, filled with riches!", dialog: &["Look what was behind the paintin'! Shiny!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A gem falls from the painting's frame, unnoticed by others. Finders keepers!", dialog: &["Ooh, shiny! The paintin' dropped a present just for me!"], weight: 8 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "The painting's beauty distracts everyone, allowing you to swipe a pouch.", dialog: &["Heh, they're all gawkin' at the paintin', they didn't see me lootin'!"], weight: 7 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "You deftly pluck a trinket from a hero's belt while they admire the art.", dialog: &["Ooh, shiny! They're too mesmerized by the paintin' to notice me sticky fingers!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "The painting's soothing colors emit a healing aura, mending your wounds.", dialog: &["That paintin's magic feels good! All patched up and ready for more!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "The painting's eyes seem to follow you, bestowing good fortune upon you.", dialog: &["The paintin' winked at me! I feel luckier already! Time to find some loot!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "The painting's depiction of shared joy reminds you of the value of giving.", dialog: &["Huh, that paintin's got a point. Maybe I don't need all the shinies for myself."], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "The painting's colors swirl, and your pouch feels lighter! It's a thief!", dialog: &["Nooo! Me shinies! The paintin' took 'em! I'll smash it to bits!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket slips from your grasp and shatters against the frame!", dialog: &["Me trinket! It's broken! Stupid paintin'! I'll get revenge!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The painting's beauty mesmerizes everyone as the loot vanishes! Panic!", dialog: &["The paintin' stole the loot! Or maybe it was you? I'll clobber ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets disappear! The painting's eyes gleam with mischief.", dialog: &["Me trinkets! They're gone! That paintin's gonna pay for this!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin bumps into you while admiring the art. Time for a slapfight!", dialog: &["Watch where you're goin', bug-face! You wanna fight about it?"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "The figures in the painting leap out to attack! Art can be dangerous!", dialog: &["The paintin's alive! And it's angry!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "The painting remains silent and still, revealing nothing of consequence.", dialog: &["Just a boring old paintin'. Nothin' to see here."], weight: 6 },
                    ],
                },
                EventScenarioAction {
                    label: "> Leave It",
                    dialog: &["Just an old painting. Let's focus on the treasure!"],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You stumble upon a hidden cache while walking away! Lucky find!", dialog: &["Wasn't even lookin' for loot, but there it was! Score!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A glint in the shadows catches your eye. A valuable trinket, all yours!", dialog: &["Ooh, shiny! Almost missed you, but I always keep me eyes peeled!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "Others are distracted by the painting, allowing you to swipe a pouch unseen.", dialog: &["Heh, that paintin's a great distraction. More loot for me!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "A hero's trinket slips into your pocket while they gaze at the art. Easy!", dialog: &["Ooh, shiny! They're too busy admirin' the paintin' to notice me lootin'!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "Focusing on your path clears your mind, allowing you to bandage your wounds.", dialog: &["No time for paintin's! Gotta patch meself up and keep movin'!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Your determination to ignore the painting attracts a stroke of good fortune!", dialog: &["Who needs paintin's when you got luck on your side? Bring on the loot!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "Dismissing the painting reminds you of the value of teamwork. Maybe sharing is good?", dialog: &["Maybe loot ain't everything. Maybe friends are the real treasure? Nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "While you're not looking, a sneaky thief pilfers your pouch! Curse them!", dialog: &["Nooo! Me shinies! Where'd they go? I'll find the culprit and smash 'em!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket vanishes! The painting must have cursed you somehow.", dialog: &["Me trinket! It's gone! That paintin's gonna pay for this!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot disappears while everyone's focused on the painting! Blame flies.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets vanish! You should have kept a closer eye on them.", dialog: &["Me trinkets! They're gone! I'll never impress the queen now!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, annoyed by your dismissiveness, starts a slapfight!", dialog: &["You think you're better than me, ignorin' the paintin'? Fight me!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "The painting's figures animate and attack from behind! You should have looked!", dialog: &["The paintin's alive! And it's angry!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "You pass the painting without incident, but a sense of unease lingers.", dialog: &["The paintin' didn't do nothin'. But I got a bad feelin' about this..."], weight: 6 },
                    ]
                },
            ],
        },
        EventScenario {
            name: "Dusty Rug",
            description: "A large, dusty rug lies on the floor. Something might be hidden beneath.",
            actions: &[
                EventScenarioAction {
                    label: "> Lift the Rug",
                    dialog: &["Rugs always hide secrets. Let's see what's under there!"],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You discover a hidden trapdoor beneath the rug, leading to a treasure trove!", dialog: &["Look what was under the rug! A secret stash! Shiny!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A forgotten trinket glimmers in the rug's fibers. Finders keepers!", dialog: &["Ooh, shiny! Got lost in the rug, but it's mine now!"], weight: 7 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "The rug's commotion distracts everyone, allowing you to swipe a pouch.", dialog: &["Heh, they're all watchin' the rug, they didn't see me lootin'!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "A hero's ring slips off as they help lift the rug. You pocket it swiftly.", dialog: &["Ooh, shiny! They're too busy with the rug to notice me sticky fingers!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "The rug's dust triggers a sneezing fit, but clears your sinuses and heals you!", dialog: &["Achoo! Ugh, dusty rug! But I feel better now. All patched up!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Your curiosity activates a hidden luck rune beneath the rug! Fortune smiles.", dialog: &["Feelin' lucky all of a sudden! Maybe the rug was magic! Bring on the loot!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "The rug's depiction of a humble home reminds you of the value of simplicity.", dialog: &["Maybe loot ain't everything. Maybe a cozy rug and a warm fire are enough? Nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "A hole in the rug swallows your pouch! It's gone, lost to the dust mites!", dialog: &["Nooo! Me shinies! Fell through a hole in the rug! I'll get 'em back!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket gets tangled in the rug's tassels and snaps in two!", dialog: &["Me trinket! It's broken! Stupid rug! I'll get revenge!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot vanishes amidst the rug's swirling dust! Panic and accusations erupt.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets disappear! The rug's patterns seem to mock you.", dialog: &["Me trinkets! They're gone! That rug was cursed! I'll never forgive it!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin trips on the rug and blames you. Time for a slapfight!", dialog: &["Watch where you're puttin' that rug, bug-face! You wanna fight about it?"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Dust mites coalesce into a monstrous Dust Bunny! It's not happy you woke it!", dialog: &["Aaaahh! The dust bunny's alive! And it's angry!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "The rug reveals nothing but dust and dirt. A disappointing anticlimax.", dialog: &["Just a dusty old rug. Nothin' to see here. What a waste of time."], weight: 5 },
                    ],
                },
                EventScenarioAction {
                    label: "> Step Over",
                    dialog: &["Watch your step, but let's not linger here."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You stumble upon a hidden cache while walking away! Lucky find!", dialog: &["Wasn't even lookin' for loot, but there it was! Score!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A glint in the rubble catches your eye. A valuable trinket, all yours!", dialog: &["Ooh, shiny! Almost missed you, but I always keep me eyes peeled!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "Others are preoccupied with the rug, allowing you to swipe a pouch unseen.", dialog: &["Heh, they're all busy with the rug. More loot for me!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "A hero's trinket slips into your pocket as you brush past them. Score!", dialog: &["Ooh, shiny! They're too busy worryin' about rugs to notice me lootin'!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "Leaving the rug behind clears your mind, allowing you to bandage your wounds.", dialog: &["No time for dusty rugs! Gotta patch meself up and keep movin'!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Your determination to move forward attracts a stroke of good fortune!", dialog: &["Who needs rugs when you got luck on your side? Bring on the loot!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "Ignoring the rug reminds you of the value of simple pleasures.", dialog: &["Maybe loot ain't everything. Maybe I should just relax by a fire? Nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "While you're not looking, a sneaky thief pilfers your pouch! Curse them!", dialog: &["Nooo! Me shinies! Where'd they go? I'll find the culprit and smash 'em!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket slips from your grasp and tumbles into a chasm! Lost!", dialog: &["Me trinket! It's gone! That rug was bad luck! I knew we shouldn'ta left it!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot disappears while everyone's moving forward! Blame and chaos erupt.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets vanish! Maybe you should have checked under the rug?", dialog: &["Me trinkets! They're gone! I'll never impress the queen now!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, frustrated by the lack of progress, starts a slapfight!", dialog: &["You think you're better than me, just walkin' away? Fight me!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Stepping over the rug triggers a hidden trap! You should have been cautious.", dialog: &["Ambush! They were waitin' for us to ignore the rug!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "You leave the rug behind, but a sense of unease lingers.", dialog: &["Maybe there wasn't even anything under the rug. But I got a bad feelin'..."], weight: 6 },
                    ],
                },
            ],
        },
        EventScenario {
            name: "Suspicious Chest",
            description: "A chest, slightly ajar, sits against the wall. It looks too easy.",
            actions: &[
                EventScenarioAction {
                    label: "> Open Chest",
                    dialog: &["A chest! Let's see what's inside, but be careful..."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "The chest bursts open, revealing a trove of gold and jewels! Jackpot!", dialog: &["Look at all this loot! I knew this chest was worth the risk!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A hidden compartment holds a gleaming trinket, overlooked by others. Finders keepers!", dialog: &["Ooh, shiny! The chest had a secret just for me! Lucky me!"], weight: 8 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "The chest's contents distract everyone, allowing you to swipe a pouch unseen.", dialog: &["Heh, they're all gawkin' at the chest, they didn't see me lootin'!"], weight: 7 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "You deftly pluck a trinket from a hero's belt while they admire the chest's contents.", dialog: &["Ooh, shiny! They're too mesmerized by the chest to notice me sticky fingers!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "A healing potion nestled within the chest mends your wounds. A welcome surprise!", dialog: &["The chest had a healin' potion! All patched up and ready for more!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "The chest's intricate carvings radiate luck, bestowing fortune upon you.", dialog: &["I feel luckier already! Must be the magic of the chest! Time to find more loot!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "The chest's contents, while valuable, pale in comparison to friendship. Maybe sharing is good?", dialog: &["Huh, maybe loot ain't everything. Maybe the real treasure is the friends we-- nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "A mimic springs from the chest, devouring your pouch! Its hunger is insatiable!", dialog: &["Nooo! Me shinies! The chest ate 'em! I'll smash it to bits!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket falls into a hidden acid trap within the chest! It dissolves instantly!", dialog: &["Me trinket! It's gone! Stupid chest! I'll get revenge!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The chest emits a blinding flash, and the loot vanishes! Panic and accusations erupt.", dialog: &["The chest stole the loot! Or maybe it was you? I'll clobber ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets disappear! The chest's lock clicks shut with a mocking laugh.", dialog: &["Me trinkets! They're gone! That chest is cursed! I'll never forgive it!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, frustrated by the meager contents, starts a slapfight!", dialog: &["This chest is a waste of time! You wanna fight about it?"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "The chest transforms into a monstrous Mimic, teeth bared and ready to devour!", dialog: &["The chest is alive! And it's hungry!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "The chest creaks open to reveal... nothing but dust and cobwebs. How disappointing.", dialog: &["Just an empty chest. Nothin' to see here. What a waste of time."], weight: 5 },
                    ],
                },
                EventScenarioAction {
                    label: "> Ignore Chest",
                    dialog: &["It's too obvious, probably a trap. Let's skip it."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You stumble upon a hidden cache while walking away! Lucky find!", dialog: &["Wasn't even lookin' for loot, but there it was! Score!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A glint in the shadows catches your eye. A valuable trinket, all yours!", dialog: &["Ooh, shiny! Almost missed you, but I always keep me eyes peeled!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "Others are preoccupied with the chest, allowing you to swipe a pouch unseen.", dialog: &["Heh, they're all busy with the chest. More loot for me!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "A hero's trinket slips into your pocket as you brush past them. Score!", dialog: &["Ooh, shiny! They're too busy worryin' about chests to notice me lootin'!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "Leaving the chest behind clears your mind, allowing you to bandage your wounds.", dialog: &["No time for chests! Gotta patch meself up and keep movin'!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Your determination to ignore temptation attracts a stroke of good fortune!", dialog: &["Who needs chests when you got luck on your side? Bring on the loot!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "Dismissing the chest reminds you of the value of caution. Maybe restraint is good?", dialog: &["Maybe loot ain't worth the risk. Maybe it's better to be careful? Nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "While you're not looking, a sneaky thief pilfers your pouch! Curse them!", dialog: &["Nooo! Me shinies! Where'd they go? I'll find the culprit and smash 'em!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket slips from your grasp and tumbles into a chasm! Lost!", dialog: &["Me trinket! It's gone! That chest was bad luck! I knew we shouldn'ta left it!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot disappears while everyone's distracted by the chest! Blame and chaos erupt.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets vanish! Maybe you should have checked the chest after all?", dialog: &["Me trinkets! They're gone! I'll never impress the queen now!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, frustrated by your cautiousness, starts a slapfight!", dialog: &["You think you're better than me, just walkin' away? Fight me!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Ignoring the chest triggers a hidden trap! You should have paid attention.", dialog: &["Ambush! They were waitin' for us to ignore the chest!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "You leave the chest behind, but a sense of unease lingers.", dialog: &["Maybe the chest was harmless. But I got a bad feelin' about this..."], weight: 6 },
                    ],
                },
            ],
        },
        EventScenario {
            name: "Loose Brick",
            description: "One of the bricks in the wall is loose. It might conceal something.",
            actions: &[
                EventScenarioAction {
                    label: "> Remove Brick",
                    dialog: &["Loose bricks are always suspicious. Help me pull it out."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "The brick reveals a hidden cache, filled with forgotten riches! Jackpot!", dialog: &["Look what was behind the brick! Shiny!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A small key tumbles out from behind the brick. It could unlock untold treasures!", dialog: &["Ooh, shiny! A key! Wonder what it opens!"], weight: 7 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "The brick's removal creates a distraction, allowing you to swipe a pouch unseen.", dialog: &["Heh, they're all watchin' the brick, they didn't see me lootin'!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "A hero's ring slips off as they help investigate the brick. You pocket it swiftly.", dialog: &["Ooh, shiny! They're too busy with the brick to notice me sticky fingers!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "A healing potion stashed behind the brick spills onto your wounds, mending them.", dialog: &["The brick had a healin' potion behind it! All patched up and ready for more!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Your curiosity activates a hidden luck rune beneath the brick! Fortune smiles.", dialog: &["Feelin' lucky all of a sudden! Maybe the brick was magic! Bring on the loot!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "The brick's simple placement within the wall reminds you of the value of harmony.", dialog: &["Maybe loot ain't everything. Maybe things are fine just as they are? Nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "A swarm of chipmunks living behind the brick steals your pouch! Cheeky rodents!", dialog: &["Nooo! Me shinies! The chipmunks took 'em! I'll get 'em back!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket gets stuck in the hole and snaps in two! You'll need a new one.", dialog: &["Me trinket! It's broken! Stupid brick! I'll get revenge!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot vanishes as the brick falls, triggering a magical trap! Panic ensues.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets disappear! The brick's hole seems to shimmer with mischievous glee.", dialog: &["Me trinkets! They're gone! That brick was cursed! I'll never forgive it!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, annoyed by your tampering, starts a slapfight!", dialog: &["Why'd you have to mess with the brick? Now we're all in trouble! Fight me!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Removing the brick unleashes a swarm of angry bats! They don't like being disturbed!", dialog: &["Aaaahh! Bats! The brick was keepin' 'em in! Run for your lives!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "The brick comes loose, revealing nothing but dust and mortar. An anticlimax.", dialog: &["Just a regular old brick. Nothin' to see here. What a waste of time."], weight: 5 },
                    ],
                },
                EventScenarioAction {
                    label: "> Leave Brick",
                    dialog: &["It's just a brick. We have bigger fish to fry."],
                    outcomes: &[
                        EventScenarioOutcome { effect: EventResult::GetLoot, description: "You stumble upon a hidden cache while walking away! Lucky find!", dialog: &["Wasn't even lookin' for loot, but there it was! Score!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::GetItem, description: "A glint in the rubble catches your eye. A valuable trinket, all yours!", dialog: &["Ooh, shiny! Almost missed you, but I always keep me eyes peeled!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::StealLoot, description: "Others are preoccupied with the brick, allowing you to swipe a pouch unseen.", dialog: &["Heh, they're all busy with the brick. More loot for me!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::StealItem, description: "A hero's trinket slips into your pocket as you brush past them. Score!", dialog: &["Ooh, shiny! They're too busy worryin' about bricks to notice me lootin'!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::Heal, description: "Leaving the brick behind clears your mind, allowing you to bandage your wounds.", dialog: &["No time for bricks! Gotta patch meself up and keep movin'!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::BoostLuck, description: "Your determination to move forward attracts a stroke of good fortune!", dialog: &["Who needs bricks when you got luck on your side? Bring on the loot!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::ReduceGreed, description: "Ignoring the brick reminds you of the value of stability and structure.", dialog: &["Maybe loot ain't everything. Maybe it's better to leave things be? Nah!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::LoseLoot, description: "While you're not looking, a sneaky thief pilfers your pouch! Curse them!", dialog: &["Nooo! Me shinies! Where'd they go? I'll find the culprit and smash 'em!"], weight: 6 },
                        EventScenarioOutcome { effect: EventResult::LoseItem, description: "Your favorite trinket slips from your grasp and tumbles into a chasm! Lost!", dialog: &["Me trinket! It's gone! That brick was bad luck! I knew we shouldn'ta left it!"], weight: 5 },
                        EventScenarioOutcome { effect: EventResult::LootGotStolen, description: "The loot vanishes while everyone's distracted by the brick! Blame and chaos erupt.", dialog: &["The loot's gone! Who took it? I'll clobber the lot of ya!"], weight: 4 },
                        EventScenarioOutcome { effect: EventResult::ItemGotStolen, description: "Your trinkets vanish! Maybe you should have checked behind the brick?", dialog: &["Me trinkets! They're gone! I'll never impress the queen now!"], weight: 3 },
                        EventScenarioOutcome { effect: EventResult::SlapFight, description: "Another goblin, frustrated by your cautiousness, starts a slapfight!", dialog: &["You think you're better than me, just walkin' away? Fight me!"], weight: 2 },
                        EventScenarioOutcome { effect: EventResult::GetAttacked, description: "Ignoring the brick triggers a hidden trap! You should have paid attention.", dialog: &["Ambush! They were waitin' for us to ignore the brick!"], weight: 1 },
                        EventScenarioOutcome { effect: EventResult::OK, description: "You leave the brick behind, but a sense of unease lingers.", dialog: &["Maybe the brick was harmless. But I got a bad feelin' about this..."], weight: 6 },
                    ],
                },
            ],
        },
    ],
};

#[rustfmt::skip]
pub const CRYSTAL_CAVERN_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Crystal Cavern",
    images: &["crystal_cavern", "crystal_cavern"],
    dialog: &["Hmmm...Looks like there's something shiny at the end of this cavern"],
    description: "A cavern aglow with natural light, showcasing its vibrant beauty.",
    scenarios: &[
        EventScenario {
            name: "Glowing Crystals",
            description: "Crystals emitting a soft glow cover the walls.",
            actions: &[
                EventScenarioAction {
                    label: "> Collect Crystals",
                    dialog: &["Ooh, shiny! Let's grab some of these crystals."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Admire Glow",
                    dialog: &["Wow, these crystals are mesmerizing. Just look at them glow!"],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Echoing Drips",
            description: "Water drips rhythmically from the ceiling.",
            actions: &[
                EventScenarioAction {
                    label: "> Follow Sound",
                    dialog: &["That dripping sound... there's something about it. Let's find out where it's coming from."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Ignore Drips",
                    dialog: &["Just some water dripping. No need to get distracted."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Strange Fossils",
            description: "Fossilized remains are embedded in the cavern walls.",
            actions: &[
                EventScenarioAction {
                    label: "> Examine Fossils",
                    dialog: &["These fossils look ancient! Let's take a closer look."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Walk Away",
                    dialog: &["Just some old bones in the wall. Let's keep moving."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Narrow Crevice",
            description: "A narrow crevice cuts through the cavern floor.",
            actions: &[
                EventScenarioAction {
                    label: "> Explore Crevice",
                    dialog: &["That crevice might lead somewhere. Let's check it out."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Step Around",
                    dialog: &["Careful around that crevice. We don't want to fall in."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Colorful Moss",
            description: "Vibrant moss grows in patches on the ground.",
            actions: &[
                EventScenarioAction {
                    label: "> Touch Moss",
                    dialog: &["This moss is so soft and colorful. I've got to touch it."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Steer Clear",
                    dialog: &["Best not to touch anything. You never know what's lurking in moss like that."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Bubbling Pool",
            description: "A small pool of water bubbles mysteriously.",
            actions: &[
                EventScenarioAction {
                    label: "> Investigate Pool",
                    dialog: &["A bubbling pool? There could be something interesting in there!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Keep Distance",
                    dialog: &["I don't trust that pool. Let's keep our distance."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Abandoned Camp",
            description: "An old campsite lies abandoned, with gear strewn about.",
            actions: &[
                EventScenarioAction {
                    label: "> Search Camp",
                    dialog: &["An abandoned camp? There might be something useful left behind."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Respect Privacy",
                    dialog: &["It's someone else's camp. We shouldn't mess with it."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Crystal Formation",
            description: "An impressive formation of crystals dominates the area.",
            actions: &[
                EventScenarioAction {
                    label: "> Study Formation",
                    dialog: &["These crystals are incredible. Let's take a moment to study them."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Move On",
                    dialog: &["Impressive, but we have more important things to do than stare at crystals."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        
    ],
};

#[rustfmt::skip]
pub const MYSTERIOUS_CAVE_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Mysterious Cave",
    images: &["mysterious_cave", "cave_1"],
    dialog: &["Darker than a dungeon down here, ain't it?"],
    description: "A cave offering both danger and discovery in its silent depths. The cave's mysteries are both alluring and foreboding.",
    scenarios: &[
        EventScenario {
            name: "Stalactite Shadows",
            description: "Shadows cast by stalactites create eerie shapes.",
            actions: &[
                EventScenarioAction {
                    label: "> Investigate Shadows",
                    dialog: &["Look at those shadows... creepy! Should we check them out?"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Move Past",
                    dialog: &["Just shadows, nothing more. Let's keep moving."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Faint Glow",
            description: "A faint glow emits from deeper within the cave.",
            actions: &[
                EventScenarioAction {
                    label: "> Approach Glow",
                    dialog: &["There's a light up ahead. Could be something good... or bad."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Stay Back",
                    dialog: &["Not sure I trust that light. Let's stay here."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Rumbling Sound",
            description: "A deep rumbling sound resonates through the cave.",
            actions: &[
                EventScenarioAction {
                    label: "> Seek Source",
                    dialog: &["That rumbling... could be treasure. Or trouble. Let's see!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Avoid Noise",
                    dialog: &["Rumbling sounds mean trouble. Best avoid it."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Slippery Stones",
            description: "The path is lined with slippery stones.",
            actions: &[
                EventScenarioAction {
                    label: "> Tread Carefully",
                    dialog: &["Watch your step on these stones, don't want to slip now."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Find Alternate Path",
                    dialog: &["Too risky. Let's find another way around."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Ancient Inscriptions",
            description: "Ancient inscriptions are carved into the cave walls.",
            actions: &[
                EventScenarioAction {
                    label: "> Read Inscriptions",
                    dialog: &["Old writings on the wall. Could be a clue or a curse."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Ignore Carvings",
                    dialog: &["Don't have time for old scribbles. Let's move on."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Cold Draft",
            description: "A cold draft flows through a small opening.",
            actions: &[
                EventScenarioAction {
                    label: "> Explore Opening",
                    dialog: &["Brrr, chilly! Maybe that draft leads somewhere interesting."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Stay in Main Cave",
                    dialog: &["Drafts usually mean exits, but let's not risk getting lost."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Strange Echoes",
            description: "Strange echoes bounce off the cave walls.",
            actions: &[
                EventScenarioAction {
                    label: "> Find Source",
                    dialog: &["Echoes can be deceiving. But I'm curious... Let's find out!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Keep Quiet",
                    dialog: &["Echoes in caves? No thanks, too spooky for me."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Hidden Nook",
            description: "A hidden nook seems to hold something valuable.",
            actions: &[
                EventScenarioAction {
                    label: "> Check Nook",
                    dialog: &["A secret spot! Could be loot, could be traps... Exciting!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Respect Boundary",
                    dialog: &["Let's not poke around every nook and cranny. Too risky."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },        
    ],
};

#[rustfmt::skip]
pub const LUMINOUS_PASSAGEWAY_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Luminous Passageway",
    images: &["luminous_passageway", "luminous_passageway"],
    dialog: &["A pleasant passageway. Surely, it leads to fortune."],
    description: "A room that holds the elusive promise of an exit, and perhaps more. The promise of escape is just within reach, but there is yet more shinies to collect.",
    scenarios: &[
        EventScenario {
            name: "Light Breeze",
            description: "A light breeze hints at a passage nearby.",
            actions: &[
                EventScenarioAction {
                    label: "> Seek Passage",
                    dialog: &["Feels like a draft. Could be a way out, or maybe a secret. Let's go check!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Ignore Breeze",
                    dialog: &["Just a breeze. Keep your eyes on the prize, not the wind."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Mysterious Rune",
            description: "A rune glows faintly near the exit.",
            actions: &[
                EventScenarioAction {
                    label: "> Examine Rune",
                    dialog: &["Look at that rune. Glowing and all. Might be worth a peek!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Exit Quickly",
                    dialog: &["Glowy things near exits make me nervous. Let's just get outta here."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Squeaking Bats",
            description: "Squeaking bats hang from the ceiling.",
            actions: &[
                EventScenarioAction {
                    label: "> Observe Bats",
                    dialog: &["Bats are good luck, right? Let's take a quick look."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Sneak Past",
                    dialog: &["Bats mean trouble. Let's sneak by quietly."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Crumbling Wall",
            description: "Part of the wall near the exit is crumbling.",
            actions: &[
                EventScenarioAction {
                    label: "> Inspect Wall",
                    dialog: &["That wall's falling apart. Maybe there's something behind it."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Leave Be",
                    dialog: &["Crumbling walls are bad news. Best leave it alone."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Reflecting Pool",
            description: "A pool reflects the light near the exit.",
            actions: &[
                EventScenarioAction {
                    label: "> Gaze in Pool",
                    dialog: &["What's that in the water? Could be something shiny!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Pass by Pool",
                    dialog: &["Don't get distracted by a pool. We've got other things to find."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Whispering Winds",
            description: "Winds whisper secrets near the exit.",
            actions: &[
                EventScenarioAction {
                    label: "> Listen Closely",
                    dialog: &["Whispers in the wind... Could be a clue, or a warning."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Focus on Exit",
                    dialog: &["Ignore the whispers. We're almost out, keep going."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Overgrown Path",
            description: "You notice a hidden path concealed by overgrown vegetation.",
            actions: &[
                EventScenarioAction {
                    label: "> Clear Path",
                    dialog: &["Let's clear this path. There might be something good along the way."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Take Alternate Route",
                    dialog: &["Overgrown paths are too much trouble. Let's find another way."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Forsaken Artifact",
            description: "An artifact lies forgotten near the exit.",
            actions: &[
                EventScenarioAction {
                    label: "> Retrieve Artifact",
                    dialog: &["That artifact looks valuable. Let's grab it before we go."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Respect Artifact",
                    dialog: &["Best not to mess with forgotten things. Let's leave it be."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },        
    ],
};

#[rustfmt::skip]
pub const HAUNTED_CAVE_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Haunted Cave",
    images: &["haunted_cave", "haunted_cave"],
    dialog: &["Can barely see me own toes in here. If I'm bein' honest, maybe it's for the best..."],
    description: "A cave shrouded in darkness, where unseen threats lurk. Every shadow in this cave seems to hold a secret or a danger.",
    scenarios: &[
        EventScenario {
            name: "Whispering Shadows",
            description: "Shadows seem to whisper secrets in the dark cave.",
            actions: &[
                EventScenarioAction {
                    label: "> Listen to Shadows",
                    dialog: &["Those whispers could be a clue... or a trap. Let's find out!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Ignore Whispers",
                    dialog: &["Ignore those whispers. Shadows can't be trusted."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Glinting Eyes",
            description: "Pairs of glinting eyes watch from the darkness.",
            actions: &[
                EventScenarioAction {
                    label: "> Confront Eyes",
                    dialog: &["Glinting eyes in the dark? Time to face whatever's hiding there!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Avoid Gaze",
                    dialog: &["Don't look at 'em! Nothing with eyes like that means well."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Muffled Cries",
            description: "Muffled cries echo faintly in the cave.",
            actions: &[
                EventScenarioAction {
                    label: "> Seek the Source",
                    dialog: &["Someone or something's making those cries. Let's go see."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Stay Silent",
                    dialog: &["Cries in a dark cave? Nope. Let's not mess with that."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Ominous Symbols",
            description: "Ominous symbols are drawn on the cave walls.",
            actions: &[
                EventScenarioAction {
                    label: "> Decipher Symbols",
                    dialog: &["These symbols look old and important. Maybe they mean something."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Disregard Drawings",
                    dialog: &["Just some creepy cave art. Let's keep our eyes ahead."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Slippery Edge",
            description: "A narrow path with a slippery edge winds through the cave.",
            actions: &[
                EventScenarioAction {
                    label: "> Walk Edge",
                    dialog: &["Careful on that edge. We don't want to slip into the unknown."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Find a Safer Path",
                    dialog: &["That path's too risky. Let's find a safer way around."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Faded Footprints",
            description: "Faded footprints lead deeper into the darkness.",
            actions: &[
                EventScenarioAction {
                    label: "> Follow Footprints",
                    dialog: &["Footprints always lead somewhere. Let's see where they go."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Ignore Footprints",
                    dialog: &["Could be anything's footprints. Best not to follow."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Chilly Gust",
            description: "A chilly gust blows through a hidden passage.",
            actions: &[
                EventScenarioAction {
                    label: "> Explore Passage",
                    dialog: &["A hidden passage? Could be a shortcut or treasure."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Stay in Main Cave",
                    dialog: &["Let's stick to the main path. Hidden passages can be tricky."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Lost Relic",
            description: "A relic seems lost in the cave's darkness.",
            actions: &[
                EventScenarioAction {
                    label: "> Retrieve Relic",
                    dialog: &["A lost relic? This I've got to see for myself!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Leave Relic",
                    dialog: &["Some things are better left alone, especially in a dark cave."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },        
    ],
};

#[rustfmt::skip]
pub const OMINOUS_CORRRIDOR_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Ominous Corridor",
    images: &["ominous_corridor", "ominous_corridor"],
    dialog: &["This corridor gives me the creeps. Should I go anyways?"],
    description: "A corridor that winds its way through history, silent and watchful. The long corridor holds many stories, each shrouded in dust.",
    scenarios: &[
        EventScenario {
            name: "Creeping Vines",
            description: "Vines creep along the walls of the corridor.",
            actions: &[
                EventScenarioAction {
                    label: "> Examine Vines",
                    dialog: &["These vines might be hiding something. Let's have a closer look."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Avoid Vines",
                    dialog: &["Never know what's lurking in those vines. Best to keep our distance."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Rustling Sounds",
            description: "Rustling sounds emerge from the corridor's end.",
            actions: &[
                EventScenarioAction {
                    label: "> Investigate Sound",
                    dialog: &["Rustling sounds ahead. Could be treasure, could be trouble!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Proceed Quietly",
                    dialog: &["Let's not make a fuss. Quietly does it, through the rustling."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Forgotten Tome",
            description: "A forgotten tome lies on a pedestal.",
            actions: &[
                EventScenarioAction {
                    label: "> Read Tome",
                    dialog: &["Old books always have secrets. Let's see what this one says."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Leave Tome",
                    dialog: &["Best not to mess with old tomes. They're often more trouble than they're worth."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Eerie Statues",
            description: "Statues line the corridor, their gazes fixed.",
            actions: &[
                EventScenarioAction {
                    label: "> Study Statues",
                    dialog: &["These statues look ancient. Might tell us something useful."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Ignore Statues",
                    dialog: &["Just some old statues. Let's keep our eyes on the path."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Dusty Chandeliers",
            description: "Dusty chandeliers hang from the ceiling.",
            actions: &[
                EventScenarioAction {
                    label: "> Check Chandeliers",
                    dialog: &["Chandeliers like these might have hidden gems. Worth a look!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Walk Underneath",
                    dialog: &["Just some dusty old lights. Let's move on, no time to waste."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Rattling Chains",
            description: "Chains rattle softly in the distance.",
            actions: &[
                EventScenarioAction {
                    label: "> Follow Chains",
                    dialog: &["That rattling could mean something's up. Let's find out."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Stay Away",
                    dialog: &["Rattling chains in a dark corridor? Nope, not today."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Worn Tapestries",
            description: "Tapestries, worn by time, adorn the walls.",
            actions: &[
                EventScenarioAction {
                    label: "> Inspect Tapestries",
                    dialog: &["Old tapestries tell stories. Maybe there's a clue or two here."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Pass By",
                    dialog: &["Just some old wall hangings. Nothing to see here, let's keep going."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Abandoned Armor",
            description: "A suit of armor stands abandoned.",
            actions: &[
                EventScenarioAction {
                    label: "> Examine Armor",
                    dialog: &["This armor's been left here for a reason. Let's see why."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Leave Armor",
                    dialog: &["Abandoned armor in a creepy corridor? Yeah, that's not suspicious at all..."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },        
    ],
};

#[rustfmt::skip]
pub const LUSH_CAVERN_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Lush Cavern",
    images: &["lush_cavern", "lush_cavern"],
    dialog: &["All these plants... I bet there's treasure hidden here!"],
    description: "A cavern overgrown with lush vegetation, a rare sight underground. Nature thrives in this secluded cavern, untouched by time.",
    scenarios: &[
        EventScenario {
            name: "Blooming Flowers",
            description: "Colorful flowers bloom throughout the cavern.",
            actions: &[
                EventScenarioAction {
                    label: "> Pick Flowers",
                    dialog: &["These flowers are too pretty not to take a few."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Admire Beauty",
                    dialog: &["Never seen anything like this underground. Let's just enjoy it a moment."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Trickling Stream",
            description: "A gentle stream trickles through the cavern.",
            actions: &[
                EventScenarioAction {
                    label: "> Follow Stream",
                    dialog: &["Streams always lead somewhere. Let's see where this one goes."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Stay Dry",
                    dialog: &["Don't fancy getting wet. Let's stay on dry land."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Moss Covered Rocks",
            description: "Rocks covered in soft moss dot the cavern.",
            actions: &[
                EventScenarioAction {
                    label: "> Examine Moss",
                    dialog: &["Moss on rocks? Might be something hidden beneath."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Step Carefully",
                    dialog: &["Careful on the moss. It's slippery, and who knows what's underneath."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Singing Birds",
            description: "Birds sing from hidden spots in the cavern.",
            actions: &[
                EventScenarioAction {
                    label: "> Find Birds",
                    dialog: &["Birds singing in a cave? Gotta see that for myself!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Enjoy Melody",
                    dialog: &["What a tune they're singing. Let's listen for a bit."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Fragrant Herbs",
            description: "The air is fragrant with fresh herbs.",
            actions: &[
                EventScenarioAction {
                    label: "> Gather Herbs",
                    dialog: &["Herbs like these could come in handy. Let's take some."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Breathe Deeply",
                    dialog: &["That smells refreshing! Let's take a deep breath and relax a moment."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Sunlit Clearing",
            description: "A clearing bathed in sunlight appears.",
            actions: &[
                EventScenarioAction {
                    label: "> Enter Clearing",
                    dialog: &["A clearing in a cave? This I've got to see."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Stay in Shade",
                    dialog: &["Sunlit clearings sound nice, but I prefer the shade."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Fluttering Butterflies",
            description: "Butterflies flutter around, adding life to the cavern.",
            actions: &[
                EventScenarioAction {
                    label: "> Catch Butterfly",
                    dialog: &["Butterflies? In a cave? Maybe they're magic. Let's catch one!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Watch in Awe",
                    dialog: &["Never seen butterflies like these. Let's just watch them."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Ancient Tree",
            description: "An ancient tree stands tall in the cavern.",
            actions: &[
                EventScenarioAction {
                    label: "> Climb Tree",
                    dialog: &["A tree in a cave? I gotta climb it. Maybe there's something at the top."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Sit Underneath",
                    dialog: &["Nothing like sitting under a tree to relax a bit. Even underground."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },        
    ],
};

#[rustfmt::skip]
pub const THRONE_ROOM_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Throne Room",
    images: &["throne_room", "throne_room"],
    dialog: &["A throne room! Wonder if there's a crown for me noggin here."],
    description: "Once the heart of a kingdom, the throne room stands silent and imposing. Regal grandeur now faded, the room still echoes with whispers of power.",
    scenarios: &[
        EventScenario {
            name: "Royal Tapestry",
            description: "A large, royal tapestry hangs on the wall.",
            actions: &[
                EventScenarioAction {
                    label: "> Examine Tapestry",
                    dialog: &["This tapestry's got to have a story behind it. Let's see."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Leave Undisturbed",
                    dialog: &["Best not to mess with royal things. You never know what's cursed."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Gilded Crown",
            description: "A gilded crown rests on a cushion.",
            actions: &[
                EventScenarioAction {
                    label: "> Try on Crown",
                    dialog: &["A crown? I've always wanted to try one of these on!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Admire from Afar",
                    dialog: &["That crown's probably booby-trapped. Let's just look at it from here."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Throne Guardian",
            description: "A statue stands guard by the throne.",
            actions: &[
                EventScenarioAction {
                    label: "> Inspect Statue",
                    dialog: &["This statue looks important. Maybe it's hiding something."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Keep Distance",
                    dialog: &["I don't trust statues in places like this. Let's stay away."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Hidden Passage",
            description: "You notice a draft from a hidden passage.",
            actions: &[
                EventScenarioAction {
                    label: "> Explore Passage",
                    dialog: &["A hidden passage? Could be a shortcut to treasure!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Stay in Room",
                    dialog: &["Hidden passages are trouble. Let's stick to the throne room."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Velvet Curtains",
            description: "Velvet curtains obscure part of the room.",
            actions: &[
                EventScenarioAction {
                    label: "> Draw Curtains",
                    dialog: &["Let's see what's behind these curtains. Could be something good!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Respect Privacy",
                    dialog: &["Those curtains are probably there for a reason. Let's not be nosy."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Ornate Chandelier",
            description: "An ornate chandelier casts light below.",
            actions: &[
                EventScenarioAction {
                    label: "> Check Chandelier",
                    dialog: &["Never know what you'll find on a fancy chandelier. Let's take a look."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Ignore Chandelier",
                    dialog: &["It's just a light. Let's keep our eyes on the ground, where the real treasure is."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Regal Armor",
            description: "A suit of regal armor stands tall.",
            actions: &[
                EventScenarioAction {
                    label: "> Examine Armor",
                    dialog: &["This armor's gotta be worth something. Let's check it out."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Walk Past",
                    dialog: &["Just old armor. Not gonna help us find treasure."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Jeweled Scepter",
            description: "A jeweled scepter lies on a pedestal.",
            actions: &[
                EventScenarioAction {
                    label: "> Handle Scepter",
                    dialog: &["That scepter looks valuable. I'm gonna take a closer look."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> View from Distance",
                    dialog: &["Things on pedestals are usually trapped. Let's not touch it."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        
    ],
};

#[rustfmt::skip]
pub const CURSED_VAULT_LOCATION_DATA: EventLocationData = EventLocationData {
    name: "Cursed Vault",
    images: &["treasure_room", "treasure_room"],
    dialog: &["Now that's quite the pile o' loot, innit?"],
    description: "A room filled with cursed treasures, each with its own tale. Wealth and grave misfortune await those who dare to claim them in equal measure.",
    scenarios: &[
        EventScenario {
            name: "Overflowing Chests",
            description: "Chests overflowing with gold and jewels.",
            actions: &[
                EventScenarioAction {
                    label: "> Open Chests",
                    dialog: &["Can't just leave these chests unopened. Who knows what's inside!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Just Look",
                    dialog: &["Look but don't touch. Sometimes the prettiest treasures are the most cursed."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Sparkling Gems",
            description: "Gems of every color sparkle brilliantly.",
            actions: &[
                EventScenarioAction {
                    label: "> Collect Gems",
                    dialog: &["Gems! These'll fetch a pretty penny. Let's grab a handful."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Admire Sparkles",
                    dialog: &["Such bright sparkles. Almost too nice to take... almost."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Mysterious Artifacts",
            description: "Artifacts with unknown powers line the shelves.",
            actions: &[
                EventScenarioAction {
                    label: "> Touch Artifacts",
                    dialog: &["These artifacts look mighty interesting. Let's see what they do."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Observe Only",
                    dialog: &["Best not to mess with unknown magic. I'll just look, thanks."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Precious Paintings",
            description: "Rare paintings hang on the walls, exuding history.",
            actions: &[
                EventScenarioAction {
                    label: "> Inspect Paintings",
                    dialog: &["These paintings could be worth a fortune. Let's take a closer look."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> View from Afar",
                    dialog: &["I'm no art critic, but I know better than to touch ancient paintings."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Golden Statues",
            description: "Statues made of solid gold catch your eye.",
            actions: &[
                EventScenarioAction {
                    label: "> Examine Statues",
                    dialog: &["Gold statues! Now that's what I'm talking about! Let's check 'em out."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Resist Temptation",
                    dialog: &["Golden statues are always trapped. Better not touch."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Locked Safes",
            description: "Safes locked tightly, holding unknown treasures.",
            actions: &[
                EventScenarioAction {
                    label: "> Attempt to Open",
                    dialog: &["A locked safe is just a challenge. Let's crack it open!"],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Leave Unopened",
                    dialog: &["Locked safes in treasure rooms? Smells like a trap to me."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Exotic Weapons",
            description: "Weapons of exotic make and origin are displayed.",
            actions: &[
                EventScenarioAction {
                    label: "> Handle Weapons",
                    dialog: &["Exotic weapons? Don't mind if I do. Let's have a look."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Keep Hands Off",
                    dialog: &["Tempting, but I'm not touching weapons I don't know. Could be cursed."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        EventScenario {
            name: "Rare Books",
            description: "Shelves of books contain rare and ancient knowledge.",
            actions: &[
                EventScenarioAction {
                    label: "> Read Books",
                    dialog: &["Rare books could hold rare secrets. Let's take a peek."],
                    outcomes: DEFAULT_RISKY_ACTION_OUTCOMES,
                },
                EventScenarioAction {
                    label: "> Leave Untouched",
                    dialog: &["Leave the reading to the wizards. I'm here for the shiny stuff."],
                    outcomes: DEFAULT_SAFE_ACTION_OUTCOMES,
                },
            ],
        },
        
    ],
};
