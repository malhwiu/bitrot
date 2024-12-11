use rand::{random, thread_rng, Rng};

pub enum CorruptRatio {
    //Precent(f32),
    Amount(usize)
}

pub fn corrupt(start: usize, end: usize, ratio: &CorruptRatio, array: &mut Vec<u8>) -> bool {
    if array.len() < start { return false; }
    if array.len() < end { return false; }
    if end < start { return false; }

    let amount = match ratio {
        CorruptRatio::Amount(a) => {*a},
        //_ => { return false;}
    };

    let total = end - start;
    if amount > total {
        return false;
    }

    for _i in 0..amount {
        let place: usize = thread_rng().gen_range(start..end);
        let value = array[place];
        array[place] = flip_random_bit(value);
    }
    
    true
}

fn flip_random_bit(int: u8) -> u8 {
    let index: u8 = random::<u8>() % 8;

    int ^ (1 << index)
}