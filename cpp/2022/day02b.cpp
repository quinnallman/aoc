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

char win(char theirs) {
  return theirs == 'A' ? 'Y' : (theirs == 'B' ? 'Z' : 'X');
}

char lose(char theirs) {
  return theirs == 'A' ? 'Z' : (theirs == 'B' ? 'X' : 'Y');
}

char draw(char theirs) {
  return theirs == 'A' ? 'X' : (theirs == 'B' ? 'Y' : 'Z');
}

int main() {
  char theirs, ours, outcome;
  int score = 0;

  cin >> theirs >> outcome;
  while(!cin.eof()) {
    if(outcome == 'X') ours = lose(theirs);
    else if(outcome == 'Y') ours = draw(theirs);
    else ours = win(theirs);

    score += shape_score(ours) + round_score(theirs, ours);

    cin >> theirs >> outcome;
  }

  cout << score << endl;

  return 0;
}