#include <iostream>
#include <vector>
#include <algorithm>

const std::vector<std::string> numbers {
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
  "seven",
  "eight",
  "nine",
};

std::string find_first_number(std::string s, bool digitsOnly, bool reversed) {
  for(size_t i = 0 ; i < s.length() ; i++) {
    if(s[i] >= '0' && s[i] <= '9') {
      return std::string { s[i] };
    } else if(!digitsOnly) {
      for(size_t j = 0 ; j < numbers.size() ; j++) {
        std::string num(numbers[j]);
        if(reversed) std::reverse(num.begin(), num.end());

        if(s.substr(i, numbers[j].length()) == num) {
          return std::string{static_cast<char>('0' + j + 1)};
        }
      }
    }
  }

  return "";
}

int main() {
  int sum = 0;
  std::string line;

  while(getline(std::cin, line)) {
    std::string first = find_first_number(line, false, false);
    std::string copy(line);
    std::reverse(copy.begin(), copy.end());
    std::string last = find_first_number(copy, false, true);

    std::cout << first << ' ' << last << ' ' << std::stoi("" + first + last) << '\n';
    sum += std::stoi("" + first + last);
  }

  std::cout << sum << '\n';

  return 0;
}