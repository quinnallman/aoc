#include <iostream>
#include <ranges>

int main() {
  int sum = 0;
  std::string line;

  while(getline(std::cin, line)) {
    auto res = line
      | std::views::filter([](const auto& ch) { return ch >= '0' && ch <= '9'; });
    auto first = res.front();
    auto last = res.back();
    std::string num = { first, last };
    sum += std::stoi(num);
  }

  std::cout << sum << '\n';

  return 0;
}