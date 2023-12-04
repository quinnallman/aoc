#include <iostream>
#include <vector>
#include <boost/algorithm/string.hpp>

int score_game(const std::vector<std::string>& game_numbers, const std::vector<std::string>& my_numbers) {
  int score = 1;

  int matches = std::ranges::count_if(my_numbers, [&game_numbers](std::string num) { return num != "" && std::find(game_numbers.begin(), game_numbers.end(), num) != game_numbers.end(); });

  if(matches < 2) return matches;

  while(matches-- > 1) {
    score *= 2;
  }

  return score;
}

int main() {
  std::string line;
  int score = 0;

  while(getline(std::cin, line)) {
    int colon = line.find(':');
    int pipe = line.find('|');
    std::string game_numbers_str = line.substr(colon + 2, pipe - (colon + 2));
    boost::trim(game_numbers_str);
    std::string my_numbers_str = line.substr(pipe + 2, std::string::npos);
    boost::trim(my_numbers_str);
    std::vector<std::string> game_numbers;
    std::vector<std::string> my_numbers;

    split(game_numbers, game_numbers_str, boost::is_any_of(" "));
    split(my_numbers, my_numbers_str, boost::is_any_of(" "));

    int game_score = score_game(game_numbers, my_numbers);
    score += game_score;
  }

  std::cout << score << '\n';
  return 0;
}