#include <unistd.h>
#include <stdlib.h>

#define EMPTY_STACK 0x00

typedef struct crate {
  char supply; // 0x00 will be the empty stack
  struct crate* bottom;
} crate;

crate* push_crate(crate* bottom, char supply) {
  crate* new_crate = malloc(sizeof (crate));
  new_crate->supply = supply;
  new_crate->bottom = bottom;
  return new_crate;
}

crate* invert_stack(crate* curr, crate* next) {
  if (next->supply != 0x00) {
    crate* tmp = next->bottom;
    next->bottom = curr;
    return invert_stack(next, tmp);
  }
  return curr;
}

/**
 * [S]                 [T] [Q]        
 * [L]             [B] [M] [P]     [T]
 * [F]     [S]     [Z] [N] [S]     [R]
 * [Z] [R] [N]     [R] [D] [F]     [V]
 * [D] [Z] [H] [J] [W] [G] [W]     [G]
 * [B] [M] [C] [F] [H] [Z] [N] [R] [L]
 * [R] [B] [L] [C] [G] [J] [L] [Z] [C]
 * [H] [T] [Z] [S] [P] [V] [G] [M] [M]
 *  1   2   3   4   5   6   7   8   9 
 * 1234567890123456789012345678901234 <-- column number
 * 0         1         2         3
*/
void parse_and_fill_stacks(int stacksc, crate** stacks) {
  char ch;
  int column = 0;

  while(read(STDIN_FILENO, &ch, 1) > 0) {
    column++;

    if (ch >= 'A' && ch <= 'Z') {
      int stack_num = (column - 2) / 4;
      stacks[stack_num] = push_crate(stacks[stack_num], ch);
    }

    if (ch == '\n') {
      column = 0;
      continue;
    }

    if (ch >= '0' && ch <= '9') {
      break;
    }
  }

  for (int i = 0; i < stacksc; i++) {
    stacks[i] = invert_stack(stacks[i], stacks[i]->bottom);
  }
}


int main(int argc, char** argv) {
  crate* stacks[9]; // if there's more than 8 stacks I'll eat a lemon
  for (int i = 0; i < 9; i++) {
    stacks[i] = malloc(sizeof (crate));
    stacks[i]->supply = EMPTY_STACK;
  }

  parse_and_fill_stacks(9, stacks);
  return 0;

}
