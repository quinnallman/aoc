#include <iostream>

using namespace std;

int shape_score(char shape) {
  if(shape == 'A' || shape == 'X') return 1;
  if(shape == 'B' || shape == 'Y') return 2;
  if(shape == 'C' || shape == 'Z') return 3;
  return 0;
}

int round_score(char theirs, char ours) {
  if(theirs == 'A') return ours == 'X' ? 3 : (ours == 'Z' ? 0 : 6);
  if(theirs == 'B') return ours == 'Y' ? 3 : (ours == 'X' ? 0 : 6);
  if(theirs == 'C') return ours == 'Z' ? 3 : (ours == 'Y' ? 0 : 6);
  return 3;
}

int main() {
  char theirs, ours;
  int score = 0;

  cin >> theirs >> ours;
  while(!cin.eof()) {
    score += shape_score(ours) + round_score(theirs, ours);
    cin >> theirs >> ours;
  }

  cout << score << endl;

  return 0;
}