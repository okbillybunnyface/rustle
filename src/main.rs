use rand::Rng;
use rand::rngs::ThreadRng;
use std::io;
use std::str::Chars;

#[derive(PartialEq)]
enum Score {
    Wrong,
    Miss,
    Hit,
    Nil,
}

struct Word {
    chars: [char; 5],
    score: [Score; 5],
}
impl Word {
    fn new(word: &str) -> Word {
        let mut chars: [char; 5] = [' '; 5];
        let mut iter: Chars = word.chars();
        for i in 0..5 {
            chars[i] = iter.next().unwrap();
        }
        Word{ chars, score: [Score::Nil, Score::Nil, Score::Nil, Score::Nil, Score::Nil] }
    }
    fn score(&mut self, target: &Word) {
        'outer: for i in 0..self.chars.len() {
            let c: &char = &self.chars[i];
            if *c == target.chars[i] {
                self.score[i] = Score::Hit;
                continue;
            }

            for j in 0..target.chars.len() {
                if *c == target.chars[j] {
                    self.score[i] = Score::Miss;
                    continue 'outer;
                }
            }

            self.score[i] = Score::Wrong;
        }
    }
    fn rand() -> Word {
        let words: [&str; 500] = ["about","above","abuse","actor","acute","admit","adopt","adult","after","again","agent","agree","ahead","alarm","album","alert","alike","alive","allow","alone","along","alter","among","anger","Angle","angry","apart","apple","apply","arena","argue","arise","array","aside","asset","audio","audit","avoid","award","aware","badly","baker","bases","basic","basis","beach","began","begin","begun","being","below","bench","billy","birth","black","blame","blind","block","blood","board","boost","booth","bound","brain","brand","bread","break","breed","brief","bring","broad","broke","brown","build","built","buyer","cable","calif","carry","catch","cause","chain","chair","chart","chase","cheap","check","chest","chief","child","china","chose","civil","claim","class","clean","clear","click","clock","close","coach","coast","could","count","court","cover","craft","crash","cream","crime","cross","crowd","crown","curve","cycle","daily","dance","dated","dealt","death","debut","delay","depth","doing","doubt","dozen","draft","drama","drawn","dream","dress","drill","drink","drive","drove","dying","eager","early","earth","eight","elite","empty","enemy","enjoy","enter","entry","equal","error","event","every","exact","exist","extra","faith","false","fault","fiber","field","fifth","fifty","fight","final","first","fixed","flash","fleet","floor","fluid","focus","force","forth","forty","forum","found","frame","frank","fraud","fresh","front","fruit","fully","funny","giant","given","glass","globe","going","grace","grade","grand","grant","grass","great","green","gross","group","grown","guard","guess","guest","guide","happy","harry","heart","heavy","hence","henry","horse","hotel","house","human","ideal","image","index","inner","input","issue","japan","jimmy","joint","jones","judge","known","label","large","laser","later","laugh","layer","learn","lease","least","leave","legal","level","lewis","light","limit","links","lives","local","logic","loose","lower","lucky","lunch","lying","magic","major","maker","march","maria","match","maybe","mayor","meant","media","metal","might","minor","minus","mixed","model","money","month","moral","motor","mount","mouse","mouth","movie","music","needs","never","newly","night","noise","north","noted","novel","nurse","occur","ocean","offer","often","order","other","ought","paint","panel","paper","party","peace","peter","phase","phone","photo","piece","pilot","pitch","place","plain","plane","plant","plate","point","pound","power","press","price","pride","prime","print","prior","prize","proof","proud","prove","queen","quick","quiet","quite","radio","raise","range","rapid","ratio","reach","ready","refer","right","rival","river","robin","roger","roman","rough","round","route","royal","rural","scale","scene","scope","score","sense","serve","seven","shall","shape","share","sharp","sheet","shelf","shell","shift","shirt","shock","shoot","short","shown","sight","since","sixth","sixty","sized","skill","sleep","slide","small","smart","smile","smith","smoke","solid","solve","sorry","sound","south","space","spare","speak","speed","spend","spent","split","spoke","sport","staff","stage","stake","stand","start","state","steam","steel","stick","still","stock","stone","stood","store","storm","story","strip","stuck","study","stuff","style","sugar","suite","super","sweet","table","taken","taste","taxes","teach","teeth","terry","texas","thank","theft","their","theme","there","these","thick","thing","think","third","those","three","threw","throw","tight","times","tired","title","today","topic","total","touch","tough","tower","track","trade","train","treat","trend","trial","tried","tries","truck","truly","trust","truth","twice","under","undue","union","unity","until","upper","upset","urban","usage","usual","valid","value","video","virus","visit","vital","voice","waste","watch","water","wheel","where","which","while","white","whole","whose","woman","women","world","worry","worse","worst","worth","would","wound","write","wrong","wrote","yield","young","youth"];
        let mut rng: ThreadRng = rand::thread_rng();
        let word: &str = words[rng.gen_range(0..words.len())];
        //println!("{}",word);
        Word::new(word)
    }
    fn score_string(&self) -> String {
        let mut output: String = String::new();
        for s in &self.score {
            match s {
                Score::Wrong => output.push_str("X"),
                Score::Miss => output.push_str("M"),
                Score::Hit => output.push_str("H"),
                Score::Nil => output.push_str(""),
            }
        }
        output
    }
    fn word_string(&self) -> String {
        let mut out: String = String::new();
        for c in self.chars {
            out.push(c);
        }
        out
    }
}

