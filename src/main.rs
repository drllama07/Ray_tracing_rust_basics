use std::io::*;


use crate::online::*;

pub mod online;


fn user_input(user: &str) -> (usize, usize) {
        if user == "you" {
            println!("Type the location(0(initial) to 2(last) for the both numbers): ");
            let mut inputx = String::new();
            stdin().read_line(&mut inputx).unwrap();
            let my_x: usize = inputx.trim().parse().expect("parsing error");
    
            let mut inputy = String::new();
            stdin().read_line(&mut inputy).unwrap();
            let my_y: usize = inputy.trim().parse().expect("parsing error");
            let _ = client(&my_x,&my_y);
            (my_x, my_y)
        } else if user == "it" {
            println!("Waiting for the other user <- ");

            let (it_x,it_y) = server();

            (it_x,it_y)
        } else {
            (10,10)
        }
}

fn graphics(game:&[[&str; 3];3 ]) {
    for i  in 0..game.len(){
        let mut a = String::new();
        a.push_str("------> |");
        for j  in 0..game[0].len() {
            a.push_str(game[i][j]);
            a.push_str("|");
        }
        println!("{a}")

    }
    println!("");
    println!("________-________");
    println!("");
}









fn main() {
    let mut turn: &str = "you";
    let mut player: &str = "X";
    let mut input = String::new();
    println!("Are you Player_X (1) or Player_O (2)/type 1 or 2:");
    println!("!!! Make sure Player_O is online before Player_1 for technical reasons !!! ");
    stdout().flush().unwrap(); 
    stdin().read_line(&mut input).unwrap();
    let number: usize = input.trim().parse().expect("parsing error");
    if number == 2 {
       turn = "it";
    }

    let mut game: [[&str; 3]; 3] = [["_","_","_"],["_","_","_"],["_","_","_"]];
    let mut i = 0;
    let mut win = "NO_WINNER";
    while i < 9 {
            graphics(&game);
            let (row,col) = user_input(&turn);
            if game[row][col] == "_" {
                if player== "X" {
                   game[row][col] = "X";
                   player = "O";
                   if turn == "you" {
                    turn = "it"
                   } else {
                    turn = "you"
                   }
                }
                else {
                    game[row][col] = "O";
                    player = "X";
                    if turn == "you" {
                        turn = "it"
                    } else {
                        turn = "you"
                    }
                }
            }
            else {
               println!("This is invalid move, {row} and {col} is occupied");
            }
            win = winner_gets_the_dinner(&game);
            if win == "NO_WINNER" {
               i += 1;
               continue
            }
            else{
               graphics(&game);
               println!("This match's winner is Player_{win}");
               break
            }
            
    }
    if win == "NO_WINNER" {
        println!("This match has $NO_WINNERS, its a t-i-e :( ");
    }
}





// game logics checking if there is a winner or not 
fn winner_gets_the_dinner<'a>(game: &'a [[&str;3];3]) -> &'a str {
       if game[0][0] == game[0][1] && game[0][0] == game[0][2] && game[0][0] != "_"{
            game[0][0]
       }
       else if  game[0][0] != "_" && game[0][0] == game[1][0] && game[0][0] == game[3][1]{
           game[0][0]
       }
       else if  game[1][1] != "_" && game[1][1] == game[1][0] && game[1][1] == game[1][2] && game[1][1] != "_" {
           game[1][1]  
       } else if game[1][1] != "_" && game[1][1] == game[0][1] && game[1][1] == game[2][1] && game[1][1] != "_" {
           game[1][1]
       } else if game[1][1] != "_" && game[1][1] == game[2][2] && game[1][1] == game[0][0] && game[1][1] != "_" {
          game[1][1]
       } else if game[1][1] != "_" && game[1][1] == game[2][0] && game[1][1] == game[0][2] && game[1][1] != "_" {
          game[1][1]
       } else if game[2][2] != "_" && game[2][2] == game[2][0] && game[2][2] == game[2][1] && game[2][2] != "_" {
         game[2][2]
       } else if game[2][2] != "_" && game[2][2] == game[0][2] && game[2][2] == game[1][2] {
         game[2][2]
       } else {
         "NO_WINNER"
       }
}