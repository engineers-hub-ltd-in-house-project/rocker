use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};

/// Generate a UUID string
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Generate a short unique ID (12 characters)
pub fn generate_short_id() -> String {
    let uuid = Uuid::new_v4();
    let bytes = uuid.as_bytes();
    
    // Use only the first 8 bytes of the UUID to create a 12-character ID
    general_purpose::URL_SAFE_NO_PAD.encode(&bytes[0..6])
}

/// Generate a container name
pub fn generate_container_name() -> String {
    // Generate a random adjective and noun combination
    let adjectives = [
        "admiring", "adoring", "affectionate", "agitated", "amazing",
        "angry", "awesome", "beautiful", "blissful", "bold",
        "boring", "brave", "busy", "charming", "clever",
        "cool", "compassionate", "competent", "confident", "crazy",
        "dazzling", "determined", "distracted", "dreamy", "eager",
        "ecstatic", "elastic", "elated", "elegant", "eloquent",
        "epic", "exciting", "fervent", "festive", "flamboyant",
        "focused", "friendly", "frosty", "funny", "gallant",
        "gifted", "gracious", "great", "happy", "hardcore",
        "heuristic", "hopeful", "hungry", "infallible", "inspiring",
        "intelligent", "interesting", "jolly", "jovial", "keen",
        "kind", "laughing", "loving", "lucid", "magical",
        "modest", "musing", "mystifying", "naughty", "nervous",
        "nice", "nifty", "nostalgic", "optimistic", "peaceful",
        "pedantic", "pensive", "practical", "priceless", "quirky",
        "quizzical", "relaxed", "reverent", "romantic", "sad",
        "serene", "sharp", "silly", "sleepy", "stoic",
        "strange", "suspicious", "sweet", "tender", "thirsty",
        "trusting", "unruffled", "upbeat", "vibrant", "vigilant",
        "vigorous", "wizardly", "wonderful", "xenodochial", "youthful",
        "zealous", "zen"
    ];

    let nouns = [
        "albattani", "allen", "almeida", "antonelli", "agnesi",
        "archimedes", "ardinghelli", "aryabhata", "austin", "babbage",
        "banach", "banzai", "bardeen", "bartik", "bassi",
        "beaver", "bell", "benz", "bhabha", "bhaskara",
        "blackburn", "blackwell", "bohr", "booth", "borg",
        "bose", "bouman", "boyd", "brahmagupta", "brattain",
        "brown", "buck", "burnell", "cannon", "carson",
        "cartwright", "carver", "cerf", "chandrasekhar", "chaplygin",
        "chatelet", "chatterjee", "chebyshev", "cohen", "chaum",
        "clarke", "colden", "cori", "cray", "curran",
        "curie", "darwin", "davinci", "dewdney", "dhawan",
        "diffie", "dijkstra", "dirac", "driscoll", "dubinsky",
        "easley", "edison", "einstein", "elbakyan", "elgamal",
        "elion", "ellis", "engelbart", "euclid", "euler",
        "faraday", "feistel", "fermat", "fermi", "feynman",
        "franklin", "gagarin", "galileo", "galois", "ganguly",
        "gates", "gauss", "germain", "goldberg", "goldstine",
        "goldwasser", "golick", "goodall", "gould", "greider",
        "grothendieck", "haibt", "hamilton", "haslett", "hawking",
        "hellman", "heisenberg", "hermann", "herschel", "hertz",
        "heyrovsky", "hodgkin", "hofstadter", "hoover", "hopper",
        "hugle", "hypatia", "ishizaka", "jackson", "jang",
        "jennings", "jepsen", "johnson", "joliot", "jones",
        "kalam", "kapitsa", "kare", "keldysh", "keller",
        "kepler", "khayyam", "khorana", "kilby", "kirch",
        "knuth", "kowalevski", "lalande", "lamarr", "lamport",
        "leakey", "leavitt", "lederberg", "lehmann", "lewin",
        "lichterman", "liskov", "lovelace", "lumiere", "mahavira",
        "margulis", "matsumoto", "maxwell", "mayer", "mccarthy",
        "mcclintock", "mclaren", "mclean", "mcnulty", "mendel",
        "mendeleev", "meitner", "meninsky", "merkle", "mestorf",
        "mirzakhani", "montalcini", "moore", "morse", "murdock",
        "moser", "napier", "nash", "neumann", "newton",
        "nightingale", "nobel", "noether", "northcutt", "noyce",
        "panini", "pare", "pascal", "pasteur", "payne",
        "perlman", "pike", "poincare", "poitras", "proskuriakova",
        "ptolemy", "raman", "ramanujan", "ride", "ritchie",
        "rhodes", "robinson", "roentgen", "rosalind", "rubin",
        "saha", "sammet", "sanderson", "satoshi", "shamir",
        "shannon", "shaw", "shirley", "shockley", "shtern",
        "sinoussi", "snyder", "solomon", "spence", "stonebraker",
        "sutherland", "swanson", "swartz", "swirles", "taussig",
        "tesla", "tharp", "thompson", "torvalds", "tu",
        "turing", "varahamihira", "vaughan", "visvesvaraya", "volhard",
        "villani", "wescoff", "wilbur", "wiles", "williams",
        "williamson", "wilson", "wing", "wozniak", "wright",
        "wu", "yalow", "yonath", "zhukovsky", "zuse"
    ];

    let mut rng = rand::thread_rng();
    let adjective = adjectives[rng.gen_range(0..adjectives.len())];
    let noun = nouns[rng.gen_range(0..nouns.len())];

    format!("{}_{}", adjective, noun)
}

/// Generate a random string of the given length
pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
} 
