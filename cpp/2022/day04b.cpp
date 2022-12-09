#include <iostream>

using namespace std;

int main() {
  string line;
  int sum = 0;
  while(getline(cin, line)) {
    string section1 = line.substr(0, line.find(','));
    string section2 = line.substr(line.find(',') + 1);
    auto first = make_pair(stoi(section1.substr(0, section1.find('-'))), stoi(section1.substr(section1.find('-') + 1)));
    auto second = make_pair(stoi(section2.substr(0, section2.find('-'))), stoi(section2.substr(section2.find('-') + 1)));
    for(int i = first.first ; i <= first.second ; i++) {
      if(i >= second.first && i <= second.second) {
        sum++;
        break;
      }
    }
  }

  cout << sum << endl;

  return 0;
}