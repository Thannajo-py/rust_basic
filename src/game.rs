use rand::{thread_rng, Rng};
use std::io;
use regex::Regex;
use std::time::SystemTime;

pub fn guess_a_number(){
    loop {
        let answer:u8 = thread_rng().gen_range(1..101);
        let mut buffer = String::new();
        let mut p_choice: u8;
        let mut attempt: u8 = 1;
        println!("Devinez un nombre entre 1 et 100\nentrez votre réponse:");
        io::stdin().read_line(&mut buffer);
        let re = Regex::new(r"^[0-9]{1,3}").unwrap();
        while !re.is_match(buffer.trim()) {
            println!("Erreur, entrez un nombre entre 1 et 100:");
            buffer.clear();
            io::stdin().read_line(&mut buffer);
        }
        p_choice = buffer.trim().parse::<u8>().unwrap();
        while p_choice != answer && attempt < 6 {
            if p_choice < answer {
                println!("Trop petit!")
            } else if p_choice > answer {
                println!("Trop grand!")
            }
            println!("nombre d'essai: {}", attempt);
            println!("choisissez un autre nombre entre 1 et 100:");
            buffer.clear();
            io::stdin().read_line(&mut buffer);
            while !re.is_match(buffer.trim()) {
                println!("Erreur, entrez un nombre entre 1 et 100:");
                buffer.clear();
                io::stdin().read_line(&mut buffer);
            }
            p_choice = buffer.trim().parse::<u8>().unwrap();
            attempt += 1;
        }
        if p_choice == answer {
            println!("Bien joué, la réponse était bien {}", answer);
        } else {
            println!("Faux la réponse était {}", answer);
        }
        println!("voulez-vous rejouer(entrez 1) ou quitter?(entrez autre chose que 1)");
        buffer.clear();
        io::stdin().read_line(&mut buffer);
        if buffer.trim() != "1"{
            break;
        }
    }
}


pub fn bataille(){
    let mut buffer = String::new();
    println!("enter player1 name:");
    io::stdin().read_line(&mut buffer);
    let p1Name = String::from(buffer.trim());
    buffer.clear();
    println!("enter player2 name:");
    io::stdin().read_line(&mut buffer);
    let p2Name = String::from(buffer.trim());
    buffer.clear();
    println!("enter number of cards between 0 and 9999");
    io::stdin().read_line(&mut buffer);
    let re = Regex::new("^[0-9]{1,2}$").unwrap();
    while !re.is_match(buffer.trim()){
        println!("error not a number between 0 and 99");
        buffer.clear();
        io::stdin().read_line(&mut buffer);
    }
    let nbCards = buffer.trim().parse::<u8>().unwrap();
    let mut p1Deck = init();
    let mut p1Cards = nbCards;
    let mut p2Deck = init();
    let mut p2Cards = nbCards;
    let now = SystemTime::now();
    let mut round = 0;
    while p1Cards > 0 && p2Cards > 0{
        transfer_a_card( &mut p1Cards,  &mut p2Cards, &mut p1Deck, &mut p2Deck, &p1Name, &p2Name);
        round += 1;
    }
    if p1Cards <= 0 && p2Cards <= 0{
        println!("Egalité!");
    }
    else{
        println!("{} gagne!", if p1Cards == 0{p2Name} else{p1Name});
    }
    match now.elapsed(){
        Ok(elapsed) => println!("Total duration: {:010} ns.\nNombre de manche total: {}", elapsed.as_nanos(), round),
        Err(_e) => println!("Error")
    }

}

fn init() -> [u8;200]{
    let mut deck = [14;200];
    let draw = [0,1,2,3,4,5,6,7,8,9,10,11,12];
    for i in 0..200 {
        deck[i] = draw[i % draw.len()];
    }
    deck
}

fn draw_a_card(length:u8)->u8{
    thread_rng().gen_range(0..length)
}

