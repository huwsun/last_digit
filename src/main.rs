fn power_mod(a: u64, b: u64, p: u64) -> u64 {
    let mut t = a;
    let mut b = b;
    let mut ret = 1;
    while b != 0 {
        if (b & 1) == 1 {
            ret = ret * t % p;
        }
        t = t * t % p;
        b >>= 1;
    }
    if p == 1 {
        ret = 0;
    }
    ret
}

fn last_digit_a(v: &[u64], pmod: u64) -> u64 {
    println!("vec:{:?},pmod:{}", v, pmod);
    if v.len() == 0 {
        return 1;
    } else if v.len() == 1 {
        return v[0] % pmod;
    } else if v.len() == 2 {
        return power_mod(v[0], v[1], pmod);
    }

    let x = v[0];

    let y = x % pmod;

    if y == 1 {
        return y;
    }

    println!("vec:{:?},y is {}", v, y);

    let mut d = y;
    let mut seg = 1;
    let mut ds = vec![];
    if y == 0 {
        ds.push(1);
    } else {
        ds.push(y);
    }
    for i in 0.. {
        d = (d * y) % pmod;

        if d == 0 {
            if v[1] > seg && v[2] > 0 {
                return 0;
            }
            ds.insert(0, 0);
            //if y==0 {
            seg += 1;
            //}
            break;
        } else if d == y {
            let l = ds.pop().unwrap();
            ds.insert(0, l);
            break;
        }

        ds.push(d);
        seg += 1;
    }
    println!("vec:{:?},seg is {},ds vec is {:?}", v, seg, ds);
    let idx;

    if v[1] == 1 && seg == 1 {
        idx = 1;
    } else {
        idx = last_digit_a(&v[1..], seg);
    }
    println!("vec:{:?},idx is {},ds vec is {:?}", v, idx, ds);

    if idx == 0 && v[1] == 0 && v[0] != 0 {
        1
    } else if idx == 0 && v[0] == 0 {
        1
    } else if idx == 1 && v[0] == 0 {
        0
    } else {
        ds[idx as usize]
    }
}

//my init version
fn last_digit_1(lst: &[u64]) -> u64 {
    last_digit_a(lst, 10)
}

fn mod_off(num: i32, mod_num: i32) -> i32 {
    let tmp = (num - 2) % mod_num + 2;

    if num > tmp {
        return tmp;
    } else {
        return num;
    }
}

fn pow_mod(exp: i32, base: i32) -> i32 {
    (mod_off(base, 20) as f64).powi(mod_off(exp, 4)) as i32
}


//cleverest version
fn last_digit(lst: &[u64]) -> u64 {
    lst.iter().map(|x| *x as i32).rev().fold(1, pow_mod) as u64 % 10
}

//simplese version
fn last_digiti_2(lst: &[u64]) -> u64 {
    if lst.is_empty() {
        1
    } else {
        let mut exp = 1;
        for i in (1..lst.len()).rev() {
            if exp == 0 {
                exp = 1;
            } else {
                exp = match lst[i] % 4 {
                    0 => if lst[i] == 0 { 0 } else { 4 },
                    1 => if lst[i] == 1 { 1 } else { 5 },
                    2 => if exp == 1 { 2 } else { 4 },
                    3 => if exp % 2 == 0 { 5 } else { 3 },
                    _ => panic!("error")
                }
            }
        }
        (lst[0] % 10).pow(exp as u32) % 10
    }
}

fn main() {
    let v = vec![2, 2, 2, 0];

    println!("power mod:{}", power_mod(2, 0, 1));
    println!("mod off:{:?}", vec![0,1,3,9,27,81,243,729].iter().map(|x|mod_off(*x, 10)).collect::<Vec<_>>());
}

#[test]
fn basic_tests() {
    let tests = vec![
        (vec![], 1),
        (vec![0, 0], 1),
        (vec![0, 0, 0], 0),
        (vec![1, 2], 1),
        (vec![3, 4, 5], 1),
        (vec![4, 3, 6], 4),
        (vec![7, 6, 21], 1),
        (vec![12, 30, 21], 6),
        (vec![2, 2, 2, 0], 4),
        (vec![937640, 767456, 981242], 0),
        (vec![123232, 694022, 140249], 6),
        (vec![499942, 898102, 846073], 6),
    ];

    for test in tests {
        assert_eq!(last_digit(&test.0), test.1);
    }
}

#[test]
fn test1() {
    assert_eq!(last_digit(&vec![2, 2, 101, 2]), 6);
}

#[test]
fn test2() {
    assert_eq!(last_digit(&vec![2, 2, 101, 2]), 6);
}

#[test]
fn test3() {
    assert_eq!(last_digit(&vec![2, 0, 1]), 1);
}

#[test]
fn test4() {
    assert_eq!(last_digit(&vec![0, 0, 1, 0, 1, 0, 0, 0, 0, 0]), 1);
}

#[test]
fn test5() {
    assert_eq!(last_digit(&vec![0, 1, 1, 0, 0, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test6() {
    assert_eq!(last_digit(&vec![2, 2, 1, 0, 2, 0, 2, 2, 1, 1]), 4);
}
