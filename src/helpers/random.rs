use rand::{thread_rng, Rng};

pub fn get_random_integers(max_value: i64) -> [i64; 3] {
    fn check_for_duplicate(random_numbers: [i64; 3]) -> bool {
        let mut duplicated = false;
        for i in 0..3 {
            for j in 0..3 {
                let same_index = i == j;
                let value_is_set = random_numbers[i] != 0;
                let duplicate_found = random_numbers[i] == random_numbers[j];
                if duplicate_found && !same_index && value_is_set {
                    duplicated = true;
                    break;
                }
            }
        }
        duplicated
    }

    let mut rng = thread_rng();
    let mut random_numbers = [0i64; 3];
    for i in 0..3 {
        random_numbers[i] = rng.gen_range(1..max_value + 1);
        let mut duplicated = check_for_duplicate(random_numbers);
        while duplicated {
            random_numbers[i] = if random_numbers[i] < max_value {
                random_numbers[i] + 1
            } else {
                1
            };
            duplicated = check_for_duplicate(random_numbers);
        }
    }
    random_numbers
}