fn transfer_a_card( p1Cards:&mut u8,p2Cards:&mut u8,
                   deckP1:&mut [u8;200], deckP2:&mut [u8;200],
                    p1Name:&str, p2Name:&str){
    let p1L = draw_a_card(*p1Cards);
    let p2L = draw_a_card(*p2Cards);
    let p1Loc = p1L as usize;
    let p2Loc = p2L as usize;
    let p1V = deckP1[p1Loc];
    let p2V = deckP2[p2Loc];
    if deckP1[p1Loc] > deckP2[p2Loc]{
        deckP1[*p1Cards as usize] = deckP2[p2Loc];
        *p1Cards +=1;
        deckP2[p2Loc] = deckP2[(*p2Cards - 1) as usize];
        *p2Cards -= 1;
        afficher_tour(p1V, p2V, p1Name, p2Name, *p2Cards)
    }
    else if deckP1[p1Loc] < deckP2[p2Loc]{
        deckP2[*p2Cards as usize] = deckP1[p1Loc];
        *p2Cards +=1;
        deckP1[p1Loc] = deckP1[(*p1Cards - 1) as usize];
        *p1Cards -= 1;
        afficher_tour(p2V, p1V, p2Name, p1Name, *p1Cards)
    }
    else{
        deckP1[p1Loc] = deckP1[(*p1Cards - 1) as usize];
        *p1Cards -= 1;
        deckP2[p2Loc] = deckP2[(*p2Cards - 1) as usize];
        *p2Cards -= 1;
        afficher_tour(p1V, p2V, p1Name, p2Name, 255)
    }

}

fn afficher_tour(pW:u8,pL:u8,pWName:&str,pLName:&str, lCards:u8){
    let pWin = cartes(pW);
    let pLos = cartes(pL);
    if lCards == 255 {
        println!("Egalité! {} et {} perdent leur {}", pWin, pWName, pLName);
    }
    else{
        println!("{2} bat la carte {1} de {3} avec son {0}!\n{3} n'a plus que {4} cartes dans sa pioche!", pWin, pLos, pWName, pLName, lCards);
    }

}

fn cartes(c:u8) -> &'static str{
    match c{
        0 => "as",
        1 => "deux",
        2 => "trois",
        3 => "quatre",
        4 => "cinq",
        5 => "six",
        6 => "sept",
        7 => "huit",
        8 => "neuf",
        9 => "dix",
        10 => "valet",
        11 => "dame",
        12 => "roi",
        _ => "?"
    }
}

pub fn bataille_with_stack(){
    let mut buffer = String::new();
    println!("enter player1 name:");
    io::stdin().read_line(&mut buffer);
    let p1Name = String::from(buffer.trim());
    buffer.clear();
    println!("enter player2 name:");
    io::stdin().read_line(&mut buffer);
    let p2Name = String::from(buffer.trim());
    buffer.clear();
    println!("enter number of cards between 0 and 9999");
    io::stdin().read_line(&mut buffer);
    let re = Regex::new("^[0-9]{1,2}$").unwrap();
    while !re.is_match(buffer.trim()){
        println!("error not a number between 0 and 99");
        buffer.clear();
        io::stdin().read_line(&mut buffer);
    }
    let nbCards = buffer.trim().parse::<u8>().unwrap();
    let p1Deck = init_with_stack(nbCards);
    let p1Cards = nbCards as u32;
    let p2Deck = init_with_stack(nbCards);
    let p2Cards = nbCards as u32;
    let mut p1 = Player::init(p1Name,p1Deck,0,p1Cards);
    let mut p2= Player::init(p2Name,p2Deck,0,p2Cards);
    let battleStack = [14;200];
    let mut stack = Player::init(String::from("stack"),battleStack,0,0);
    let now = SystemTime::now();
    let mut round = 0;
    while p1.len()  > 0 && p2.len() > 0{
        transfer_a_card_with_stack(&mut p1, &mut p2, &mut stack);
        round += 1;
    }
    if p1.len() == p2.len() {
        println!("Egalité!");
    }
    else{
        println!("{} gagne!", if p1Cards == 0{p2.name} else{p1.name});
    }
    match now.elapsed(){
        Ok(elapsed) => println!("Total duration: {:010} ns.\nNombre de manche total: {}", elapsed.as_nanos(), round),
        Err(_e) => println!("Error")
    }
}

