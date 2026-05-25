use serde::{Deserialize, Serialize};

/// A named preset for just the Amp plugin parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmpPreset {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub gain: f32,
    pub master: f32,
    pub bass: f32,
    pub mid: f32,
    pub treble: f32,
    pub drive: f32,
    /// 0.0 = guitar EQ, 1.0 = bass EQ (shifted frequencies)
    pub bass_mode: f32,
}

impl AmpPreset {
    pub fn new(name: &str, description: &str, tags: Vec<&str>, gain: f32, master: f32, bass: f32, mid: f32, treble: f32, drive: f32) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            tags: tags.into_iter().map(|s| s.to_string()).collect(),
            gain: gain.clamp(0.0, 1.0),
            master: master.clamp(0.0, 1.0),
            bass: bass.clamp(0.0, 1.0),
            mid: mid.clamp(0.0, 1.0),
            treble: treble.clamp(0.0, 1.0),
            drive: drive.clamp(0.0, 1.0),
            bass_mode: 0.0,
        }
    }

    /// Create a bass-specific preset with bass_mode=1.0 (shifted EQ).
    pub fn new_bass(name: &str, description: &str, tags: Vec<&str>, gain: f32, master: f32, bass: f32, mid: f32, treble: f32, drive: f32) -> Self {
        let mut p = Self::new(name, description, tags, gain, master, bass, mid, treble, drive);
        p.bass_mode = 1.0;
        p
    }

    pub fn to_parameter_map(&self) -> std::collections::HashMap<String, f32> {
        let mut map = std::collections::HashMap::new();
        map.insert("gain".to_string(), self.gain);
        map.insert("master".to_string(), self.master);
        map.insert("bass".to_string(), self.bass);
        map.insert("mid".to_string(), self.mid);
        map.insert("treble".to_string(), self.treble);
        map.insert("drive".to_string(), self.drive);
        map.insert("bass_mode".to_string(), self.bass_mode);
        map
    }
}

fn clean_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("American Clean", "Crisp & sparkling Fender-style clean. Bright highs, punchy lows, bell-like clarity.", vec!["clean", "fender", "american", "versatile", "bright"], 0.15, 0.80, 0.60, 0.50, 0.60, 0.10),
        AmpPreset::new("Twin Clean", "Fender Twin Reverb. Massive headroom, glassy top-end, scooped mids.", vec!["clean", "fender", "twin", "headroom", "glassy"], 0.10, 0.85, 0.50, 0.35, 0.55, 0.05),
        AmpPreset::new("Deluxe Clean", "Fender Deluxe Reverb. Sweet compression, warm lows, chimey highs.", vec!["clean", "fender", "deluxe", "sweet", "touch-sensitive"], 0.22, 0.65, 0.50, 0.55, 0.50, 0.08),
        AmpPreset::new("Princeton Clean", "Fender Princeton. Small-voice charm, round warm tone, vocal presence.", vec!["clean", "fender", "princeton", "vintage", "warm"], 0.18, 0.60, 0.55, 0.60, 0.45, 0.06),
        AmpPreset::new("Vibrolux Clean", "Fender Vibrolux. Scooped mids, lush low-end, shimmering top.", vec!["clean", "fender", "vibrolux", "shimmer", "scooped"], 0.12, 0.70, 0.60, 0.30, 0.65, 0.05),
        AmpPreset::new("Showman Clean", "Fender Showman. Maximum clean headroom, surf-ready, gigantic.", vec!["clean", "fender", "showman", "surf", "headroom"], 0.08, 0.90, 0.55, 0.45, 0.55, 0.03),
        AmpPreset::new("Bassman Clean", "Fender Bassman tweed. Fat low-end, woody midrange. Rock's foundation.", vec!["clean", "fender", "bassman", "tweed", "fat"], 0.25, 0.65, 0.60, 0.55, 0.45, 0.12),
        AmpPreset::new("Jazz Clean", "Warm, round, and mellow. Rolled-off treble, smooth vocal midrange.", vec!["clean", "jazz", "warm", "smooth", "round"], 0.20, 0.70, 0.70, 0.70, 0.30, 0.05),
        AmpPreset::new("Boutique Clean", "Dumble-inspired. Complex midrange, blooming harmonics, dynamic.", vec!["clean", "boutique", "dumble", "dynamic", "complex"], 0.25, 0.75, 0.55, 0.65, 0.50, 0.15),
        AmpPreset::new("SSS Clean", "Dumble Steel String Singer. Crystal clear, massive dynamic range.", vec!["clean", "boutique", "dumble", "sss", "crystal"], 0.10, 0.80, 0.55, 0.55, 0.55, 0.05),
        AmpPreset::new("Two Rock Clean", "Two Rock TS-1. Crystal boutique clean with rich overtones and 3D presence.", vec!["clean", "boutique", "two-rock", "modern", "premium"], 0.15, 0.75, 0.50, 0.60, 0.60, 0.08),
        AmpPreset::new("AC15 Clean", "Vox AC15. Chimey, jangly British bite. Less headroom than the 30.", vec!["clean", "vox", "ac15", "british", "chime"], 0.25, 0.60, 0.40, 0.55, 0.70, 0.15),
        AmpPreset::new("AC30 Chime", "Vox AC30 top-boost chime. Jangly highs, cutting mids. British Invasion.", vec!["clean", "vox", "ac30", "british", "jangle"], 0.20, 0.70, 0.40, 0.60, 0.70, 0.10),
        AmpPreset::new("AC30TB Bright", "Vox AC30 top-boost dimed. Extra-brilliant, percussive, edge of breakup.", vec!["clean", "vox", "ac30", "top-boost", "bright"], 0.30, 0.65, 0.35, 0.55, 0.80, 0.20),
        AmpPreset::new("Roland JC Clean", "Roland Jazz Chorus. Glassy, pristine, almost sterile purity.", vec!["clean", "roland", "jc-120", "glass", "pristine"], 0.10, 0.80, 0.50, 0.45, 0.60, 0.03),
        AmpPreset::new("Matchless Clean", "Matchless DC-30. Vox meets Hiwatt — chimey, punchy, authoritative lows.", vec!["clean", "matchless", "boutique", "british", "punchy"], 0.20, 0.70, 0.55, 0.55, 0.60, 0.12),
        AmpPreset::new("BadCat Clean", "Bad Cat Hot Cat. Vox-inspired boutique with tighter lows, aggressive chime.", vec!["clean", "badcat", "boutique", "vox-style"], 0.20, 0.65, 0.50, 0.55, 0.65, 0.15),
        AmpPreset::new("Dr Z Clean", "Dr. Z MAZ 18. Class-A boutique with chime, punch, and rich harmonics.", vec!["clean", "dr-z", "boutique", "class-a", "harmonic"], 0.20, 0.65, 0.50, 0.60, 0.55, 0.12),
        AmpPreset::new("Lonestar Clean", "Mesa Lonestar. Fender-style clean with Mesa's signature low-end girth.", vec!["clean", "mesa", "lonestar", "american", "big"], 0.15, 0.70, 0.60, 0.50, 0.50, 0.08),
        AmpPreset::new("Mesa Mark Clean", "Mesa Mark IIC+ clean. Warm, slightly compressed, unique mid-voicing.", vec!["clean", "mesa", "mark-iic", "american", "warm"], 0.20, 0.65, 0.55, 0.60, 0.50, 0.10),
        AmpPreset::new("5150 Clean", "EVH 5150 clean. Round, full, with a surprising edge. Not your typical high-gain clean.", vec!["clean", "evh", "5150", "modern", "full"], 0.25, 0.65, 0.60, 0.55, 0.55, 0.15),
        AmpPreset::new("Milkman Clean", "Milkman Creamer. Modern boutique with vintage soul. Sweet, detailed, addictive.", vec!["clean", "milkman", "boutique", "modern-vintage"], 0.18, 0.70, 0.50, 0.60, 0.55, 0.08),
    ]
}

fn crunch_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("British Crunch", "Marshall Plexi into natural breakup. The classic rock crunch that defined a generation.", vec!["crunch", "marshall", "plexi", "british", "classic-rock"], 0.55, 0.60, 0.60, 0.50, 0.55, 0.50),
        AmpPreset::new("Plexi 1959", "Marshall Super Lead 100W. Open, punchy, harmonically rich when pushed.", vec!["crunch", "marshall", "plexi", "super-lead", "vintage"], 0.55, 0.55, 0.55, 0.50, 0.55, 0.55),
        AmpPreset::new("Plexi 1987", "Marshall Super Bass 100W. Tighter low-end than the 1959, punk-rock favorite.", vec!["crunch", "marshall", "plexi", "super-bass", "tight"], 0.55, 0.55, 0.60, 0.50, 0.55, 0.50),
        AmpPreset::new("JTM45 Crunch", "Marshall JTM45. Bluesier, more organic than the Plexi. Early Keith Richards.", vec!["crunch", "marshall", "jtm45", "bluesy", "vintage"], 0.50, 0.60, 0.55, 0.55, 0.50, 0.45),
        AmpPreset::new("Blues Breakup", "Pushed just past the edge. Touch-sensitive, dynamic, responsive. Blues nirvana.", vec!["crunch", "blues", "edge-of-breakup", "touch-sensitive", "dynamic"], 0.40, 0.65, 0.55, 0.60, 0.50, 0.35),
        AmpPreset::new("Texas Blues", "SRV-style hot blues. Big lows, punchy mids, singing highs.", vec!["crunch", "blues", "texas", "srv", "hot"], 0.50, 0.60, 0.50, 0.70, 0.60, 0.45),
        AmpPreset::new("AC30 Crunch", "Vox AC30 pushed into snarling breakup. Chime turns to harmonic-rich bite.", vec!["crunch", "vox", "ac30", "british", "snarl"], 0.50, 0.55, 0.40, 0.55, 0.65, 0.50),
        AmpPreset::new("AC15 Crunch", "Smaller Vox pushed harder. More aggressive breakup with that same jangle.", vec!["crunch", "vox", "ac15", "british", "aggressive"], 0.55, 0.50, 0.40, 0.55, 0.65, 0.55),
        AmpPreset::new("800 Crunch", "JCM800 rhythm crunch. Tight, aggressive, percussive. 80s hard rock backbone.", vec!["crunch", "marshall", "jcm800", "hard-rock", "rhythm"], 0.50, 0.55, 0.55, 0.45, 0.60, 0.55),
        AmpPreset::new("Plexi Drive", "Marshall Super Lead cranked. Raw, punchy, barely contained.", vec!["crunch", "marshall", "plexi", "cranked", "wild"], 0.60, 0.50, 0.55, 0.50, 0.60, 0.60),
        AmpPreset::new("Bassman Pushed", "Fender Bassman tweed at 7. Big, fat, barking breakup. Early rock and roll.", vec!["crunch", "fender", "bassman", "tweed", "bark"], 0.50, 0.55, 0.60, 0.55, 0.50, 0.45),
        AmpPreset::new("Tweed Deluxe", "Fender Deluxe tweed full tilt. Compressed, warm, singing breakup.", vec!["crunch", "fender", "tweed", "deluxe", "singing"], 0.55, 0.50, 0.50, 0.60, 0.45, 0.55),
        AmpPreset::new("Orange Crunch", "Orange AD30 pushed. Thick, mid-forward crunch with distinctive British bark.", vec!["crunch", "orange", "ad30", "british", "thick"], 0.50, 0.55, 0.55, 0.60, 0.50, 0.55),
        AmpPreset::new("Trainwreck Express", "Trainwreck at breakup. Legendary touch sensitivity. Clean to mean with pick attack.", vec!["crunch", "trainwreck", "boutique", "dynamic"], 0.50, 0.55, 0.50, 0.65, 0.55, 0.50),
        AmpPreset::new("Dumble ODS Crunch", "Dumble Overdrive Special on edge. Smooth, complex, vocal. The Holy Grail of breakup.", vec!["crunch", "boutique", "dumble", "ods", "holy-grail"], 0.50, 0.60, 0.55, 0.70, 0.50, 0.50),
        AmpPreset::new("Hiwatt Crunch", "Hiwatt DR-103 pushed. High headroom, then punchy articulate crunch when dimed.", vec!["crunch", "hiwatt", "british", "articulate", "high-headroom"], 0.55, 0.55, 0.55, 0.45, 0.60, 0.50),
    ]
}

