#include <iostream>
#include <vector>
#include <boost/algorithm/string.hpp>
#include <unordered_map>
#include <numeric>

int get_matches(const std::vector<std::string>& game_numbers, const std::vector<std::string>& my_numbers) {
  int matches = std::ranges::count_if(my_numbers, [&game_numbers](std::string num) { return num != "" && std::find(game_numbers.begin(), game_numbers.end(), num) != game_numbers.end(); });
  return matches;
}

int main() {
  std::string line;
  std::unordered_map<int, int> counts;

  while(getline(std::cin, line)) {
    int colon = line.find(':');
    int game_number = std::stoi(line.substr(5, colon));
    counts[game_number]++;

    int pipe = line.find('|');
    std::string game_numbers_str = line.substr(colon + 2, pipe - (colon + 2));
    boost::trim(game_numbers_str);
    std::string my_numbers_str = line.substr(pipe + 2, std::string::npos);
    boost::trim(my_numbers_str);
    std::vector<std::string> game_numbers;
    std::vector<std::string> my_numbers;

    split(game_numbers, game_numbers_str, boost::is_any_of(" "));
    split(my_numbers, my_numbers_str, boost::is_any_of(" "));

    int matches = get_matches(game_numbers, my_numbers);

    for(int i = 1 ; i < matches + 1 ; i++) {
      counts[game_number + i] += counts[game_number];
    }
  }

  int num_cards = 0;
  for(const auto& pair : counts) {
    num_cards += pair.second;
  }

  std::cout << num_cards << '\n';
  return 0;
}