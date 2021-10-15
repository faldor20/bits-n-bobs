use rand::prelude::*;
fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();

    let mut population:Vec<_>=(0..50).map(|a|{
      /*   let allel=
            if a/25<1{0}else{1} */
        Creature{male:rng.gen(), allels:vec![(rng.gen_range(0..2),rng.gen_range(0..2))]}
        }).collect();
    
    for i in 0..1000{
      population=step(population)  ;
    }
}
///Contains a name and an id
type Allel=u16;

struct Creature {
    allels:Vec<(Allel,Allel)>,

    ///not currently used
    male:bool
}
type Gamete=Vec<Allel>;

fn GetGamete(creature:&Creature)->Gamete{
    let out:Vec<_>=creature.allels.iter().map(|(a,b)|(if rand::random() {*a}else {*b})).collect();
    out
}
fn breed(creature1:&Creature,creature2:&Creature,offspringCount:u8)->Vec<Creature>{
    let breeding=||{
        let gam1=GetGamete( &creature1);
        let gam2= GetGamete(&creature2);
        let babyAllels:Vec<_>=gam1.into_iter().zip(gam2.into_iter()).collect();
        Creature { allels: babyAllels,male: rand::random() }

    };

 
    (0..offspringCount).map(|_|(breeding())).collect()
}
fn step(population:Vec<Creature>)->Vec<Creature>{
    let mut rng = rand::thread_rng();

    let nextGen:Vec<_>=population.iter().flat_map(|_|(breed(&population[rng.gen_range(0..population.len())],&population[rng.gen_range(0..population.len())],1))).collect();
    nextGen
//maintain population
//pick two at random
//breed
}