fn classic_rock_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Whole Lotta", "Jimmy Page's Plexi. Vocal mids, slightly overdriven, that famous percussive attack.", vec!["classic-rock", "led-zeppelin", "page", "marshall", "plexi"], 0.55, 0.55, 0.55, 0.60, 0.55, 0.55),
        AmpPreset::new("AC/DC Crunch", "Angus Young's cranked Plexi. Raw, mid-forward, no low-end flab. Pure rock.", vec!["classic-rock", "acdc", "angus", "marshall"], 0.60, 0.55, 0.40, 0.65, 0.60, 0.60),
        AmpPreset::new("Aerosmith Lead", "Joe Perry's hot-rodded Marshall. Smooth singing lead with just enough hair.", vec!["classic-rock", "aerosmith", "perry", "lead"], 0.60, 0.55, 0.50, 0.55, 0.55, 0.60),
        AmpPreset::new("ZZ Top Crunch", "Billy Gibbons' Pearly Gates. Fat, mid-pushed, with a touch of grind.", vec!["classic-rock", "zz-top", "gibbons", "fat"], 0.50, 0.55, 0.60, 0.65, 0.50, 0.50),
        AmpPreset::new("Brown Sugar", "Keith Richards' JTM45 pushed. Open, airy, mid-rich. The riff master.", vec!["classic-rock", "rolling-stones", "richards", "jtm45"], 0.50, 0.60, 0.55, 0.60, 0.50, 0.45),
        AmpPreset::new("Won't Get Fooled", "Pete Townshend's Hiwatt stack. Punchy, articulate windmill power chord crunch.", vec!["classic-rock", "the-who", "townshend", "hiwatt", "power-chords"], 0.55, 0.60, 0.55, 0.50, 0.60, 0.50),
        AmpPreset::new("You Really Got Me", "Dave Davies' cranked ElPico. The original distorted rock tone. Razor-edged.", vec!["classic-rock", "kinks", "davies", "vintage-distortion"], 0.60, 0.50, 0.40, 0.55, 0.65, 0.65),
        AmpPreset::new("Proud Mary", "John Fogerty's Bassman clean-to-crunch. Swampy, mid-forward, unmistakable.", vec!["classic-rock", "creedence", "fogerty", "bassman"], 0.45, 0.60, 0.55, 0.60, 0.50, 0.40),
        AmpPreset::new("American Girl", "Tom Petty's Vox AC30 chime. Jangling, bright, relentlessly optimistic.", vec!["classic-rock", "tom-petty", "vox", "jangle"], 0.35, 0.65, 0.40, 0.55, 0.70, 0.30),
        AmpPreset::new("Ace of Spades", "Lemmy's Marshall full-stack grind. Mid-heavy, aggressive, zero bass flab.", vec!["classic-rock", "motorhead", "lemmy", "marshall", "aggressive"], 0.60, 0.55, 0.35, 0.70, 0.55, 0.65),
        AmpPreset::new("Highway Star", "Ritchie Blackmore's Marshall lead. Cutting, aggressive, classically-tinged.", vec!["classic-rock", "deep-purple", "blackmore", "lead"], 0.55, 0.55, 0.50, 0.55, 0.60, 0.60),
        AmpPreset::new("Rebel Rebel", "David Bowie's Mick Ronson tone. Marshally, crystalline, larger than life.", vec!["classic-rock", "bowie", "ronson", "glam-rock"], 0.50, 0.60, 0.50, 0.50, 0.65, 0.45),
        AmpPreset::new("Free Bird", "Gary Rossington's Plexi lead. Singing, sustained, deeply Southern rock.", vec!["classic-rock", "lynyrd-skynyrd", "rossington", "southern-rock"], 0.55, 0.55, 0.55, 0.60, 0.55, 0.55),
        AmpPreset::new("Heartbreaker", "Page's Plexi on the verge of chaos. Aggressive, biting, about-to-explode sound.", vec!["classic-rock", "led-zeppelin", "page", "aggressive"], 0.60, 0.50, 0.50, 0.55, 0.60, 0.65),
    ]
}

fn blues_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Texas Flood", "SRV's cranked Vibroverb. Massive, glassy, touch-sensitive. The sound of Texas.", vec!["blues", "srv", "texas-flood", "fender", "vibroverb"], 0.50, 0.60, 0.55, 0.65, 0.60, 0.45),
        AmpPreset::new("BB King", "BB's Gibson Lab Series. Smooth, vocal, warm mids. Never harsh, always singing.", vec!["blues", "bb-king", "smooth", "vocal", "warm"], 0.30, 0.70, 0.60, 0.70, 0.40, 0.20),
        AmpPreset::new("Bluesbreaker", "Clapton's Marshall Bluesbreaker combo. The Beano album tone — pure, organic, legendary.", vec!["blues", "clapton", "bluesbreaker", "marshall", "john-mayall"], 0.45, 0.60, 0.55, 0.55, 0.55, 0.40),
        AmpPreset::new("Woman Tone", "Clapton's Cream-era SG into cranked Marshall. Fat, sustaining, deeply vocal.", vec!["blues", "clapton", "woman-tone", "cream", "fat"], 0.55, 0.55, 0.60, 0.60, 0.50, 0.55),
        AmpPreset::new("Buddy Guy", "Buddy's cranked Fender. Pure Chicago blues — aggressive, raw, crying.", vec!["blues", "buddy-guy", "chicago", "fender", "raw"], 0.55, 0.55, 0.50, 0.60, 0.55, 0.55),
        AmpPreset::new("Albert King", "Albert's Flying V into cranked solid state. Piercing, string-bending sustain.", vec!["blues", "albert-king", "flying-v", "bending"], 0.55, 0.55, 0.40, 0.50, 0.70, 0.60),
        AmpPreset::new("Albert Collins", "Master of the Telecaster. Ice-pick treble, biting attack, deeply funky.", vec!["blues", "albert-collins", "telecaster", "ice-pick"], 0.40, 0.65, 0.30, 0.55, 0.80, 0.35),
        AmpPreset::new("Peter Green", "Greeny's out-of-phase Les Paul into cranked Marshall. Warm, crying, supernatural.", vec!["blues", "peter-green", "fleetwood-mac", "les-paul"], 0.50, 0.55, 0.55, 0.60, 0.55, 0.50),
        AmpPreset::new("Muddy Waters", "Muddy's small Fender. Delta grit through Chicago amp. Raw, authoritative.", vec!["blues", "muddy-waters", "chicago", "delta"], 0.50, 0.55, 0.55, 0.60, 0.50, 0.50),
        AmpPreset::new("Howlin' Wolf", "Hubert Sumlin's dark tone. Grindy, unpredictable. Blues' dangerous edge.", vec!["blues", "howlin-wolf", "sumlin", "dark"], 0.55, 0.50, 0.65, 0.55, 0.45, 0.60),
        AmpPreset::new("Gary Moore Blues", "Gary's cranked Marshall lead. Thick, singing, deeply emotional.", vec!["blues", "gary-moore", "marshall", "lead", "emotional"], 0.60, 0.55, 0.55, 0.55, 0.55, 0.60),
        AmpPreset::new("Robben Ford", "Robben's Dumble ODS. Smooth, complex, jazz-blues perfection. Touch-sensitive.", vec!["blues", "robben-ford", "dumble", "smooth", "jazz-blues"], 0.45, 0.60, 0.55, 0.70, 0.50, 0.40),
        AmpPreset::new("Larry Carlton", "Larry's Dumble 335 tone. Warm, vocal, bell-like. Jazz-blues royalty.", vec!["blues", "larry-carlton", "dumble", "335", "vocal"], 0.40, 0.65, 0.55, 0.65, 0.55, 0.35),
        AmpPreset::new("John Lee Hooker", "One-chord voodoo. Dark, grinding, primitive. Amps shouldn't sound this good.", vec!["blues", "john-lee-hooker", "primitive", "dark"], 0.55, 0.55, 0.65, 0.50, 0.35, 0.60),
        AmpPreset::new("T-Bone Walker", "The pioneer. Sharp, cutting, single-note perfection. Jazz-influenced blues.", vec!["blues", "t-bone-walker", "vintage", "cutting"], 0.35, 0.65, 0.45, 0.55, 0.65, 0.30),
        AmpPreset::new("Freddie King", "Freddie's brown-faced Fender. Thick, punchy, with vocal midrange.", vec!["blues", "freddie-king", "fender", "punchy"], 0.50, 0.55, 0.55, 0.60, 0.55, 0.50),
        AmpPreset::new("Otis Rush", "West Side Chicago. Upside-down guitar, inside-out soul. Mid-heavy, crying top.", vec!["blues", "otis-rush", "chicago", "crying"], 0.50, 0.55, 0.50, 0.65, 0.55, 0.50),
    ]
}

