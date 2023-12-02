use fancy_regex::Regex;

fn convert_digit(input: &str) -> u32 {
   if input.len() == 1 && input.as_bytes()[0].is_ascii_digit() {
      return input.chars().next().unwrap().to_digit(10).unwrap();
   }; 
   match input {

      "one" => 1,
      "two" => 2,
      "three" => 3,
      "four" => 4,
      "five" => 5,
      "six" => 6,
      "seven" => 7,
      "eight" => 8,
      "nine" => 9,
      c => panic!("could not convert digit {}", c),
   }
}

fn sum_line(input: &str) -> u32 {
   let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|ten|[0-9]))").unwrap();
   let mut digits = vec![];
   for group in re.captures_iter(input).filter_map(|c| c.ok()?.get(1)) {
      let digit = convert_digit(group.as_str());
      digits.push(digit);
   };
   match digits.len() {
      0 => 0,
      1 => digits[0]*10 + digits[0],
      len => digits[0]*10 + digits[len-1],
    }
}

fn sum_lines(input: &str) -> u32 {
   input.lines()
      .map(sum_line)
      .sum()
}

fn main() {
   let input_file = std::fs::read_to_string("./input.txt").expect("could not read input.txt in current directory"); 
   println!("{}", sum_lines(&input_file));
}
