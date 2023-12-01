#include <iostream>
#include <algorithm>
#include <set>
#include <map>

using namespace std;

const int ROW = 2000000;

void print_cave(set<pair<long, long>>& sensors, set<pair<long, long>>& beacons, long top, long right, long bottom, long left) {
  for(int y = top ; y <= bottom ; y++) {
    for(int x = left ; x <= bottom ; x++) {
      if(sensors.contains(make_pair(x, y))) cout << 'S';
      else if(beacons.contains(make_pair(x, y))) cout << 'B';
      else cout << '.';
    }
    cout << endl;
  }
  cout << endl;
}

long manhattan(pair<long, long> sensor, pair<long, long> beacon) {
  return abs(sensor.first - beacon.first) + abs(sensor.second - beacon.second);
}

int main() {
  string line;
  long left = INT64_MAX, right = INT64_MIN, top = INT64_MAX, bottom = INT64_MIN;
  set<pair<long, long>> sensors, beacons;
  map<pair<long, long>, pair<long, long>> connections;

  while(getline(cin, line)) {
    pair<long, long> sensor;
    pair<long, long> beacon;

    sensor.first = stoi(line.substr(line.find("x=") + 2));
    sensor.second = stoi(line.substr(line.find("y=") + 2));

    beacon.first = stoi(line.substr(line.rfind("x=") + 2));
    beacon.second = stoi(line.substr(line.rfind("y=") + 2));

    sensors.insert(sensor);
    beacons.insert(beacon);

    long dist = manhattan(sensor, beacon);
    left = min(left, sensor.first - dist);
    right = max(right, sensor.first + dist);
    top = min(top, sensor.second - dist);
    bottom = max(bottom, sensor.second + dist);

    left = min(left, beacon.first);
    right = max(right, beacon.first);
    top = min(top, beacon.second);
    bottom = max(bottom, beacon.second);

    connections[sensor] = beacon;
  }

  //print_cave(sensors, beacons, top, right, bottom, left);

  set<int> invalid;
  int count = 0;
  for(int x = left ; x <= right ; x++) {
    if(beacons.contains(make_pair(x, ROW))) continue;
    for(auto s : sensors) {
      long sdist = manhattan(s, connections[s]);
      long dist = manhattan(s, make_pair(x, ROW));
      if(dist <= sdist) invalid.insert(x);
    }
  }

  cout << invalid.size() << endl;

  return 0;
}