fn hard_rock_metal_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Brown Sound", "EVH's iconic tone. Moderate gain, scooped-yet-mid bark, percussive attack. Frankenstrat required.", vec!["lead", "van-halen", "brown", "evh", "classic"], 0.60, 0.55, 0.60, 0.30, 0.70, 0.60),
        AmpPreset::new("EVH 1984", "Van Halen's 1984 album. Hotter, tighter, more saturated than the brown sound.", vec!["lead", "van-halen", "1984", "evh", "hot"], 0.65, 0.55, 0.60, 0.25, 0.65, 0.65),
        AmpPreset::new("Hot Rod Lead", "Hot-rodded Marshall. More gain, more mids, more sustain. Quintessential hard rock lead.", vec!["lead", "hot-rodded", "marshall", "sustain", "singing"], 0.70, 0.50, 0.50, 0.45, 0.60, 0.70),
        AmpPreset::new("800 Lead", "JCM800 boosted for lead. Tight low-end, cutting mids, iconic British bite.", vec!["lead", "marshall", "jcm800", "boosted", "cutting"], 0.65, 0.50, 0.50, 0.40, 0.65, 0.65),
        AmpPreset::new("JCM900 Lead", "Marshall JCM900. Thicker, more saturated than the 800. That 90s rock crunch.", vec!["lead", "marshall", "jcm900", "90s", "thick"], 0.70, 0.50, 0.55, 0.40, 0.55, 0.70),
        AmpPreset::new("JVM Lead", "Marshall JVM. Modern Marshall high-gain. Versatile, tight, aggressive.", vec!["lead", "marshall", "jvm", "modern", "high-gain"], 0.75, 0.45, 0.55, 0.35, 0.55, 0.75),
        AmpPreset::new("Modern Metal", "5150-inspired. Tight, aggressive. The modern metal standard. Chug machine.", vec!["high-gain", "metal", "5150", "modern", "chug"], 0.85, 0.40, 0.65, 0.20, 0.55, 0.85),
        AmpPreset::new("EVH 5150", "EVH 5150-III. Tighter low-end, aggressive mid-voice, legendary evolution.", vec!["high-gain", "evh", "5150", "tight", "legendary"], 0.85, 0.40, 0.60, 0.25, 0.55, 0.85),
        AmpPreset::new("5150 Red", "EVH 5150-III red channel. Saturated, singing, tight. Modern high-gain benchmark.", vec!["high-gain", "evh", "5150", "red-channel", "saturated"], 0.90, 0.35, 0.60, 0.20, 0.55, 0.90),
        AmpPreset::new("Rectifier Modern", "Mesa Dual Rectifier modern. Wall of gain, massive lows, scooped mids.", vec!["high-gain", "mesa", "rectifier", "modern", "massive"], 0.80, 0.45, 0.65, 0.15, 0.55, 0.80),
        AmpPreset::new("Rectifier Vintage", "Rectifier on vintage. Looser, creamier gain with more mid presence.", vec!["high-gain", "mesa", "rectifier", "vintage", "creamy"], 0.75, 0.45, 0.60, 0.25, 0.55, 0.75),
        AmpPreset::new("Rectifier Raw", "Rectifier raw mode. Unfiltered, aggressive, less compressed. Punishing.", vec!["high-gain", "mesa", "rectifier", "raw", "aggressive"], 0.80, 0.40, 0.65, 0.20, 0.60, 0.85),
        AmpPreset::new("Triple Recto", "Mesa Triple Rectifier. Even more lows, even more gain. When the Dual isn't enough.", vec!["high-gain", "mesa", "triple-rectifier", "brutal"], 0.85, 0.40, 0.70, 0.15, 0.50, 0.85),
        AmpPreset::new("Mark IIC+", "Mesa Mark IIC+. The grail. Tight, focused, signature mid-voice. Thrash perfected.", vec!["high-gain", "mesa", "mark-iic", "thrash", "tight"], 0.80, 0.40, 0.55, 0.30, 0.55, 0.80),
        AmpPreset::new("Mark III", "Mesa Mark III. Raunchier than the IIC+ with more low-end punch. Hetfield's early sound.", vec!["high-gain", "mesa", "mark-iii", "punchy"], 0.80, 0.40, 0.60, 0.25, 0.55, 0.80),
        AmpPreset::new("Mark IV", "Mesa Mark IV. Versatile high-gain with Mark-series mid-range. Studio legend.", vec!["high-gain", "mesa", "mark-iv", "versatile"], 0.80, 0.40, 0.55, 0.30, 0.55, 0.80),
        AmpPreset::new("Mark V", "Mesa Mark V. Modern evolution. 3 channels, every Mark tone in one box.", vec!["high-gain", "mesa", "mark-v", "modern", "versatile"], 0.80, 0.40, 0.55, 0.25, 0.60, 0.80),
        AmpPreset::new("SLO-100", "Soldano SLO-100. The definitive high-gain lead. Smooth, saturated, singing.", vec!["high-gain", "soldano", "slo-100", "lead", "smooth"], 0.80, 0.45, 0.55, 0.30, 0.55, 0.80),
        AmpPreset::new("Soldano Avenger", "Soldano Avenger. Tighter, more aggressive than the SLO. Modern grind.", vec!["high-gain", "soldano", "avenger", "tight", "aggressive"], 0.80, 0.40, 0.55, 0.25, 0.60, 0.85),
        AmpPreset::new("Soldano Hot Rod", "Soldano Hot Rod 100. SLO-inspired with more flexibility. Signature sizzle.", vec!["high-gain", "soldano", "hot-rod", "sizzle"], 0.80, 0.45, 0.55, 0.30, 0.60, 0.80),
        AmpPreset::new("ENGL Powerball", "ENGL Powerball. Tight German precision-gain. Sterile and brutal.", vec!["high-gain", "engl", "powerball", "german", "precision"], 0.85, 0.40, 0.60, 0.20, 0.55, 0.85),
        AmpPreset::new("ENGL Savage", "ENGL Savage. Even tighter, more aggressive than the Powerball. Metal perfection.", vec!["high-gain", "engl", "savage", "tight", "metal"], 0.90, 0.35, 0.60, 0.20, 0.55, 0.90),
        AmpPreset::new("ENGL Fireball", "ENGL Fireball. Punchy, aggressive, less low-end flub. Rhythm machine.", vec!["high-gain", "engl", "fireball", "rhythm", "punchy"], 0.85, 0.40, 0.55, 0.25, 0.55, 0.85),
        AmpPreset::new("ENGL Blackmore", "ENGL Blackmore. Ritchie's signature. Tight, clear, classical articulation.", vec!["high-gain", "engl", "blackmore", "articulate"], 0.80, 0.40, 0.55, 0.30, 0.60, 0.80),
        AmpPreset::new("Diezel VH4", "Diezel VH4. The sound of modern metal. Tight, huge, three-dimensional.", vec!["high-gain", "diezel", "vh4", "modern", "huge"], 0.85, 0.40, 0.60, 0.25, 0.55, 0.85),
        AmpPreset::new("Diezel Herbert", "Diezel Herbert. Massive, crushing low-end with signature mid complexity.", vec!["high-gain", "diezel", "herbert", "massive", "crushing"], 0.85, 0.40, 0.65, 0.20, 0.50, 0.85),
        AmpPreset::new("Diezel Einstein", "Diezel Einstein. Smaller Diezel, still huge. Punchy, articulate, aggressive.", vec!["high-gain", "diezel", "einstein", "articulate"], 0.80, 0.45, 0.60, 0.30, 0.55, 0.80),
        AmpPreset::new("Bogner Uberschall", "Bogner Uberschall. Earth-shaking lows, searing highs. The definition of br00tal.", vec!["high-gain", "bogner", "uberschall", "brutal", "heavy"], 0.90, 0.35, 0.70, 0.15, 0.55, 0.90),
        AmpPreset::new("Bogner Ecstasy", "Bogner Ecstasy. Versatile high-gain with legendary harmonic richness.", vec!["high-gain", "bogner", "ecstasy", "versatile", "harmonic"], 0.80, 0.45, 0.55, 0.30, 0.55, 0.80),
        AmpPreset::new("Bogner Shiva", "Bogner Shiva. Clean-to-crunch with gorgeous singing lead voice.", vec!["high-gain", "bogner", "shiva", "singing"], 0.75, 0.50, 0.55, 0.35, 0.55, 0.75),
        AmpPreset::new("Randall Satan", "Randall Satan by Mike Fortin. Tight, angry, uncompromising brutality.", vec!["high-gain", "randall", "satan", "fortin", "brutal"], 0.90, 0.35, 0.65, 0.15, 0.60, 0.90),
        AmpPreset::new("Fryette VHT", "Fryette Deliverance. Crushing, aggressive, no-nonsense high-gain.", vec!["high-gain", "fryette", "vht", "deliverance", "crushing"], 0.85, 0.40, 0.60, 0.25, 0.55, 0.85),
        AmpPreset::new("Krank Revolution", "Krank Revolution. Searing, aggressive, unique mid-voicing. Metal machine.", vec!["high-gain", "krank", "revolution", "aggressive"], 0.85, 0.40, 0.60, 0.20, 0.60, 0.85),
        AmpPreset::new("Framus Cobra", "Framus Cobra. Tight German high-gain with massive low-end punch.", vec!["high-gain", "framus", "cobra", "german", "tight"], 0.85, 0.40, 0.65, 0.20, 0.55, 0.85),
        AmpPreset::new("Laney AOR", "Laney AOR. The 80s metal sleeper. Thick, raw, ballsy low-end.", vec!["high-gain", "laney", "aor", "80s", "raw"], 0.80, 0.45, 0.65, 0.30, 0.55, 0.80),
        AmpPreset::new("Ampeg VH140C", "Ampeg VH140C. Solid state brutality. The OG death metal sound. Razor-sharp.", vec!["high-gain", "ampeg", "vh140c", "death-metal", "solid-state"], 0.90, 0.40, 0.65, 0.15, 0.65, 0.90),
        AmpPreset::new("Peavey 6505", "Peavey 6505/5150. The iconic American high-gain sound. Metal standard.", vec!["high-gain", "peavey", "6505", "american", "standard"], 0.85, 0.40, 0.65, 0.20, 0.55, 0.85),
        AmpPreset::new("Peavey 6505+", "Peavey 6505+. Tighter, more gain, more saturation. Modern evolution.", vec!["high-gain", "peavey", "6505-plus", "modern", "tight"], 0.85, 0.40, 0.65, 0.20, 0.55, 0.85),
        AmpPreset::new("Fortin NATAS", "Fortin NATAS. Extreme gain saturation with ungodly tightness. A masterpiece of noise.", vec!["high-gain", "fortin", "natas", "extreme", "tight"], 0.95, 0.30, 0.60, 0.15, 0.60, 0.95),
    ]
}

