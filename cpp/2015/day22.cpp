#include <iostream>
#include <vector>

using namespace std;

struct spell {
  int mana;
  int damage;
  int hp;
  int armour;
  int turns;
  int regen;
  int turns_left;
};

struct character {
  int hp;
  int mana;
  int damage;
  int armour;
};

spell spells[] = {
  { .mana = 53, .damage = 4, .hp = 0, .armour = 0, .turns = 0, .regen = 0, .turns_left = 0 },
  { .mana = 73, .damage = 2, .hp = 2, .armour = 0, .turns = 0, .regen = 0, .turns_left = 0 },
  { .mana = 113, .damage = 0, .hp = 0, .armour = 7, .turns = 6, .regen = 0, .turns_left = 0 },
  { .mana = 173, .damage = 3, .hp = 0, .armour = 0, .turns = 6, .regen = 0, .turns_left = 0 },
  { .mana = 229, .damage = 0, .hp = 0, .armour = 0, .turns = 5, .regen = 101, .turns_left = 0 },
};

pair<bool, int> run(character player, character boss, bool player_turn, vector<spell>& effects) {
  if(player.hp < 0) return make_pair(false, -1);
  if(boss.hp < 0) return make_pair(true, 0);

  if(spells[2].turns_left > 0) spells[2].turns_left--;
  if(spells[2].turns_left == 0) player.armour -= spells[2].armour;

  if(spells[3].turns_left > 0) {
    boss.hp -= spells[3].damage;
    spells[3].turns_left--;
  }

  if(spells[4].turns_left > 0) {
    player.mana += spells[4].regen;
    spells[4].turns_left--;
  }

  if(player_turn) {
    int min = -1;
    int mana = 0;

    if(player.mana < spells[0].mana) {
      player.mana -= spells[0].mana;
      boss.hp -= spells[0].damage;
      pair<bool, int> res = run(player, boss, !player_turn, effects);
      boss.hp += spells[0].damage;
      player.mana += spells[0].mana;
      if(res.first) {
        mana = spells[0].mana + res.second;
        if(min < 0 || mana < min)
          min = mana;
      }
    }
    if(player.mana < spells[1].mana) {
      player.mana -= spells[1].mana;
      boss.hp -= spells[1].damage;
      player.hp += spells[1].hp;
      pair<bool, int> res = run(player, boss, !player_turn, effects);
      player.hp -= spells[1].hp;
      boss.hp += spells[1].damage;
      player.mana += spells[1].mana;
      if(res.first) {
        mana = spells[0].mana + res.second;
        if(min < 0 || mana < min)
          min = mana;
      }
    }
    if(player.mana < spells[2].mana) {
      if(spells[2].turns_left < 1) {
        player.mana -= spells[2].mana;
        spells[2].turns_left = spells[2].turns;
        
        spells[2].turns_left = 0;
        player.mana += spells[2].mana;
      }
    }
    if(player.mana < spells[0].mana) {
    }
    if(player.mana < spells[0].mana) {
    }

    if(min >= 0) {
      return make_pair(true, min);
    } else {
      return make_pair(false, -1);
    }
  } else {
    player.hp -= max(boss.damage - player.armour, 1);
    return run(player, boss, !player_turn, effects);
  }
}

int main() {
  character player { .hp = 50, .mana = 500, .damage = 0, .armour = 0 };
  character boss { .hp = 71, .mana = 0, .damage = 10, .armour = 1 };

  vector<spell> effects;
  auto res = run({.hp = 50, .mana = 500, .damage = 0, .armour = 0 }, {.hp = 71, .mana = 0, .damage = 10, .armour = 1 }, true, effects);

  cout << res.second << endl;

  return 0;
}