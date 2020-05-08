const BANNED: [char; 3] = ['i', 'o', 'l'];

pub fn next_password(password: &str) -> String {
    let password = increment(password.to_string());
    let mut res = remove_banned_letter(password);
    while !valid_password(&res) {
        res = increment(res);
    }
    res
}

fn remove_banned_letter(mut password: String) -> String {
    let mut ban = false;
    unsafe {
        password.as_bytes_mut().iter_mut().for_each(|c| {
            if ban {
                *c = b'a';
            } else if BANNED.contains(&(*c as char)) {
                *c += 1;
                ban = true;
            }
        })
    };
    password
}

fn increment(mut password: String) -> String {
    let rev = unsafe { password.as_bytes_mut().iter_mut().rev() };
    for c in rev {
        if *c == b'z' {
            *c = b'a';
        } else {
            *c += 1;
            if BANNED.contains(&(*c as char)) {
                *c += 1;
            }
            break;
        }
    }
    password
}

fn valid_password(password: &str) -> bool {
    no_banned_letter(password) && increasing(password) && two_pair(password)
}

fn no_banned_letter(password: &str) -> bool {
    !password.chars().any(|c| BANNED.contains(&c))
}

fn increasing(password: &str) -> bool {
    let password: Vec<u8> = password.bytes().collect();
    password
        .windows(3)
        .any(|v| v[0] + 1 == v[1] && v[1] + 1 == v[2])
}

fn two_pair(password: &str) -> bool {
    let password: Vec<char> = password.chars().collect();
    let iter = password.windows(2);

    let mut first = None;

    for pair in iter {
        if pair[0] == pair[1] {
            if first == Some(pair[0]) {
                continue;
            } else if first == None {
                first = Some(pair[0]);
            } else {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_banned_letter() {
        assert_eq!(no_banned_letter("abcdefgh"), true);
        assert_eq!(no_banned_letter("abcdefghi"), false);
        assert_eq!(no_banned_letter("abcdefghjk"), true);
        assert_eq!(no_banned_letter("abcdefghjkl"), false);
    }

    #[test]
    fn test_increasing() {
        assert_eq!(increasing("abc"), true);
        assert_eq!(increasing("adcbefkhi"), false);
        assert_eq!(increasing("tesiran efg etdlau"), true);
    }

    #[test]
    fn test_two_pair() {
        assert_eq!(two_pair("aabcc"), true);
        assert_eq!(two_pair("aaa"), false);
        assert_eq!(two_pair("aabaa"), false); // I’m not sure about this one
        assert_eq!(two_pair("tesiiranetddlau"), true);
    }

    #[test]
    fn test_increment() {
        assert_eq!(&increment("aabcc".to_string()), "aabcd");
    }

    #[test]
    fn test_next_password() {
        assert_eq!(&next_password("abcdefgh"), "abcdffaa");
        assert_eq!(&next_password("ghijklmn"), "ghjaabcc");
    }
}