fn modern_metal_prog_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Djent", "Tight, percussive low-end. Polyrhythm-ready with controlled gain and massive attack.", vec!["high-gain", "djent", "tight", "percussive", "prog"], 0.75, 0.45, 0.65, 0.25, 0.65, 0.80),
        AmpPreset::new("Periphery", "Modern high-gain with tight bass and cutting mids. The Periphery rhythm sound.", vec!["high-gain", "periphery", "modern", "tight", "rhythm"], 0.80, 0.40, 0.60, 0.30, 0.55, 0.85),
        AmpPreset::new("Meshuggah", "The ultimate chug. Fortin-designed, impossibly tight, mathematically precise.", vec!["high-gain", "meshuggah", "fortin", "chug", "mathematical"], 0.90, 0.35, 0.65, 0.20, 0.60, 0.90),
        AmpPreset::new("Tesseract", "Ambient, precise modern metal. Tight distortion with room to breathe.", vec!["high-gain", "tesseract", "modern", "ambient", "djent"], 0.80, 0.45, 0.60, 0.30, 0.55, 0.80),
        AmpPreset::new("Monuments", "Groove-oriented modern metal. Percussive, syncopated, massive.", vec!["high-gain", "monuments", "djent", "groove"], 0.80, 0.40, 0.60, 0.25, 0.55, 0.85),
        AmpPreset::new("Animals as Leaders", "Progressive, clean, precise. Orchestral metal with extreme clarity.", vec!["high-gain", "animals-as-leaders", "prog", "precise"], 0.75, 0.45, 0.55, 0.30, 0.60, 0.75),
        AmpPreset::new("Architects", "Modern UK metalcore. Big lows, sampled precision, massive walls of sound.", vec!["high-gain", "architects", "metalcore", "modern", "uk"], 0.85, 0.40, 0.65, 0.20, 0.55, 0.85),
        AmpPreset::new("Northlane", "Atmospheric modern metal with crushing lows and ambient mids.", vec!["high-gain", "northlane", "atmospheric", "modern"], 0.80, 0.45, 0.65, 0.25, 0.55, 0.80),
        AmpPreset::new("Parkway Drive", "Australian metalcore. Bold, aggressive, rhythm-focused.", vec!["high-gain", "parkway-drive", "metalcore", "aggressive"], 0.85, 0.40, 0.60, 0.25, 0.55, 0.85),
        AmpPreset::new("Killswitch Engage", "Melodic metalcore. Harmonious brutality. Big gain with singing mids.", vec!["high-gain", "killswitch-engage", "metalcore", "melodic"], 0.80, 0.45, 0.60, 0.30, 0.55, 0.80),
        AmpPreset::new("Trivium", "Modern thrash-infused metal. Aggressive, technical, cutting.", vec!["high-gain", "trivium", "thrash", "modern", "technical"], 0.80, 0.40, 0.55, 0.25, 0.60, 0.85),
        AmpPreset::new("Gojira", "Unique French metal. Crushing, rhythmic, signature low-end thump.", vec!["high-gain", "gojira", "french", "rhythmic", "unique"], 0.85, 0.40, 0.70, 0.20, 0.50, 0.85),
        AmpPreset::new("Lamb of God", "American groove metal. Aggressive, percussive, wall-of-sound rhythm.", vec!["high-gain", "lamb-of-god", "groove-metal", "american"], 0.80, 0.45, 0.60, 0.25, 0.55, 0.85),
        AmpPreset::new("Opeth", "Progressive Swedish metal. From clean to crushing, dynamic range for days.", vec!["high-gain", "opeth", "prog", "swedish", "dynamic"], 0.75, 0.50, 0.55, 0.35, 0.55, 0.75),
        AmpPreset::new("Mastodon", "Sludgey, progressive, massive. Thick low-end with psychedelic mids.", vec!["high-gain", "mastodon", "sludge", "prog", "thick"], 0.80, 0.45, 0.65, 0.30, 0.50, 0.80),
        AmpPreset::new("Tool", "Adam Jones' Diezel/Darkglass hybrid. Big, dark, atmospheric. Tension personified.", vec!["high-gain", "tool", "adam-jones", "diezel", "dark"], 0.70, 0.50, 0.60, 0.35, 0.50, 0.70),
        AmpPreset::new("Deftones", "Stephen Carpenter's 5150 wall. Shoegaze meets metal. Thick, ethereal, punishing.", vec!["high-gain", "deftones", "shoegaze-metal", "atmospheric"], 0.80, 0.45, 0.65, 0.30, 0.50, 0.80),
    ]
}

fn doom_stoner_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Doom", "Thick, dark, sludgy. Massive low-end, rolled-off highs. Riffs that shake the earth.", vec!["doom", "sludge", "heavy", "dark", "low-end"], 0.70, 0.65, 0.90, 0.40, 0.15, 0.65),
        AmpPreset::new("Electric Wizard", "Sabbath-worship through 1000 watts. Filthy, terrifying, impossibly slow.", vec!["doom", "electric-wizard", "filthy", "slow"], 0.80, 0.60, 0.95, 0.35, 0.10, 0.75),
        AmpPreset::new("Sleep", "The riff is the law. Dopesmoker-thick, earth-moving sustain.", vec!["doom", "sleep", "stoner", "riff", "thick"], 0.75, 0.60, 0.90, 0.40, 0.15, 0.70),
        AmpPreset::new("Kyuss", "Blue-green desert tone. Thick, fuzzy, rolling. QOTSA before QOTSA.", vec!["doom", "kyuss", "stoner", "desert", "fuzzy"], 0.65, 0.60, 0.80, 0.45, 0.25, 0.60),
        AmpPreset::new("QOTSA", "Josh Homme's stone-cold tone. Mid-pushed, grinding, attitude to spare.", vec!["doom", "qotsa", "homme", "stoner", "grinding"], 0.65, 0.55, 0.70, 0.60, 0.30, 0.65),
        AmpPreset::new("Fu Manchu", "California desert stoner. Fast, fuzzy, relentless. Low-end for days.", vec!["doom", "fu-manchu", "stoner", "desert", "fast"], 0.65, 0.55, 0.80, 0.40, 0.30, 0.65),
        AmpPreset::new("Clutch", "Tim Sult's Neil Young-meets-Metal tone. Grinding, grooving, unmistakable.", vec!["doom", "clutch", "groove", "grinding"], 0.60, 0.55, 0.65, 0.55, 0.35, 0.60),
        AmpPreset::new("Corrosion", "Pepper Keenan's sludgy Southern tone. Thick, dirty, deeply Southern.", vec!["doom", "corrosion-of-conformity", "sludge", "southern"], 0.65, 0.55, 0.70, 0.50, 0.30, 0.65),
        AmpPreset::new("Down", "NOLA sludge. Phil Anselmo's crushing, thick-as-molasses tone.", vec!["doom", "down", "nola", "sludge", "southern"], 0.70, 0.55, 0.80, 0.45, 0.20, 0.70),
        AmpPreset::new("Crowbar", "Kirk Windstein's signature. The heaviest sludge — low, slow, brutal.", vec!["doom", "crowbar", "sludge", "heavy", "slow"], 0.75, 0.55, 0.90, 0.35, 0.15, 0.75),
        AmpPreset::new("Monolord", "Swedish doom. Massive, hypnotic, fuzzed-out repetition. Beautiful darkness.", vec!["doom", "monolord", "swedish", "fuzz", "hypnotic"], 0.75, 0.60, 0.90, 0.35, 0.10, 0.70),
        AmpPreset::new("Windhand", "Droning, ethereal doom. Thick enough to swim through, pretty enough to cry to.", vec!["doom", "windhand", "ethereal", "drone"], 0.70, 0.60, 0.85, 0.40, 0.20, 0.65),
        AmpPreset::new("YOB", "Mike Scheidt's psychedelic doom. Deep, spiritual, crushing. Riffs from another dimension.", vec!["doom", "yob", "psychedelic", "spiritual"], 0.70, 0.60, 0.85, 0.45, 0.20, 0.65),
        AmpPreset::new("Conan", "British extreme doom. Minimalist, monolithic, completely overwhelming.", vec!["doom", "conan", "extreme", "monolithic"], 0.80, 0.55, 0.95, 0.30, 0.10, 0.80),
        AmpPreset::new("Sunn O)))", "The sound of the void. Minimal guitar, maximum amplifier. Worship the sun.", vec!["doom", "sunn", "drone", "experimental", "void"], 0.70, 0.70, 0.90, 0.30, 0.05, 0.60),
        AmpPreset::new("Acid King", "Lori's fuzzed-out San Francisco doom. Thick, heavy, hypnotic.", vec!["doom", "acid-king", "stoner", "sf"], 0.70, 0.55, 0.85, 0.40, 0.20, 0.70),
    ]
}

