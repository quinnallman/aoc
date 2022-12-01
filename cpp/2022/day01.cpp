#include <iostream>

using namespace std;

int main() {
  string line;
  int sum = 0;
  int max = -1;
  while(getline(cin, line)) {
    if(line.length() == 0) {
      if(sum > max) max = sum;
      sum = 0;
    } else {
      sum += stoi(line);
    }
  }

  cout << max << endl;

  return 0;
}