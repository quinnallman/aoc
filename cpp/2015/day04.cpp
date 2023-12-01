#include <iostream>
#include <algorithm>
#include <ranges>

#include "../libs/md5.h"

int main() {
  std::string secret;
  getline(std::cin, secret);

  for(int i = 1; ; i++) {
    std::string input = secret + std::to_string(i);
    MD5 result = MD5(input);
    std::string str = result.hexdigest();
    auto all_zero = std::ranges::all_of(str | std::ranges::views::take(5), [](const auto &c) { return c == '0'; });

    if(all_zero) {
      std::cout << i << '\n';
      break;
    }
  }

  return 0;
}