fn punk_hardcore_alt_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Punk", "Angry mid-forward distortion. Raw, cutting, designed to be heard over a loud band.", vec!["punk", "aggressive", "raw", "mid-forward"], 0.60, 0.55, 0.50, 0.65, 0.55, 0.60),
        AmpPreset::new("Ramones", "Johnny's mid-range attack. All mids, no bass, straight into a Vox or Marshall.", vec!["punk", "ramones", "johnny-ramone", "downstrokes"], 0.50, 0.60, 0.30, 0.75, 0.55, 0.45),
        AmpPreset::new("God Save The Queen", "Steve Jones' cranked Marshall. The blueprint for UK punk. Snotty, perfect.", vec!["punk", "sex-pistols", "steve-jones", "uk"], 0.60, 0.55, 0.45, 0.60, 0.60, 0.60),
        AmpPreset::new("London Calling", "The Clash's wall of Marshalls. Raw, powerful, mid-forward rock and roll.", vec!["punk", "the-clash", "mick-jones", "uk"], 0.55, 0.55, 0.50, 0.60, 0.55, 0.55),
        AmpPreset::new("Dead Kennedys", "East Bay Ray's twangy surf-meets-hardcore. Reverb-soaked punk at its finest.", vec!["punk", "dead-kennedys", "east-bay-ray", "surf-punk"], 0.50, 0.55, 0.40, 0.55, 0.65, 0.50),
        AmpPreset::new("Black Flag", "Greg Ginn's atonal, buzzsaw tone. Unhinged, chaotic, utterly unique.", vec!["punk", "black-flag", "greg-ginn", "buzzsaw"], 0.65, 0.50, 0.40, 0.60, 0.60, 0.65),
        AmpPreset::new("Minor Threat", "Lyle's tight hardcore tone. No frills, all fury.", vec!["punk", "minor-threat", "hardcore", "tight"], 0.60, 0.55, 0.45, 0.60, 0.55, 0.60),
        AmpPreset::new("Dookie", "Green Day Dookie album. Pushed Marshall with punchy mids and bright top.", vec!["punk", "green-day", "dookie", "marshall", "90s"], 0.55, 0.55, 0.50, 0.60, 0.60, 0.55),
        AmpPreset::new("American Idiot", "Green Day's modern wall. Bigger, more polished, still punk at heart.", vec!["punk", "green-day", "american-idiot", "modern"], 0.60, 0.55, 0.55, 0.55, 0.55, 0.60),
        AmpPreset::new("Blink Crunch", "Tom DeLonge's oversized Marshall. Big, scooped, pop-punk perfection.", vec!["punk", "blink-182", "delonge", "pop-punk"], 0.55, 0.55, 0.65, 0.30, 0.65, 0.55),
        AmpPreset::new("Rancid", "Tim Armstrong's raw, punky tone. Reggae-influenced grind meets hardcore.", vec!["punk", "rancid", "armstrong", "ska-punk"], 0.55, 0.55, 0.50, 0.60, 0.55, 0.55),
        AmpPreset::new("NOFX", "Fat Mike's punk crunch. Bright, fast, irreverent. Punk at warp speed.", vec!["punk", "nofx", "fat-mike", "fast"], 0.55, 0.55, 0.45, 0.55, 0.60, 0.55),
        AmpPreset::new("Grunge", "Raw, unpolished, broken-in amp pushed past its limits. The Seattle sound.", vec!["grunge", "seattle", "raw", "aggressive", "broken"], 0.60, 0.50, 0.55, 0.50, 0.45, 0.65),
        AmpPreset::new("Nevermind", "Kurt's DS-1 into a cranked Fender/Mesa. Crunchy, messy, iconic. Smells like teen spirit.", vec!["grunge", "nirvana", "cobain", "nevermind", "fender"], 0.55, 0.55, 0.50, 0.50, 0.55, 0.55),
        AmpPreset::new("In Utero", "Cobain's Twin Reverb cranked into the red. Fierce, uncompromising, raw.", vec!["grunge", "nirvana", "cobain", "in-utero", "fender"], 0.60, 0.50, 0.50, 0.55, 0.55, 0.65),
        AmpPreset::new("Pearl Jam", "Mike McCready's cranked Fender/Marshall. Solo-soaring, blues-infused grunge.", vec!["grunge", "pearl-jam", "mccready", "lead"], 0.55, 0.55, 0.55, 0.55, 0.55, 0.55),
        AmpPreset::new("Superunknown", "Chris Cornell's wall of Marshalls. Dark, heavy, brooding grunge.", vec!["grunge", "soundgarden", "cornell", "dark", "heavy"], 0.60, 0.55, 0.60, 0.45, 0.50, 0.60),
        AmpPreset::new("Dirt", "Jerry Cantrell's thick, sludgy leads. Les Paul into cranked amp. Pure Alice.", vec!["grunge", "alice-in-chains", "cantrell", "sludge", "thick"], 0.60, 0.55, 0.60, 0.50, 0.45, 0.60),
        AmpPreset::new("The Strokes", "Julian's Vox-like jangle. Lo-fi, charming, cutting through a crowded room.", vec!["alt", "the-strokes", "vox", "lo-fi"], 0.40, 0.60, 0.40, 0.60, 0.60, 0.35),
        AmpPreset::new("White Stripes", "Jack White's crumbling, no-bass tone. Plasticene fury through a broken amp.", vec!["alt", "white-stripes", "jack-white", "lo-fi", "raw"], 0.55, 0.55, 0.20, 0.70, 0.60, 0.60),
        AmpPreset::new("Arctic Monkeys", "Alex Turner's Vox chime. British indie with bite and jangle.", vec!["alt", "arctic-monkeys", "turner", "vox", "british"], 0.45, 0.60, 0.40, 0.55, 0.65, 0.40),
        AmpPreset::new("Foo Fighters", "Dave Grohl's cranked Mesa. Big, anthemic, powerful. The sound of rock revival.", vec!["alt", "foo-fighters", "grohl", "mesa", "anthemic"], 0.60, 0.55, 0.55, 0.45, 0.55, 0.60),
        AmpPreset::new("Smashing Pumpkins", "Billy's Big Muff into a Marshall. Huge, wall-of-fuzz, zero subtlety.", vec!["alt", "smashing-pumpkins", "corgan", "wall-of-sound"], 0.60, 0.55, 0.60, 0.40, 0.55, 0.65),
    ]
}

fn jazz_fusion_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Wes Montgomery", "Thumb-picked warmth through a Fender Twin. Round, octave-heavy, deeply soulful.", vec!["jazz", "wes-montgomery", "fender", "warm", "thumb"], 0.20, 0.70, 0.55, 0.65, 0.40, 0.10),
        AmpPreset::new("George Benson", "GB's hot-rodded clean. Punchy, articulate, brilliant single-note attack.", vec!["jazz", "george-benson", "boutique", "punchy"], 0.30, 0.70, 0.50, 0.65, 0.50, 0.20),
        AmpPreset::new("Pat Metheny", "Pat's ultra-clean Roland JC. Chorusy, lush, pitch-perfect clarity.", vec!["jazz", "pat-metheny", "roland", "clean", "lush"], 0.15, 0.75, 0.50, 0.50, 0.60, 0.08),
        AmpPreset::new("John Scofield", "Sco's gritty, mid-heavy tone. Dirty but sophisticated. Jazz with a snarl.", vec!["jazz", "john-scofield", "gritty", "mid-heavy"], 0.45, 0.60, 0.50, 0.70, 0.50, 0.40),
        AmpPreset::new("Allan Holdsworth", "Ultra-clean, horn-like legato. Peak clarity, zero distortion. The master's canvas.", vec!["jazz", "fusion", "holdsworth", "legato", "clean"], 0.15, 0.80, 0.50, 0.55, 0.55, 0.05),
        AmpPreset::new("Frank Gambale", "Super-clean sweep-picking tone. Output for days, clarity forever.", vec!["jazz", "fusion", "gambale", "sweep", "crystal"], 0.20, 0.75, 0.50, 0.55, 0.60, 0.10),
        AmpPreset::new("Greg Howe", "Searing high-gain shred meets soulful phrasing. Technique with feel.", vec!["jazz", "fusion", "greg-howe", "shred", "high-gain"], 0.65, 0.55, 0.50, 0.40, 0.60, 0.65),
        AmpPreset::new("Guthrie Govan", "The modern master. Ultra-versatile — clean to mean, always musical.", vec!["jazz", "fusion", "guthrie-govan", "versatile"], 0.35, 0.65, 0.50, 0.55, 0.55, 0.30),
        AmpPreset::new("Andy Timmons", "Singing lead tone. Warm, expressive, gorgeous mid-range push.", vec!["jazz", "fusion", "andy-timmons", "singing", "lead"], 0.55, 0.55, 0.55, 0.60, 0.55, 0.55),
        AmpPreset::new("Eric Johnson", "EJ's meticulous, chiming clean. Layers upon layers of perfect tone.", vec!["jazz", "fusion", "eric-johnson", "chiming", "meticulous"], 0.30, 0.70, 0.50, 0.60, 0.65, 0.15),
        AmpPreset::new("Mike Stern", "High-octane jazz. Overdriven, aggressive, horn-like phrasing.", vec!["jazz", "mike-stern", "overdriven", "aggressive"], 0.50, 0.55, 0.50, 0.60, 0.55, 0.50),
        AmpPreset::new("Lee Ritenour", "Polished jazz clean. Smooth, professional, refined.", vec!["jazz", "lee-ritenour", "clean", "polished"], 0.25, 0.70, 0.50, 0.60, 0.55, 0.15),
    ]
}

fn country_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Nashville Clean", "Tele into a Fender Twin. Bright, twangy, chicken-pickin' snap.", vec!["country", "nashville", "telecaster", "twin", "twang"], 0.15, 0.80, 0.40, 0.50, 0.70, 0.05),
        AmpPreset::new("Brad Paisley", "Brad's cranked Fender DRRI. Twang with a bite. Modern country lead.", vec!["country", "brad-paisley", "fender", "twang", "lead"], 0.40, 0.65, 0.40, 0.55, 0.70, 0.35),
        AmpPreset::new("Brent Mason", "Nashville session legend. Ultra-clean, dynamic, perfect Tele snap.", vec!["country", "brent-mason", "session", "telecaster", "nashville"], 0.20, 0.75, 0.45, 0.55, 0.65, 0.10),
        AmpPreset::new("Johnny Hiland", "Blazing Tele through a clean Fender. Twang at warp speed.", vec!["country", "johnny-hiland", "telecaster", "fast", "twang"], 0.25, 0.70, 0.40, 0.50, 0.75, 0.15),
        AmpPreset::new("Vince Gill", "Singing, soulful country lead. Warm mids, clear top, tons of feel.", vec!["country", "vince-gill", "lead", "singing"], 0.30, 0.70, 0.50, 0.60, 0.55, 0.20),
        AmpPreset::new("Albert Lee", "Country legend. Bright, fast, impeccably clean with a cutting edge.", vec!["country", "albert-lee", "fast", "bright"], 0.25, 0.70, 0.40, 0.55, 0.70, 0.15),
        AmpPreset::new("James Burton", "Elvis's Tele man. Pioneering country-rock bite.", vec!["country", "james-burton", "telecaster", "vintage"], 0.30, 0.65, 0.40, 0.55, 0.65, 0.20),
        AmpPreset::new("Don Rich", "Buck Owens' right hand. Pure Bakersfield twang. Bright, cutting, classic.", vec!["country", "don-rich", "bakersfield", "twang"], 0.25, 0.65, 0.35, 0.50, 0.75, 0.15),
        AmpPreset::new("Chet Atkins", "Mr. Guitar. Warm, finger-picked perfection through a clean Fender.", vec!["country", "chet-atkins", "fingerstyle", "warm"], 0.20, 0.70, 0.50, 0.60, 0.50, 0.10),
        AmpPreset::new("Tele Twang", "Pure Tele bridge into a clean Fender. Country in a box.", vec!["country", "telecaster", "twang", "bridge"], 0.15, 0.75, 0.35, 0.50, 0.75, 0.05),
        AmpPreset::new("Outlaw Country", "Waylon's cranked Fender/Marshall mix. Darker, bigger, outlaw attitude.", vec!["country", "outlaw", "waylon", "dark"], 0.40, 0.60, 0.60, 0.55, 0.45, 0.35),
    ]
}

