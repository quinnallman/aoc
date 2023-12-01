#include <iostream>
#include <vector>
#include <map>

using namespace std;

struct valve {
  string id;
  int rate;
  vector<string> connections;
  bool open;
};

void print_valve(valve v) {
  cout << "Valve " << v.id << " has flow rate=" << v.rate << "; tunnels lead to valves ";
  for(auto c : v.connections)
    cout << c << ", ";
  cout << endl;
}

int cost(string start, string end, map<string, valve>& valves, bool debug = false) {
  char ignore;
  map<string, map<string, int>> scores;
  for(auto v : valves)
    for(auto w : valves)
      scores[v.first][w.first] = v.first == w.first ? 0 : -1;

  vector<string> visit;
  visit.push_back(end);
  while(!visit.empty()) {
    string current = visit.back();
    visit.pop_back();

    int score = max(scores[end][current] + 1, 1);
    for(auto c : valves[current].connections) {
      if(end == c) continue;
      if(scores[end][c] < 0 || score < scores[end][c]) {
        scores[end][c] = score;

        if(find(visit.begin(), visit.end(), c) == visit.end()) {
          visit.push_back(c);
        }
      }
    }
  }

  return scores[end][start];
}

int calc_pressure(vector<string>& path, map<pair<string, string>, int>& costs, map<string, valve>& valves) {
  int time = 0;
  int pps = 0;
  int pressure = 0;
  for(int i = 1 ; i < path.size() ; i++) {
    int extra_time = costs[make_pair(path[i-1], path[i])] + 1;
    if(time + extra_time > 30) {
      return pressure + pps * (30 - time);
    } else {
      time += extra_time;
      pressure += pps * extra_time;
      pps += valves[path[i]].rate;
    }
  }

  if(time < 30) {
    pressure += pps * (30 - time);
  }

  return pressure;
}

void print_path(vector<string>& path) {
  for(auto p : path) {
    cout << p << " -> ";
  }
  cout << endl;
}

int count_minutes(vector<string>& path, map<pair<string, string>, int>& costs) {
  int ret = 0;
  for(int i = 1 ; i < path.size() ; i++) {
    ret += costs[make_pair(path[i-1], path[i])] + 1;
  }

  return ret;
}

int solve(vector<string> path, map<pair<string, string>, int>& costs, map<string, valve>& valves, vector<string>& working) {
  if(path.size() == working.size() + 1 || count_minutes(path, costs) >= 30) {
    return calc_pressure(path, costs, valves);
  }

  int pressure = 0;
  for(int i = 0 ; i < working.size() ; i++) {
    if(find(path.begin(), path.end(), working[i]) != path.end()) continue;
    path.push_back(working[i]);
    pressure = max(pressure, solve(path, costs, valves, working));
    path.pop_back();
  }

  return pressure;
}

int main() {
  string line;

  map<string, valve> valves;
  string start;
  vector<string> working;

  while(getline(cin, line)) {
    valve v;

    v.id = line.substr(6, 2);
    v.rate = stoi(line.substr(23, line.find(';') - 23));
    v.open = false;

    if(line.find(',') == string::npos) {
      line = line.substr(line.find("valve ") + 6);
    } else {
      line = line.substr(line.find("valves ") + 7);
    }

    int pos;
    while((pos = line.find(", ")) != string::npos) {
      v.connections.push_back(line.substr(0, pos));
      line = line.substr(pos + 2);
    }
    v.connections.push_back(line);

    valves[v.id] = v;

    if(start.length() < 1)
      start = v.id;
    if(v.rate > 0)
      working.push_back(v.id);
  }

  map<pair<string, string>, int> costs;
  for(auto w : working) {
    costs[make_pair(start, w)] = cost(start, w, valves);
  }

  for(int i = 0 ; i < working.size() - 1; i++) {
    for(int j = i + 1 ; j < working.size() ; j++) {
      int c = cost(working[i], working[j], valves);
      costs[make_pair(working[i], working[j])] = c;
      costs[make_pair(working[j], working[i])] = c;
    }
  }

  vector<string> path;
  path.push_back(start);
  //int max = solve(path, costs, valves, working);

  //cout << max << endl;

  for(int i = 0 ; i < working.size() ; i++) {
    if(i == 0) {
      cout << "\tOJ\t";
      for(int j = 0 ; j < working.size() ; j++)
        cout << working[j] << '\t';
      cout << endl;
      cout << "OJ\t0\t";
      for(int j = 0 ; j < working.size() ; j++) {
        cout << costs[make_pair("OJ", working[j])] << '\t';
      }
      cout << endl;
    }
    cout << working[i] << '\t' << costs[make_pair(working[i], "OJ")] << '\t';
    for(int j = 0 ; j < working.size() ; j++) {
      cout << costs[make_pair(working[i], working[j])] << '\t';
    }
    cout << endl;
  }
  cout << endl;

  // for(auto v : valves) {
  //   cout << v.first << '\t';
  //   cout << costs[make_pair("SA", v.first)] << endl;
  // }
  // cout << endl;

  cout << cost("SA", "RL", valves, true) << endl;
  print_valve(valves["SA"]);
  print_valve(valves["RL"]);

  //cout << minute_solve("OJ", 

  return 0;
}