#include <iostream>
#include <deque>
#include <vector>

using namespace std;

const int N = 9;

void print_stacks(deque<char> stacks[N]) {
  for(int i = 0 ; i < N ; i++) {
    cout << "Stack " << i << ": ";

    for(int j = 0 ; j < stacks[i].size() ; j++) {
      cout << stacks[i][j] << " ";
    }
    cout << endl;
  }
}

int main() {
  char ch;
  char crate;
  deque<char> stacks[N];
  string line;

  while(true) {
    getline(cin, line);
    if(line.length() > 1 && line[1] == '1') break;
    for(int i = 1 ; i < line.length() ; i+=4) {
      if(line[i] != ' ') {
        crate = line[i];
        stacks[i/4].push_front(crate);
      }
    }
  }

  //print_stacks(stacks);

  getline(cin, line);

  while(getline(cin, line)) {
    //cout << line << " (" << line.length() << ")" << endl;
    //cout << line[5] << " " << line[12] << " " << line[17] << endl;
    int n = stoi(line.substr(5, line.find(" from") - 5));
    int source = stoi(line.substr(line.find(" from ") + 5, line.find(" to ") - (line.find(" from ") + 5))) - 1;
    int dest = stoi(line.substr(line.find(" to ") + 4)) - 1;

    //cout << "move " << n << " from " << source << " to " << dest << endl;

    for(int i = 0 ; i < n ; i++) {
      auto item = stacks[source].back();
      stacks[dest].push_back(item);
      stacks[source].pop_back();
    }

    //print_stacks(stacks);
  }

  for(int i = 0 ; i < N ; i++)
    cout << stacks[i].back();
  cout << endl;

  return 0;
}