fn funk_rnb_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Nile Rodgers", "That Strat into a clean Roland. The most sampled guitar tone in history. Funky.", vec!["funk", "nile-rodgers", "roland", "strat", "iconic"], 0.10, 0.80, 0.40, 0.50, 0.65, 0.03),
        AmpPreset::new("Cory Wong", "Ultra-tight, compressed funk rhythm. Clean, percussive, impossibly tight.", vec!["funk", "cory-wong", "tight", "compressed"], 0.12, 0.80, 0.45, 0.55, 0.60, 0.05),
        AmpPreset::new("Prince", "Purple majesty. Crisp, funky, larger than life. Hohner into a Twin.", vec!["funk", "prince", "fender", "funky"], 0.20, 0.70, 0.45, 0.50, 0.65, 0.10),
        AmpPreset::new("James Brown", "Jimmy Nolan's rhythmic assault. Clean, percussive, relentless.", vec!["funk", "james-brown", "rhythm", "percussive"], 0.15, 0.75, 0.45, 0.55, 0.60, 0.08),
        AmpPreset::new("Sly Stone", "Sly's funky Fender. Deep pocket, rolling groove, swagger.", vec!["funk", "sly-stone", "vintage", "groove"], 0.20, 0.70, 0.50, 0.55, 0.55, 0.12),
        AmpPreset::new("Earth Wind & Fire", "Al McKay's clean, percussive rhythm. Tight as a drum, bright as the sun.", vec!["funk", "ewf", "al-mckay", "rhythm"], 0.15, 0.75, 0.40, 0.50, 0.65, 0.08),
        AmpPreset::new("Stevie Wonder", "Stevie's guitar tones. Lively, rhythmic, always in service of the song.", vec!["funk", "stevie-wonder", "rhythm", "groove"], 0.20, 0.70, 0.45, 0.55, 0.60, 0.12),
        AmpPreset::new("Anderson .Paak", "Modern funk-groove. Clean, bouncy, infectious rhythm guitar.", vec!["funk", "anderson-paak", "modern", "groove"], 0.20, 0.70, 0.45, 0.55, 0.60, 0.12),
    ]
}

fn shoegaze_dream_pop_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Loveless", "My Bloody Valentine's wall of sound. Drenched, swirling, glorious noise.", vec!["shoegaze", "my-bloody-valentine", "wall-of-sound", "noise"], 0.55, 0.60, 0.60, 0.40, 0.55, 0.55),
        AmpPreset::new("Slowdive", "Washy, ethereal, dreamlike. Soft distortion that floats rather than punches.", vec!["shoegaze", "slowdive", "ethereal", "dream-pop"], 0.45, 0.60, 0.55, 0.55, 0.50, 0.40),
        AmpPreset::new("Cocteau Twins", "Robin Guthrie's shimmering walls. Guitars as atmosphere.", vec!["shoegaze", "cocteau-twins", "shimmer", "atmospheric"], 0.35, 0.65, 0.50, 0.55, 0.65, 0.25),
        AmpPreset::new("Ride", "Oxford's finest. Swirling, jangly, just the right amount of grit.", vec!["shoegaze", "ride", "oxford", "jangle"], 0.50, 0.55, 0.50, 0.55, 0.60, 0.45),
        AmpPreset::new("Swervedriver", "Heavier shoegaze. Riff-driven, driving, less dream more machine.", vec!["shoegaze", "swervedriver", "heavy", "driving"], 0.60, 0.55, 0.55, 0.40, 0.55, 0.60),
        AmpPreset::new("Jesus and Mary Chain", "Feedback-soaked pop songs. Reznor-worship with pop hooks.", vec!["shoegaze", "jesus-and-mary-chain", "feedback", "noise-pop"], 0.60, 0.55, 0.45, 0.50, 0.60, 0.60),
        AmpPreset::new("Lush", "Ethereal, jangly, gorgeous. The lighter side of shoegaze, all melody.", vec!["shoegaze", "lush", "ethereal", "melodic"], 0.40, 0.65, 0.50, 0.55, 0.60, 0.30),
        AmpPreset::new("Chapterhouse", "Whirlpool-era swirl. Drenched in reverb, swimming in delay.", vec!["shoegaze", "chapterhouse", "swirl", "reverb"], 0.45, 0.60, 0.55, 0.50, 0.55, 0.40),
    ]
}

fn synthwave_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Synthwave Clean", "Bright, gated-clean tone for arpeggiated synthwave leads. Crisp attack, no mud. Riding the night.", vec!["synthwave", "retro", "clean", "arpeggio", "bright"], 0.15, 0.80, 0.35, 0.50, 0.75, 0.05),
        AmpPreset::new("Synthwave Lead", "Saturated single-note lead for soaring, anthemic lines. Slight breakup for warmth.", vec!["synthwave", "retro", "lead", "saturated", "soaring"], 0.50, 0.65, 0.45, 0.55, 0.70, 0.40),
        AmpPreset::new("OutRun Lead", "Punchy, gated lead with slight mid-scoop. Aggressive and clean. Testarossa decals included.", vec!["synthwave", "outrun", "lead", "gated", "punchy"], 0.45, 0.65, 0.55, 0.35, 0.65, 0.40),
        AmpPreset::new("Kavinsky Lead", "Compressed, aggressive arpeggio. Searing highs, tight low-end. Drive at night.", vec!["synthwave", "kavinsky", "arpeggio", "aggressive"], 0.50, 0.65, 0.40, 0.35, 0.75, 0.45),
        AmpPreset::new("Carpenter Brut", "Heavily distorted synthwave-metal. Dark, driving, relentless. The brutal side of retro.", vec!["synthwave", "carpenter-brut", "heavy", "dark", "distorted"], 0.70, 0.50, 0.60, 0.25, 0.60, 0.75),
        AmpPreset::new("Perturbator Dark", "Dark, reverb-drenched lead. Low-mid focused, crushed highs. Cyberpunk menace.", vec!["synthwave", "perturbator", "dark", "cyberpunk", "menacing"], 0.60, 0.55, 0.65, 0.40, 0.40, 0.55),
        AmpPreset::new("FM-84 Chime", "Bright, jangly, chorus-soaked rhythm. Sunset-drive Californian tone.", vec!["synthwave", "fm-84", "jangle", "bright", "chime"], 0.20, 0.75, 0.35, 0.50, 0.75, 0.10),
        AmpPreset::new("The Midnight Warm", "Warm, clean arpeggio tone. Slight compression, gentle top-end. Night-river cruise.", vec!["synthwave", "the-midnight", "warm", "arpeggio", "clean"], 0.20, 0.75, 0.50, 0.60, 0.55, 0.10),
        AmpPreset::new("Gunship Saturated", "Saturated, compressed riff tone. Thick layers of syncopated guitar over analog synths.", vec!["synthwave", "gunship", "saturated", "compressed"], 0.65, 0.55, 0.55, 0.40, 0.60, 0.65),
        AmpPreset::new("RetroWave Rhythm", "Clean, percussive, chorus-friendly rhythm. The backbone of any good synthwave track.", vec!["synthwave", "retrowave", "rhythm", "clean", "percussive"], 0.20, 0.75, 0.40, 0.55, 0.65, 0.10),
        AmpPreset::new("Miami Vice Clean", "Tuxedo-bright, pastel-clean. For those 80s detective show moments. The neon aesthetic.", vec!["synthwave", "miami-vice", "80s", "clean", "aesthetic"], 0.12, 0.80, 0.35, 0.50, 0.80, 0.05),
        AmpPreset::new("Dark Synth Lead", "Menacing, mid-heavy lead for dark synthwave. Giallo-inspired horror melody.", vec!["synthwave", "dark-synth", "lead", "horror", "mid-heavy"], 0.55, 0.60, 0.55, 0.60, 0.50, 0.55),
        AmpPreset::new("Cyberpunk Drive", "Gritty, distorted rhythm for dystopian cyberpunk riffs. Bladerunner meets hard rock.", vec!["synthwave", "cyberpunk", "drive", "dystopian", "gritty"], 0.65, 0.55, 0.60, 0.30, 0.55, 0.65),
        AmpPreset::new("Synth Bass", "Picked, sub-heavy tone mimicking analog synth bass. Low-end focus, rolled-off highs.", vec!["synthwave", "synth-bass", "low-end", "sub-heavy"], 0.25, 0.80, 0.90, 0.60, 0.15, 0.10),
    ]
}

fn ambient_experimental_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Ambient Clean", "Pristine, open clean. Maximum headroom for delay and reverb to work their magic.", vec!["ambient", "clean", "pristine", "pedal-platform"], 0.10, 0.85, 0.50, 0.55, 0.55, 0.03),
        AmpPreset::new("Frippertronics", "Robert Fripp's looping tone. Clear, articulate, extended high-frequency response.", vec!["ambient", "fripp", "loop", "experimental"], 0.20, 0.80, 0.50, 0.55, 0.60, 0.08),
        AmpPreset::new("Lanois Pad", "Daniel Lanois' ambient edge. Warm, organic, slightly broken. Beautiful.", vec!["ambient", "lanois", "warm", "organic"], 0.35, 0.70, 0.55, 0.60, 0.50, 0.25),
        AmpPreset::new("Bill Frisell", "Frisell's quirky, effects-laden clean. Surprising, textural, utterly unique.", vec!["ambient", "bill-frisell", "experimental", "textural"], 0.25, 0.70, 0.50, 0.55, 0.55, 0.15),
        AmpPreset::new("David Torn", "Splintered, processed, chaotic. A canvas for processing more than a tone.", vec!["ambient", "david-torn", "experimental", "processed"], 0.40, 0.65, 0.55, 0.50, 0.55, 0.35),
        AmpPreset::new("E-Bow Pad", "Clean, compressed, mid-pushed. Optimized for e-bow sustain and harmonic swells.", vec!["ambient", "e-bow", "sustain", "pad"], 0.20, 0.75, 0.55, 0.65, 0.50, 0.08),
        AmpPreset::new("Swell Pad", "Smooth, violin-like tone. Rolled-off attack for volume pedal swells.", vec!["ambient", "swell", "pad", "smooth"], 0.15, 0.80, 0.55, 0.60, 0.50, 0.05),
        AmpPreset::new("Reverse Ambience", "Mid-heavy, compressed tone for reverse delay. Haunting textures.", vec!["ambient", "reverse", "delay", "haunting"], 0.30, 0.70, 0.50, 0.60, 0.55, 0.20),
    ]
}

