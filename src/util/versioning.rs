pub struct Version {
    //really shouldn't go above 9, but is valid <256
    pub major_revision: u8,
    pub minor_revision: u8,
    pub patch_revision: u8,
}
impl Version {
    pub fn as_string(&self) -> String {
        let x: String = self.major_revision.to_string();
        let y: String = self.minor_revision.to_string();
        let z: String = self.patch_revision.to_string();
        return x.to_string() + "." + y.as_str() + "." + z.as_str();
    }
    pub fn as_u32(&self) -> u32 {
        let mut accumulator: u32 = 0;
        accumulator += self.major_revision as u32 * 1000000;
        accumulator += self.minor_revision as u32 * 1000;
        accumulator += self.patch_revision as u32;
        return accumulator;
    }
}
impl From<String> for Version {
    fn from(version: String) -> Self {
        let mut iter = version.split(".").map(|s| s.parse::<u8>().unwrap());
        Version {
            patch_revision: iter.next().unwrap(),
            minor_revision: iter.next().unwrap(),
            major_revision: iter.next().unwrap(),
        }
    }
}
impl From<u32> for Version {
    fn from(version: u32) -> Self {
        let mut sliced: Vec<u32> = digit_deconstructor(version);
        Version {
            patch_revision: digit_constructor(sliced[0..3].to_vec()) as u8,
            minor_revision: digit_constructor(sliced[3..6].to_vec()) as u8,
            major_revision: digit_constructor(sliced[6..9].to_vec()) as u8,
        }
    }
}

//helper functions
fn digit_deconstructor(mut num: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    loop {
        digits.push(num % 10);
        num /= 10;
        if num <= 0 {
            break;
        }
    }
    return digits;
}
fn digit_constructor(mut digits: Vec<u32>) -> u32 {
    let ten: u32 = 10;
    let mut sum = 0;
    loop {
        sum += digits.pop().unwrap() * ten.pow(digits.len() as u32);
        if digits.is_empty() {
            break;
        }
    }
    return sum;
}
