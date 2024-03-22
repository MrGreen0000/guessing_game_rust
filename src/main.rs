use rand::Rng;
use std::io;
use std::cmp::Ordering;
use std::collections::HashMap;


fn main() {
    
   let mut rng = rand::thread_rng();
   let secret_number = rng.gen_range(1..=100);

   let mut guesses: Vec<u32> = Vec::new();
   let mut remaining_guesses = 10;

   let mut scores: HashMap<String, u32> = HashMap::new();

   loop {
       
       // Consignes
       println!("Devinez le nombre restant (entre 1 et 100 inclus) !");
       println!("Entrez un nombre");

       if !guesses.is_empty() {
         println!("Tentative précédente : {:?}", guesses);
       }

       let mut  guess = String::new();
       io::stdin().read_line(&mut guess).expect("Echec de la lecture");

       let guess: u32 = match guess.trim().parse() {
         Ok(num) => num,
         Err(_) => {
            println!("Veuillez entrez un nombre valide ");
            continue;
         }
       };

       guesses.push(guess);

       match guess.cmp(&secret_number) {
            Ordering::Equal => {
               println!("Bravo, vous avez deviné le nombre !");
               break;
            },
            Ordering::Greater => {
               println!("Trop grand !");
               if remaining_guesses > 0 {
                  remaining_guesses -= 1;
               }
            },
            Ordering::Less => {
               println!("Trop petit !");
               if remaining_guesses > 0 {
                  remaining_guesses -= 1;
               }
            }
       }
   }

   // Gestion du score final

   let final_score = remaining_guesses * 10;
   println!("Score final est de {}", final_score);

   // Demander au joueur son nom
   println!("Veullez entrer votre nom : ");
   let mut player_name = String::new();
   io::stdin().read_line(&mut player_name).expect("Echec de la lecture");
   let  player_name = player_name.trim().to_string();


   // Enregistrement dans la Hashmap
   scores.entry(player_name).or_insert(final_score);
   println!("Meilleurs scores : {:?}", scores);
   



}
