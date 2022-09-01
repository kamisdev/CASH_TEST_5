extern crate rand;
extern crate math;

use rand::{prelude::*, distributions::Alphanumeric};
use std::io;
use math::round;
use std::process;
use std::convert::From;
use std::collections::HashMap;

// function for creating random number
fn generate_price() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen_range(1.00..10.00);
    round::ceil(y, 2)
}

fn generate_name() -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
}

// function for comparing input price and generated random price
// fn compare(price_origin: f64, price_input: f64) -> bool {
//     price_input >= price_origin
// }

fn input_payment() -> (f64, usize) {

    let mut price_input: f64;
    let mut tried_cnt: usize = 0;

    loop {
        println!("Please input value for payment. Or please press 'q' to quit.");
        let mut str_input = String::new();

        // input string
        io::stdin()
            .read_line(&mut str_input)
            .expect("Cannot read line");

        // check if input 'q'
        let pp = str_input.as_bytes();
        if pp[0] == b'q' {
            println!("Do you really want to exit? y/n");

            // confirm quit
            loop {
                str_input = String::from("");
                io::stdin()
                    .read_line(&mut str_input)
                    .expect("Cannot read line");
                
                let p = str_input.to_lowercase();
                let pp = p.as_bytes();
                if pp[0] == b'y' {
                    process::exit(1);
                } else if pp[0] == b'n' {
                    break;
                } else {
                    continue;
                }
            }
            continue;
        }
        
        price_input = match str_input.trim().parse() {
            Ok(pay) => {
                if pay <= 0.0 || pay >= 1000.0 {
                    println!("Invalid input! Must be between 1.00 to 999.99");
                    continue;
                }

                // check format 000.00
                let check_pay = round::ceil(pay, 2);
                if check_pay * 100.0 != pay * 100.0 {
                    
                    println!("Invalid input format! Must be format like : 000.00");
                    continue;
                }
                tried_cnt += 1;
                pay
            },
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        break;
        
        // if compare(price_origin, price_input) {
        //     break;
        // }
        
        // println!("Price is not enough! Tried count : {tried_cnt}");

    }

    (price_input, tried_cnt)
}

fn calculate_coin_amount(price_change: f64, cash_box: &HashMap<usize, i32>, coin_array: &Vec<f64>, cash_limit: &HashMap<usize, i32>) -> (bool, HashMap<usize, i32>) {
    // let coin_array = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];

    // for value in coin_array {
    //     println!("{value}");
    // }

    let mut coin_limit = HashMap::new();
    for (key, value) in cash_limit {
        coin_limit.insert(*key, *value);
    }
    let mut sorted_cash_box: Vec<_> = cash_box.iter().collect();
    sorted_cash_box.sort_by(|a, b| b.0.cmp(a.0));

    let mut price_change = price_change;

    let mut cur_index = 0;

    // let mut result = String::new();

    let mut cash_updated = HashMap::new();
    for (key, value) in cash_box {
        cash_updated.insert(*key, *value);
    }

    if price_change < 0.0 {
        // need to use coins in cash_box
        price_change = round::ceil(0.0 - price_change, 2);

        while price_change > 0.0 {
            println!("{price_change}");

            if cur_index > coin_array.len() - 1 && price_change > 0.0 {
                // initialize cash_updated as cash_box
                for (key, value) in cash_box {
                    let count = cash_updated.entry(*key).or_insert(0);
                    *count = *value;
                }

                return (false, cash_updated);
            }

            if price_change < coin_array[cur_index] - 0.001 {
                if cur_index == coin_array.len() - 1 {
                    break;
                }
                cur_index += 1;
                continue;
            }
    
            let rest_price = (price_change*100.0) as usize;
            let coin_unit = (coin_array[cur_index]*100.0) as usize;
    
            let mut coin_cnt = rest_price / coin_unit;

            let available_cnt = *sorted_cash_box[cur_index].1;
            if available_cnt <= coin_cnt as i32 {
                coin_cnt = available_cnt as usize;                
            }
            let count = cash_updated.entry((coin_array[cur_index] * 100.0) as usize).or_insert(0);
            *count -= coin_cnt as i32;

            // result.insert_str(result.len(), format!("{} coin X {coin_cnt}\n", coin_array[cur_index]).as_str());
            let rest = rest_price - coin_cnt * coin_unit;
            // println!("{}, {}, {}", coin_unit, coin_cnt, rest);
    
            price_change = (rest as f64) / 100.0;
            cur_index += 1;
        }
    }

    else {
        // need to add coins to cash_box
        while price_change > 0.0 {

            if cur_index > coin_array.len() - 1 && price_change > 0.0 {
                // initialize cash_updated as cash_box
                for (key, value) in cash_box {
                    let count = cash_updated.entry(*key).or_insert(0);
                    *count = *value;
                }

                return (false, cash_updated);
            }

            if price_change < coin_array[cur_index] - 0.001 {
                if cur_index == coin_array.len() - 1 {
                    break;
                }
                cur_index += 1;
                continue;
            }
    
            let rest_price = (price_change*100.0) as usize;
            let coin_unit = (coin_array[cur_index]*100.0) as usize;
    
            let mut coin_cnt = rest_price / coin_unit;
            let cur_coin_limit = coin_limit.get(&coin_unit).unwrap();
            // let cur_coin_limit = coin_limit.entry(coin_unit).or_insert(0);

            let available_cnt = cur_coin_limit - sorted_cash_box[cur_index].1;
            if available_cnt <= coin_cnt as i32 {
                coin_cnt = available_cnt as usize;                
            }
            let count = cash_updated.entry((coin_array[cur_index] * 100.0) as usize).or_insert(0);
            *count += coin_cnt as i32;

            // result.insert_str(result.len(), format!("{} coin X {coin_cnt}\n", coin_array[cur_index]).as_str());
            let rest = rest_price - coin_cnt * coin_unit;
            // println!("{}, {}, {}", coin_unit, coin_cnt, rest);
    
            price_change = (rest as f64) / 100.0;
            cur_index += 1;
        }
    }

    return (true, cash_updated);
}

struct Product {
    product_no: usize,
    product_name: String,
    product_price: f64,
}

impl Product {
    fn new(product_no: usize, product_name: String, product_price: f64) -> Product {
        Product {
            product_no,
            product_name,
            product_price,
        }
    }
}

fn generate_product_list(product_cnt: usize) -> Vec<Product> {
    let mut index = 0;

    let mut result: Vec<Product> = Vec::new();

    while index < product_cnt {
        index += 1;
        result.push(
            Product::new(
                index,
                generate_name(),
                generate_price(),
            )
        );
    }

    return result;
}

fn input_product_number(limit: usize) -> usize {

    let mut str_input = String::new();
    let mut product_no: usize = 0;

    loop {
        io::stdin()
            .read_line(&mut str_input)
            .expect("Failed to read line");

        // check if input 'q'
        let pp = str_input.as_bytes();
        if pp[0] == b'q' {
            println!("Do you really want to exit? y/n");

            // confirm quit
            loop {
                str_input = String::from("");
                io::stdin()
                    .read_line(&mut str_input)
                    .expect("Cannot read line");
                
                let p = str_input.to_lowercase();
                let pp = p.as_bytes();
                if pp[0] == b'y' {
                    process::exit(1);
                } else if pp[0] == b'n' {
                    break;
                } else {
                    continue;
                }
            }
            continue;
        }
        
        product_no = match str_input.trim().parse() {
            Ok(num) => {
                if num > limit || num < 1 {
                    println!("{} Error: Input Correct Number", str_input);
                    str_input = String::from("");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("{} Error: Input Correct Number", str_input);
                str_input = String::from("");
                continue;
            }
        };
        break;
    }

    product_no
}

fn input_usable_coin() -> Vec<f64> {
    println!("Please input coin you want to use.");
    let mut str_input = String::new();

    let mut result = Vec::new();

    io::stdin()
        .read_line(&mut str_input)
        .expect("Failed to read line");
    
    let v: Vec<&str> = str_input.split(' ').collect();
    for str_number in v {
        let num: f64 = match str_number.trim().parse() {
            Ok(a) => a,
            Err(_) => continue,
        };
        result.push(num);
    }

    result.sort_by(|a, b| b.total_cmp(a));
    result
}

fn input_add_new_coin() -> (bool, Vec<f64>) {
    println!("Do you want to add new coin? y/n");

    let mut str_input = String::new();
    let mut new_coin_list = Vec::new();

    loop {
        str_input = String::from("");
        io::stdin()
            .read_line(&mut str_input)
            .expect("Cannot read line");
        
        let p = str_input.to_lowercase();
        let pp = p.as_bytes();
        if pp[0] == b'y' {
            break;
        } else if pp[0] == b'n' {
            return (false, new_coin_list);
        } else {
            continue;
        }
    }

    str_input = String::from("");
    io::stdin()
        .read_line(&mut str_input)
        .expect("Failed to read line");
    
    let v: Vec<&str> = str_input.split(' ').collect();
    for str_number in v {
        let num: f64 = match str_number.trim().parse() {
            Ok(a) => a,
            Err(_) => continue,
        };
        new_coin_list.push(num);
    }

    (true, new_coin_list)
}

fn input_set_coin_limit(cash_limit: HashMap<usize, i32>) -> HashMap<usize, i32> {
    println!("Do you want to set new limit of amount of each coin? y/n");
    
    let mut str_input = String::new();
    let mut cash_updated = HashMap::new();

    for (key, value) in &cash_limit {
        cash_updated.entry(*key).or_insert(*value);
    }

    loop {
        str_input = String::from("");
        io::stdin()
            .read_line(&mut str_input)
            .expect("Cannot read line");
        
        let p = str_input.to_lowercase();
        let pp = p.as_bytes();
        if pp[0] == b'y' {
            break;
        } else if pp[0] == b'n' {
            return cash_updated;
        } else {
            continue;
        }
    }

    for (key, value) in &cash_limit {
        println!("{:.2} : ", (*key as f64) / 100.0);
        let mut limit: i32;
        loop {
            str_input = String::from("");
            io::stdin()
                .read_line(&mut str_input)
                .expect("Cannot read line");
            
            limit = match str_input.trim().parse() {
                Ok(v) => {
                    if v < 0 {
                        println!("Input Valid Integer");
                        continue;
                    }
                    v
                },
                Err(_) => {
                    println!("Input Valid Integer");
                    continue;
                }
            };
            break;
        }
        let current_limit = cash_updated.entry(*key).or_insert(*value);
        *current_limit = limit
    }

    cash_updated
}

fn main() {
    let mut cash_limit = HashMap::new();
    let mut cash_box = HashMap::new();
    loop {

        let coin_array = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];
        

        for coin in coin_array {
            let coin = (coin*100.0) as usize;
            cash_box.insert(coin, 20);
            cash_limit.insert(coin, 50);
        }

        for (key, value) in &cash_box {
            println!("{:.2} coin : {}", (*key as f64) / 100.0, value);
        }
        println!("------------------------------------------------------");

        let product_list = generate_product_list(10);

        let (is_added, added_coin_list) = input_add_new_coin();
        if is_added {
            for value in added_coin_list {
                cash_limit.entry((value*100.0) as usize).or_insert(50);
                cash_box.entry((value*100.0) as usize).or_insert(20);
            }
        }
        cash_limit = input_set_coin_limit(cash_limit);

        for product in &product_list {
            println!("{} : {} : {}", product.product_no, product.product_name, product.product_price);
        }
        println!("Please input product number : ");
        let product_no = input_product_number(10);

        let selected_product = product_list.get(product_no - 1).unwrap();
        let price_origin = selected_product.product_price;
        let product_name = String::from(selected_product.product_name.as_str());
        println!("You selected {} : {} : {}", product_no, product_name, price_origin);
        println!("------------------------------------------------------");
    
        let (price_input, tried_cnt) = input_payment();

        let price_change = price_input - price_origin;

        let usable_coin = input_usable_coin();

        println!("Change is {:.2}", price_change);

        let (possibility, result) = calculate_coin_amount(price_change, &cash_box, &usable_coin, &cash_limit);
        println!("{possibility}");

        for (key, value) in result {
            println!("{:.2} coin : {}", (key as f64) / 100.0, value);
        }
        println!("------------------------------------------------------");
        if possibility {
            break;
        }
    }
}