#![allow(dead_code)]

use std::env;


fn kolakoski_increment(buffer: &mut [i8; 256], i: i8) -> i8 {
    // see http://www.emis.ams.org/journals/JIS/VOL15/Nilsson/nilsson5.pdf
    let current = buffer[i as usize];
    if current == 11 || current == 22 || current == 0 {
        buffer[i as usize] = if current == 11 {1} else {2};
        return if current == 1 || current == 11 {1} else {2};
    }
    buffer[i as usize] = if current == 1 || current == 11 {
        if kolakoski_increment(buffer, i + 1) == 1 {2} else {22}
    } else {
        if kolakoski_increment(buffer, i + 1) == 1 {1} else {11}
    };
    return if buffer[i as usize] == 1 || buffer[i as usize] == 11 {1} else {2};
}


fn buffer_string(buffer: &mut [i8; 256]) -> String {
    let mut capacity = 0;
    for i in 0..255 {
        let current = buffer[i as usize];
        if current == 0 {
            break
        } else {
            capacity += 1;
        }
    }
    let mut result = String::with_capacity(capacity);
    for i in 0..capacity {
        let current = buffer[i as usize];
        if current == 1 {
            result.push('1');
        } else if current == 2 {
            result.push('2');
        } else if current == 11 {
            result.push('3');
        } else if current == 22 {
            result.push('4');
        }
    }
    return result
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut length: i64 = 10000000;
    if args.len() >= 2 {
        length = args[1].parse::<i64>().unwrap();
    }

    // the buffer is a constant size and should be large enough for the first ~3E44 digits
    let mut buffer: [i8; 256] = [0; 256];
    let mut ones_and_twos_counts: [i64; 2] = [0, 0];

    let mut current_time =  std::time::SystemTime::now();
    let mut display_update_counter: i64 = 0;
    let mut rate = 0.0;

    // initialize the first two digits in the sequence
    if length >= 1 {
        ones_and_twos_counts[0] += 1;
    }
    if length >= 2 {
        ones_and_twos_counts[1] += 1;
    }
    for count in 2..(length) {
        let next = kolakoski_increment(&mut buffer, 0) - 1;
        ones_and_twos_counts[next as usize] += 1;
        display_update_counter += 1;
        if count % 10000 == 0 {
            let now = std::time::SystemTime::now();
            let nanos = now.duration_since(current_time).unwrap().subsec_nanos();
            if nanos >= 10000000 {
                rate = display_update_counter as f64 / 10000.0;
                current_time = now;
                display_update_counter = 0;
                let message = format!(
                    "\r{}M digits/s, n={} [{} ones vs {} twos] buffer {} ", rate, count + 1,
                    ones_and_twos_counts[0], ones_and_twos_counts[1], buffer_string(&mut buffer)
                );
                print!("{}", message);
            }
        }
    }
    let message = format!("{}M digits/s, n={} [{} ones vs {} twos] buffer {}",
                          rate, length, ones_and_twos_counts[0],
                          ones_and_twos_counts[1], buffer_string(&mut buffer));
    print!("\r{}\n", message);
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_sequence(length: i64, expected_ones: i64, expected_twos: i64) {
        // the buffer is a constant size and should be large enough for the first ~3E44 digits
        let mut buffer: [i8; 256] = [0; 256];
        let mut ones_and_twos_counts: [i64; 2] = [0, 0];

        // initialize the first two digits in the sequence
        if length >= 1 {
            ones_and_twos_counts[0] += 1;
        }
        if length >= 2 {
            ones_and_twos_counts[1] += 1;
        }
        for _ in 2..(length) {
            let next = kolakoski_increment(&mut buffer, 0) - 1;
            ones_and_twos_counts[next as usize] += 1;
        }
        if (ones_and_twos_counts[0], ones_and_twos_counts[1]) != (expected_ones, expected_twos) {
            println!("failed test! n={}, expected {}:{} vs {}:{}", length, expected_ones,
                     expected_twos, ones_and_twos_counts[0], ones_and_twos_counts[1]);
        } else {
            println!("passed test! n={}", length);
        }
    }

    #[test]
    fn test_10() {
        test_sequence(10, 5, 5);
    }

    #[test]
    fn test_100() {
        test_sequence(100, 49, 51);
    }

    #[test]
    fn test_1000() {
        test_sequence(1000, 502, 498);
    }

    #[test]
    fn test_10000() {
        test_sequence(10000, 4996, 5004);
    }

    #[test]
    fn test_100000() {
        test_sequence(100000, 49972, 50028);
    }

    #[test]
    fn test_1000000() {
        test_sequence(1000000, 499986, 500014);
    }
}