fn bass_amp_presets() -> Vec<AmpPreset> {
    vec![
        // ── Clean / Classic Rock ──
        AmpPreset::new_bass("SVT Clean", "Ampeg SVT. Massive lows, clear mids. The bass tone that defined rock. Deep, punchy, authoritative.", vec!["bass", "clean", "ampeg", "svt", "rock"], 0.20, 0.80, 0.85, 0.55, 0.30, 0.08),
        AmpPreset::new_bass("SVT Grind", "Ampeg SVT pushed. That classic rock grind with the SVT's unmistakable low-end thump.", vec!["bass", "crunch", "ampeg", "svt", "rock"], 0.45, 0.70, 0.85, 0.55, 0.30, 0.40),
        AmpPreset::new_bass("Bassman Bass", "Fender Bassman for bass. Warm, round woody tone. The sound of the P-bass through a tweed.", vec!["bass", "clean", "fender", "bassman", "vintage"], 0.20, 0.75, 0.80, 0.55, 0.25, 0.10),
        AmpPreset::new_bass("Vintage P-Bass", "Precision Bass into a tube amp. Fat, thumpy, fundamental. The sound of rock and roll.", vec!["bass", "clean", "p-bass", "vintage", "thumpy"], 0.25, 0.75, 0.80, 0.55, 0.30, 0.12),
        AmpPreset::new_bass("Vintage J-Bass", "Jazz Bass into a clean amp. Mid-forward with that signature bridge pickup growl.", vec!["bass", "clean", "j-bass", "vintage", "growl"], 0.25, 0.75, 0.70, 0.60, 0.40, 0.12),
        AmpPreset::new_bass("Flatwound P", "P-Bass with flats. Deep, thuddy, smooth. Motown and old-school soul.", vec!["bass", "clean", "flatwound", "motown", "smooth"], 0.20, 0.80, 0.85, 0.60, 0.15, 0.05),
        AmpPreset::new_bass("Motown Thump", "James Jamerson's tone. Deep, round, fingerstyle. The sound of the classic Motown records.", vec!["bass", "clean", "motown", "jamerson", "thump"], 0.20, 0.80, 0.90, 0.60, 0.20, 0.08),
        AmpPreset::new_bass("Studio Precision", "Clean, focused, sits perfectly in a mix. The engineer's choice for tracked P-bass.", vec!["bass", "clean", "studio", "precision", "session"], 0.18, 0.80, 0.75, 0.55, 0.35, 0.08),
        AmpPreset::new_bass("SVT Scooped", "Ampeg SVT with mids pulled back. Big lows and sizzling highs. Slap-friendly.", vec!["bass", "clean", "ampeg", "svt", "scooped"], 0.25, 0.80, 0.85, 0.25, 0.55, 0.12),
        AmpPreset::new_bass("British Valve", "Marshall Super Bass 100. Growly, aggressive, rock-solid low-end.", vec!["bass", "clean", "marshall", "british", "valve"], 0.30, 0.75, 0.75, 0.55, 0.40, 0.25),

        // ── Modern Rock / Alt ──
        AmpPreset::new_bass("Modern Rock", "Punchy mid-forward rock bass. Cut-through presence with tight lows.", vec!["bass", "rock", "modern", "punchy", "mid"], 0.35, 0.75, 0.70, 0.60, 0.45, 0.30),
        AmpPreset::new_bass("Alt Rock", "Alternative rock bass. Gritty, present, slightly overdriven edge.", vec!["bass", "rock", "alt", "gritty", "overdriven"], 0.40, 0.70, 0.70, 0.55, 0.45, 0.35),
        AmpPreset::new_bass("Indie Bass", "Clean, jangly indie bass tone. Articulate and punchy without being aggressive.", vec!["bass", "indie", "clean", "articulate"], 0.25, 0.78, 0.70, 0.55, 0.40, 0.15),
        AmpPreset::new_bass("Grind Bass", "Gritty, grinding rock tone. Broken-in tubes, aggressive pick attack, attitude.", vec!["bass", "rock", "grind", "aggressive", "pick"], 0.50, 0.65, 0.75, 0.55, 0.40, 0.50),
        AmpPreset::new_bass("Foo Fighters Bass", "Nate Mendel's clean-but-present tone. Medium grind, great note definition.", vec!["bass", "rock", "foo-fighters", "clean"], 0.30, 0.75, 0.70, 0.50, 0.45, 0.22),

        // ── Metal Bass ──
        AmpPreset::new_bass("Darkglass Drive", "Darkglass Microtubes. Modern aggressive bass with saturated distortion and crushing articulation.", vec!["bass", "metal", "darkglass", "drive", "aggressive"], 0.65, 0.55, 0.75, 0.35, 0.45, 0.60),
        AmpPreset::new_bass("Darkglass Alpha", "Darkglass Alpha Omega. Versatile metal distortion with growling mids and tight low-end.", vec!["bass", "metal", "darkglass", "alpha-omega", "tight"], 0.60, 0.55, 0.70, 0.40, 0.45, 0.60),
        AmpPreset::new_bass("Darkglass X", "Darkglass Ultra. Modern high-gain bass distortion. Cutting, aggressive, gnarly.", vec!["bass", "metal", "darkglass", "ultra", "high-gain"], 0.70, 0.50, 0.70, 0.35, 0.50, 0.70),
        AmpPreset::new_bass("Mesa Bass Drive", "Mesa Bass 400 pushed. Huge, aggressive growl with that Mesa mid-voice.", vec!["bass", "metal", "mesa", "drive", "growl"], 0.55, 0.60, 0.80, 0.45, 0.35, 0.55),
        AmpPreset::new_bass("SVT Metal", "Ampeg SVT into overdrive. Classic metal bass — thick, grinding, enormous.", vec!["bass", "metal", "ampeg", "svt", "crunch"], 0.55, 0.60, 0.80, 0.40, 0.35, 0.55),
        AmpPreset::new_bass("Djent Bass", "Tight, percussive modern metal. Compressed attack, controlled lows, massive mids.", vec!["bass", "metal", "djent", "tight", "percussive"], 0.60, 0.55, 0.65, 0.50, 0.50, 0.65),
        AmpPreset::new_bass("Tech Death", "Technical death metal bass. Articulate, cutting, extreme clarity at high speed.", vec!["bass", "metal", "tech-death", "articulate"], 0.55, 0.60, 0.65, 0.50, 0.55, 0.55),
        AmpPreset::new_bass("Black Metal Bass", "Treble-heavy, aggressive, cold. Minimal lows, cutting mids, raw attack.", vec!["bass", "metal", "black-metal", "cold", "aggressive"], 0.55, 0.60, 0.40, 0.55, 0.75, 0.60),
        AmpPreset::new_bass("Thrash Bass", "Aggressive picked thrash tone. Mid-forward, cutting, no low-end flab.", vec!["bass", "metal", "thrash", "picked", "aggressive"], 0.55, 0.60, 0.55, 0.60, 0.55, 0.60),
        AmpPreset::new_bass("Nu-Metal Bass", "Dropped-tuning aggressive bass. Big lows, scooped mids, percussive attack.", vec!["bass", "metal", "nu-metal", "dropped", "scooped"], 0.60, 0.55, 0.80, 0.20, 0.55, 0.65),
        AmpPreset::new_bass("Doom Bass", "Thick, sludgy, earth-shaking bass. Maximum low-end, rolled-off highs, crushing weight.", vec!["bass", "doom", "sludge", "heavy", "low"], 0.65, 0.60, 0.95, 0.35, 0.10, 0.65),
        AmpPreset::new_bass("Stoner Bass", "Fuzzy, thick, wall-of-sound bass. Big muff-style saturation for desert riffs.", vec!["bass", "stoner", "fuzz", "thick", "desert"], 0.60, 0.60, 0.85, 0.45, 0.25, 0.60),

        // ── Jazz / Fusion ──
        AmpPreset::new_bass("Jazz Bass Clean", "Warm, woody, articulate. The quintessential jazz bass tone. Rounds and upright-style.", vec!["bass", "jazz", "clean", "warm", "woody"], 0.20, 0.80, 0.75, 0.60, 0.30, 0.10),
        AmpPreset::new_bass("Fusion Bass", "Modern fusion. Clean, punchy, with extended highs for slap and solo work.", vec!["bass", "fusion", "clean", "punchy", "solo"], 0.25, 0.78, 0.65, 0.50, 0.50, 0.15),
        AmpPreset::new_bass("Jazz Solo", "Mid-forward solo bass tone. Singing, vocal, with cut-through presence.", vec!["bass", "jazz", "solo", "mid", "vocal"], 0.30, 0.75, 0.60, 0.70, 0.45, 0.25),
        AmpPreset::new_bass("Upright Sim", "Electric bass EQ'd to approximate upright bass. Warm, round, rolled-off top-end.", vec!["bass", "jazz", "upright", "sim", "acoustic"], 0.20, 0.80, 0.80, 0.65, 0.15, 0.08),
        AmpPreset::new_bass("Marcus Miller", "Marcus's slapped J-Bass. Aggressive mids, smacking highs, huge dynamic range.", vec!["bass", "jazz", "marcus-miller", "slap", "aggressive"], 0.35, 0.75, 0.60, 0.65, 0.55, 0.30),
        AmpPreset::new_bass("Victor Wooten", "Victor's signature modern tone. Clean, articulate, massive dynamic headroom.", vec!["bass", "fusion", "victor-wooten", "clean", "articulate"], 0.25, 0.80, 0.65, 0.55, 0.50, 0.12),
        AmpPreset::new_bass("Jaco Tone", "Jaco's fretless J-Bass tone. Mid-rich, singing, glassy top-end. The sound of electric bass revolution.", vec!["bass", "jazz", "jaco", "fretless", "mid"], 0.25, 0.80, 0.60, 0.70, 0.55, 0.15),
        AmpPreset::new_bass("Christian McBride", "Modern jazz master. Thick, round, deeply swinging. Walking lines that push the band.", vec!["bass", "jazz", "christian-mcbride", "walking", "thick"], 0.25, 0.78, 0.80, 0.60, 0.30, 0.15),

        // ── Funk / R&B ──
        AmpPreset::new_bass("Get Lucky", "Nathan East's 'Get Lucky' bass. Clean, warm fingerstyle funk. Pocket so deep you'll never leave the dancefloor.", vec!["bass", "funk", "daft-punk", "nathan-east", "pocket"], 0.25, 0.78, 0.65, 0.55, 0.40, 0.12),
        AmpPreset::new_bass("Funk Slap", "Percussive, bright, aggressive slap tone. Mid-scooped with punchy highs and thumping lows.", vec!["bass", "funk", "slap", "percussive", "bright"], 0.30, 0.80, 0.75, 0.25, 0.65, 0.18),
        AmpPreset::new_bass("Bootsy Funk", "Bootsy Collins' cosmic funk. Deep, round, with that rubbery envelope filter thing.", vec!["bass", "funk", "bootsy", "deep", "rubbery"], 0.20, 0.85, 0.85, 0.50, 0.40, 0.08),
        AmpPreset::new_bass("Larry Graham", "The originator of slap. Aggressive thumb attack, popping highs, earth-shaking lows.", vec!["bass", "funk", "larry-graham", "slap", "originator"], 0.35, 0.80, 0.75, 0.35, 0.60, 0.25),
        AmpPreset::new_bass("R&B Bass", "Smooth, round modern R&B. Clean, compressed, sits perfectly in the pocket.", vec!["bass", "rnb", "smooth", "clean", "pocket"], 0.20, 0.82, 0.80, 0.55, 0.35, 0.10),
        AmpPreset::new_bass("Neo-Soul Bass", "Warm, thumpy, sub-heavy. The modern neo-soul bass sound — deep and melodic.", vec!["bass", "neo-soul", "warm", "sub", "melodic"], 0.18, 0.85, 0.90, 0.55, 0.20, 0.08),
        AmpPreset::new_bass("P-Funk", "Parliament-Funkadelic rubbery goodness. Deep pocket, wah-inflected, hypnotic.", vec!["bass", "funk", "p-funk", "rubbery", "pocket"], 0.20, 0.82, 0.80, 0.50, 0.35, 0.12),

        // ── Reggae / Dub ──
        AmpPreset::new_bass("Dub Reggae", "Deep, round, sub-heavy. The foundation of dub reggae. Earth-shaking lows, minimal mids.", vec!["bass", "reggae", "dub", "sub", "deep"], 0.15, 0.88, 0.95, 0.40, 0.08, 0.05),
        AmpPreset::new_bass("Roots Reggae", "Traditional reggae. Deep walking basslines, warm fundamentals, woody attack.", vec!["bass", "reggae", "roots", "walking", "warm"], 0.18, 0.82, 0.85, 0.55, 0.20, 0.08),
        AmpPreset::new_bass("Rocksteady", "Classic rocksteady bass. Heavy, slow, deep. The transitional sound from ska to reggae.", vec!["bass", "reggae", "rocksteady", "heavy", "slow"], 0.20, 0.82, 0.85, 0.50, 0.15, 0.10),

        // ── Pop / Country ──
        AmpPreset::new_bass("Pop Bass", "Clean, focused, radio-ready. Tight lows, clear mids, sits perfectly in a pop mix.", vec!["bass", "pop", "clean", "tight", "radio"], 0.20, 0.80, 0.70, 0.55, 0.40, 0.10),
        AmpPreset::new_bass("Country Bass", "Fingerstyle country. Warm, woody with a pick attack. Nashville-approved.", vec!["bass", "country", "fingerstyle", "warm", "nashville"], 0.20, 0.78, 0.75, 0.55, 0.35, 0.12),
        AmpPreset::new_bass("Americana Bass", "Roots rock bass. Natural, dynamic, old-school tube warmth.", vec!["bass", "americana", "roots", "tube", "warm"], 0.25, 0.75, 0.75, 0.55, 0.30, 0.18),

        // ── Modern / Prog ──
        AmpPreset::new_bass("Modern Clean", "Ultra-clean modern bass. Extended range, high clarity, articulate every note.", vec!["bass", "modern", "clean", "articulate", "extended"], 0.22, 0.80, 0.70, 0.50, 0.45, 0.12),
        AmpPreset::new_bass("Prog Bass", "Progressive rock bass. Dynamic, articulate, with room for both delicate and aggressive playing.", vec!["bass", "prog", "dynamic", "articulate", "versatile"], 0.30, 0.78, 0.70, 0.55, 0.45, 0.25),
        AmpPreset::new_bass("Geddy Lee", "Geddy's signature growl. Pushed mids, Rickenbacker bite, that unmistakable run-through-your-fingers sound.", vec!["bass", "prog", "geddy-lee", "rush", "growl"], 0.40, 0.70, 0.60, 0.65, 0.50, 0.40),
        AmpPreset::new_bass("Chris Squire", "Squire's aggressive Rickenbacker growl. Bright, mid-pushed, percussive. Yes defined.", vec!["bass", "prog", "chris-squire", "rickenbacker", "aggressive"], 0.40, 0.70, 0.55, 0.65, 0.60, 0.40),
        AmpPreset::new_bass("John Entwistle", "The Ox. Aggressive, mid-forward, picked to death. Maximum attack, zero lows flab.", vec!["bass", "rock", "entwistle", "the-who", "picked"], 0.50, 0.65, 0.55, 0.65, 0.55, 0.55),

        // ── Pick / Aggressive ──
        AmpPreset::new_bass("Pick Attack", "Aggressive picked bass. Bright, percussive, attack-forward. For when you need to cut.", vec!["bass", "pick", "aggressive", "bright", "attack"], 0.30, 0.78, 0.65, 0.45, 0.55, 0.22),
        AmpPreset::new_bass("Punk Bass", "Raw, driving punk bass. Mid-forward, aggressive, simple and loud.", vec!["bass", "punk", "aggressive", "raw", "loud"], 0.50, 0.65, 0.65, 0.60, 0.50, 0.50),
        AmpPreset::new_bass("Hardcore Bass", "Hardcore punk aggression. Maximum cut, zero subtlety. Just lows and mids and fury.", vec!["bass", "punk", "hardcore", "aggressive", "cut"], 0.55, 0.60, 0.60, 0.65, 0.50, 0.60),

        // ── Drive / Fuzz ──
        AmpPreset::new_bass("Tube Drive", "Warm tube overdrive for bass. Smooth breakup, enhanced harmonics, fat compression.", vec!["bass", "drive", "tube", "warm", "overdrive"], 0.50, 0.70, 0.75, 0.50, 0.35, 0.50),
        AmpPreset::new_bass("Fuzz Bass", "Wall-of-fuzz bass. Thick, saturated, completely filthy. Woolly and massive.", vec!["bass", "fuzz", "wall", "thick", "filthy"], 0.65, 0.55, 0.80, 0.40, 0.25, 0.70),
        AmpPreset::new_bass("Synth Bass", "Simulated analog synth bass. Deep sub lows, rolled-off attack, filter-esque mids.", vec!["bass", "synth", "sub", "analog", "filtered"], 0.20, 0.85, 0.95, 0.45, 0.10, 0.08),
        AmpPreset::new_bass("Distorted Bass", "Full distortion on bass. Grinding, saturated, aggressive. For metal and heavy rock.", vec!["bass", "distortion", "metal", "aggressive", "saturated"], 0.70, 0.50, 0.70, 0.35, 0.45, 0.75),
        AmpPreset::new_bass("Octave Bass", "Simulated octave-down sub-bass. Massive low-end extension, synth-like depth.", vec!["bass", "octave", "sub", "deep", "synth"], 0.20, 0.85, 0.95, 0.40, 0.10, 0.10),
    ]
}

