#include <iostream>
#include <algorithm>
#include <set>
#include <map>

using namespace std;

const long MAX_COORD = 4000000;

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
  map<pair<long, long>, long> distances;

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
    distances[sensor] = dist;
  }

  //print_cave(sensors, beacons, top, right, bottom, left);

  set<pair<long, long>> points;
  for(auto s : sensors) {
    auto dist = distances[s] + 1;

    for(int i = -dist ; i <= dist ; i++) {
      long height = dist - abs(i);
      points.insert(make_pair(s.first + i, s.second + height));
      points.insert(make_pair(s.first + i, s.second - height));
    }
  }

  for(auto p : points) {
    if(p.first < 0 || p.first > MAX_COORD || p.second < 0 || p.second > MAX_COORD) continue;
    if(beacons.contains(p)) continue;

    bool invalid = false;
    for(auto s : sensors) {
      long sdist = manhattan(s, connections[s]);
      long dist = manhattan(s, p);
      if(dist <= sdist) {
        invalid = true;
        break;
      }
    }

    if(!invalid) {
      cout << p.first * 4000000L + p.second << endl;
      return 0;
    }
  }

  return 0;
}