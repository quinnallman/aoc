#include <iostream>
#include <ranges>
#include <algorithm>

int main() {
  std::string str;
  int count = 0;

  while(getline(std::cin, str)) {
    auto is_vowel = [] (const char c) -> bool { return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'; };
    const size_t num_vowels = std::ranges::count_if(str, is_vowel);

    if (num_vowels < 3)
      continue;

    bool found = false;
    bool bad = false;
    for(size_t i = 1 ; i < str.length() ; i++) {
      if (str[i] == str[i-1]) {
        found = true;
        continue;
      } else if((str[i-1] == 'a' && str[i] == 'b') ||
                (str[i-1] == 'c' && str[i] == 'd') ||
                (str[i-1] == 'p' && str[i] == 'q') ||
                (str[i-1] == 'x' && str[i] == 'y')) {
        bad = true;
        break;
      }
    }

    if (!found || bad)
      continue;

    count++;
  }

  std::cout << count << '\n';
}