enum Turn {
    Guess(Word),
    Nil
}
impl Turn {
    fn new(target: &Word) -> Turn {
        let mut guess: String = String::new();
        'input: loop {
            println!("Please enter a 5-character word:");
            io::stdin().read_line(&mut guess).unwrap();
            guess = String::from(guess.trim());

            if guess.len() != 5 {
                println!("{} is not 5 characters. Please try again.", guess);
                guess = String::new();
                continue 'input;
            }
            else {
                break 'input
            }
        }

        let mut word: Word = Word::new(guess.as_str());
        word.score(target);
        Turn::Guess(word)
    }
    fn is_win(&self) -> bool {
        match self {
            Turn::Guess(word) => {
                let mut win: bool = true;
                for s in &word.score {
                    if *s != Score::Hit {
                        win = false;
                        break;
                    }
                }
                win
            },
            Turn::Nil => return false,
        }
    }
    fn score_string(&self) -> String {
        match self {
            Turn::Guess(word) => return word.score_string(),
            Turn::Nil => return String::new(),
        }
    }
}

#[derive(PartialEq)]
enum State {
    Playing,
    Win,
    Lose,
}

struct Game {
    turn: usize,
    state: State,
    target: Word,
    turns: [Turn; 6]
}
impl Game {
    fn new() -> Game {
        Game { 
            turn: 0, 
            state: State::Playing, 
            target: Word::rand(), 
            turns: [Turn::Nil, Turn::Nil, Turn::Nil, Turn::Nil, Turn::Nil, Turn::Nil], 
        }
    }
    fn take_turn(&mut self){
        self.turns[self.turn] = Turn::new(&self.target);
        if self.turns[self.turn].is_win() {
            self.state = State::Win;
            return;
        }
        self.turn += 1;
        if self.turn >= self.turns.len() {
            self.state = State::Lose;
            return;
        }
    }
}

fn main() {
    println!("Welcome to Rustle!");

    let mut game: Game = Game::new();
    while game.state == State::Playing {
        game.take_turn();
        println!("\nGame Summary ({} turns remaining):", game.turns.len() - game.turn);
        for t in &game.turns {
            match &t {
                &Turn::Guess(_word) => println!("{}", t.score_string()),
                &Turn::Nil => print!(""),
            }
        }
        println!("");
    }
    if game.state == State::Win {
        println!("Good job!");
    }
    if game.state == State::Lose {
        println!("Game over :(");
        println!("Word: {}", game.target.word_string());
    }
}