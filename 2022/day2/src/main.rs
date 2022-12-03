use std::{ fs, collections::HashMap };

fn main() {
    
    let strat_guide_encrypted = match fs::read_to_string("input.txt") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("biffed it reading the encrypted strategy guide {:?}", e);
            std::process::exit(1)
        }
    };

    let d = 3; // DRAW
    let l = 0; // LOSE
    let w = 6; // WIN
    let r = 1; // ROCK
    let p = 2; // PAPER
    let s = 3; // SISSORS
    let points = vec![
        // enemy ->
        //   A=r, B=p, C=s
        vec![r+d, r+l, r+w], // me = X (Rock = 1);
        vec![p+w, p+d, p+l], // me = Y (Paper = 2)
        vec![s+l, s+w, s+d], // me = Z (Sissors = 3)
    ];

    // these are the indexes in the points map ...
    let mut decryp_to_pts_idx = HashMap::new();
    decryp_to_pts_idx.insert("A", 0);
    decryp_to_pts_idx.insert("B", 1);
    decryp_to_pts_idx.insert("C", 2);
    decryp_to_pts_idx.insert("X", 0);
    decryp_to_pts_idx.insert("Y", 1);
    decryp_to_pts_idx.insert("Z", 2);

    let mut total_points = 0;
    for line in strat_guide_encrypted.split("\n") {
        let mut plays = line.split(" ");
        let en_play_encrypted = plays.next().unwrap();
        let me_play_encrypted = plays.next().unwrap();

        let en_play = *decryp_to_pts_idx.get(en_play_encrypted).unwrap();
        let me_play = *decryp_to_pts_idx.get(me_play_encrypted).unwrap();
        
        let pts = points[me_play][en_play];
        println!("for enemy={:?} me={:?} i scored {:?}", en_play_encrypted, me_play_encrypted, pts);
        
        total_points += pts;
    }
    
    println!("PART 1 = at the end of the elf RPS tournament I won {:?}", total_points);
    
}
