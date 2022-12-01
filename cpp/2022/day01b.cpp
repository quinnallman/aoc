#include <iostream>
#include <vector>

using namespace std;

int main() {
  string line;
  int sum = 0;
  int max1 = -1, max2 = -1, max3 = -1;

  getline(cin, line);
  while(true) {
    if(line.length() == 0) {
      if(sum > max1) {
        max3 = max2;
        max2 = max1;
        max1 = sum;
      } else if(sum > max2) {
        max3 = max2;
        max2 = sum;
      } else if(sum > max3) {
        max3 = sum;
      }

      sum = 0;
    } else {
      sum += stoi(line);
    }

    if(cin.eof()) break;
    getline(cin, line);
  }

  cout << max1 + max2 + max3 << endl;

  return 0;
}