#include <iostream>
#include <vector>
#include <map>
#include <algorithm>

using namespace std;

bool shrink(string current, map<string, vector<string>> replacements, vector<string> results, int depth, int max_depth) {
  if(current == "e") {
    cout << depth << endl;
    return true;
  }

  if(depth >= max_depth) return false;

  for(auto result : results) {
    if(depth == 0) cout << result << endl;
    int loc = current.find(result);
    if(loc == string::npos) continue;

    for(auto rep : replacements[result]) {
      string new_str = current.substr(0, loc) + rep + current.substr(loc + result.length());
      if(shrink(new_str, replacements, results, depth+1, max_depth)) return true;
    }
  }

  return false;
}

int main() {
  string line;
  map<string, vector<string>> replacements;
  vector<string> results;

  while(getline(cin, line)) {
    if(line.length() == 0) {
      break;
    }

    int i_split = line.find(" => ");
    string lhs = line.substr(0, i_split);
    string rhs = line.substr(i_split + 4);

    replacements[rhs].push_back(lhs);
    if(find(results.begin(), results.end(), rhs) == results.end())
      results.push_back(rhs);
  }

  getline(cin, line);

  return 0;
}