fn studio_utility_presets() -> Vec<AmpPreset> {
    vec![
        AmpPreset::new("Direct In", "Flat, uncolored. Use when your pedals do all the work — the amp is just a conduit.", vec!["studio", "direct", "flat", "pedal-platform"], 0.15, 0.80, 0.50, 0.50, 0.50, 0.05),
        AmpPreset::new("Reamp Clean", "Transparent, high-headroom platform for reamping. No coloration, just amplification.", vec!["studio", "reamp", "clean", "transparent"], 0.10, 0.85, 0.50, 0.50, 0.50, 0.03),
        AmpPreset::new("Pedal Platform", "Neutral, responsive clean tone that lets your pedals shine. The ideal blank canvas.", vec!["studio", "pedal-platform", "clean", "neutral"], 0.20, 0.80, 0.50, 0.55, 0.50, 0.10),
        AmpPreset::new("Double Tracker", "Slightly mid-scooped, bright. Sits in a different space from your main tone for width.", vec!["studio", "double-track", "width", "scooped"], 0.30, 0.70, 0.45, 0.35, 0.65, 0.20),
        AmpPreset::new("Acoustic Sim", "EQ'd to approximate an acoustic through a PA. Piezo-friendly.", vec!["studio", "acoustic-sim", "piezo", "bright"], 0.15, 0.75, 0.30, 0.60, 0.80, 0.05),
        AmpPreset::new("Piezo Clean", "Optimized for piezo bridge pickups. Extra headroom, rolled-off low-end, airy top.", vec!["studio", "piezo", "clean", "airy"], 0.12, 0.80, 0.30, 0.55, 0.75, 0.05),
        AmpPreset::new("Neck Pickup Warm", "Pushed low-mids for warm neck-pickup jazz tone. Thick, round, vocal.", vec!["studio", "neck-pickup", "warm", "jazz"], 0.25, 0.70, 0.65, 0.65, 0.35, 0.15),
        AmpPreset::new("Bridge Pickup Cut", "Mid-boosted for bridge pickup punch. Cuts through any mix.", vec!["studio", "bridge", "cutting", "punchy"], 0.35, 0.70, 0.45, 0.70, 0.55, 0.30),
    ]
}

pub fn built_in_amp_presets() -> Vec<AmpPreset> {
    let mut all = Vec::new();
    all.extend(clean_presets());
    all.extend(crunch_presets());
    all.extend(classic_rock_presets());
    all.extend(blues_presets());
    all.extend(hard_rock_metal_presets());
    all.extend(modern_metal_prog_presets());
    all.extend(doom_stoner_presets());
    all.extend(punk_hardcore_alt_presets());
    all.extend(jazz_fusion_presets());
    all.extend(country_presets());
    all.extend(funk_rnb_presets());
    all.extend(shoegaze_dream_pop_presets());
    all.extend(synthwave_presets());
    all.extend(ambient_experimental_presets());
    all.extend(bass_amp_presets());
    all.extend(studio_utility_presets());
    all
}