fn transfer_a_card_with_stack( p1:&mut Player, p2:&mut Player, stack:&mut Player ){
    let p1V = p1.draw_a_card();
    let p2V = p2.draw_a_card();
    if p1V > p2V{
        p1.put_under_pile();
        p1.get_a_card(p2.draw_a_card());
        p2.lose_a_card();
        if stack.len() > 0 {
            let end = stack.len();
            for _ in 0..end{
                p1.get_a_card(stack.draw_a_card());
                stack.lose_a_card();
            }
        }

        afficher_tour_with_stack(p1V, p2V, p1, p2, stack);
    }
    else if p1V < p2V{
        p2.put_under_pile();
        p2.get_a_card(p1.draw_a_card());
        p1.lose_a_card();
        if stack.len() > 0 {
            let end = stack.len();
            for _ in 0..end{
                p2.get_a_card(stack.draw_a_card());
                stack.lose_a_card();
            }
        }

        afficher_tour_with_stack(p2V, p1V, p2, p1, stack );
    }
    else{
        if p1.len() < 2{
            p1.end = p1.start;
        }
        else if p2.len() < 2{
            p2.end = p2.start;
        }
        else{
            for _ in 0..2{
                stack.get_a_card(p1.draw_a_card());
                p1.lose_a_card();
                stack.get_a_card(p2.draw_a_card());
                p2.lose_a_card();
            }
        }
        afficher_tour_with_stack(p1V, p2V, p1, p2, stack);
    }

}

struct Player{
    name:String,
    deck:[u8;200],
    start:u32,
    end:u32,
}
impl Player{
    fn init(p_name:String,p_deck:[u8;200],p_start:u32,p_end:u32) -> Player{
        Player{
            name:p_name,
            deck:p_deck,
            start:p_start,
            end:p_end
        }
    }
    fn draw_a_card(&self) -> u8{
        return self.deck[(self.start % self.deck.len() as u32) as usize];
    }
    fn lose_a_card(&mut self){
        self.start += 1;
    }
    fn get_a_card(&mut self, card:u8){
        self.deck[(self.end % self.deck.len() as u32) as usize] = card;
        self.end += 1;
    }
    fn len(&mut self) -> u32{
        self.end - self.start
    }
    fn put_under_pile(&mut self) {
        self.deck[(self.end % self.deck.len() as u32) as usize] = self.deck[(self.start % self.deck.len() as u32) as usize];
        self.end += 1;
        self.start += 1;
    }
}

fn afficher_tour_with_stack(win_value:u8,lose_value:u8, win_player:&mut Player, lose_player:&mut Player, stack:&mut Player){
    let p_win = cartes(win_value);
    let p_los = cartes(lose_value);
    if p_win == p_los {
        println!("Egalité! {0} et {1} mettent leur {2} dans le stack.\n\
        {0} a {3} cartes\n{1} a {4} cartes\nle stack fait maintenant {5} cartes"
                 , win_player.name, lose_player.name, p_win,
                 (win_player.end - win_player.start), (lose_player.end - lose_player.start),
                 (stack.end - stack.start));
    }
    else{
        println!("{2} bat la carte {1} de {3} avec son {0}!\
        \n{3} n'a plus que {4} cartes dans sa pioche!",
                 p_win, p_los, win_player.name, lose_player.name,
                 (lose_player.end - lose_player.start));
    }

}
fn init_with_stack(nb:u8) -> [u8;200] {
    let mut deck = [14;200];
    let mut draw:Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9,10,11,12];
    for i in 0..nb {
        let cards = thread_rng().gen_range(0..draw.len());
        deck[i as usize] = draw[cards];
        draw.remove(cards);
        if draw.len() == 0 {
            draw = vec![0,1,2,3,4,5,6,7,8,9,10,11,12];
        }
    }
    deck
}
