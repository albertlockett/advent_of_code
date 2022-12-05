#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>

#define EMPTY_STACK 0x00

#define INCR(target, digit_char) \
    target = target * 10 + digit_char - '0'

typedef struct crate {
  char supply; // 0x00 will be the empty stack
  struct crate* bottom;
} crate;

typedef struct move {
  short limit;
  short from;
  short to;
} move;

crate* push_crate(crate* bottom, char supply) {
  crate* new_crate = malloc(sizeof (crate));
  new_crate->supply = supply;
  new_crate->bottom = bottom;
  return new_crate;
}

char pop_crate(crate** stack) {
  crate* old_top = *stack;
  char supply = old_top->supply;
  *stack = old_top->bottom;
  free(old_top);
  return supply;
}

crate* invert_stack(crate* curr, crate* next) {
  if (next->supply != 0x00) {
    crate* tmp = next->bottom;
    next->bottom = curr;
    return invert_stack(next, tmp);
  }
  return curr;
}

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

    // we reached the row w/ all the numbers on it
    if (ch >= '0' && ch <= '9') {
      // read to end of line
      while(read(STDIN_FILENO, &ch, 1) > 0) if (ch == '\n') break;
      
      // break out input loop
      break;
    }
  }

  // we read the stacks top from bottom but set them up upsidw down using
  // the bottom pointers, so now must invert them
  for (int i = 0; i < stacksc; i++) {
    stacks[i] = invert_stack(stacks[i], stacks[i]->bottom);
  }
}

move parse_next_move(int* iter_continue) {
  char ch;
  int offset = 0;

  move next_move = {
    limit:  0,
    from:   0,
    to:     0
  };

  while(read(STDIN_FILENO, &ch, 1) > 0) {
    if (ch == 'f' /*from*/ || ch =='t' /*to*/) {
      offset++;
    }

    if (ch >= '0' && ch <= '9') {
      INCR(((short*)&next_move)[offset], ch);
    }

    if (ch == '\n') {
      return next_move;
    }
  };

  *iter_continue = 0;
  return next_move;
}

void handle_next_move(move move, crate* stacks[], int mover_9001_mode) {
  if (mover_9001_mode) {
    crate *tmp = malloc(sizeof (crate));

    for (int i = 0; i < move.limit; i++) {
      char supply = pop_crate(&stacks[move.from-1]);
      tmp = push_crate(tmp, supply);
    }
    for (int i = 0; i < move.limit; i++) {
      char supply = pop_crate(&tmp);
      stacks[move.to-1] = push_crate(stacks[move.to-1], supply);
    }
    free(tmp);
  } else {
    for (int i = 0; i < move.limit; i++) {
      char supply = pop_crate(&stacks[move.from-1]);
      stacks[move.to-1] = push_crate(stacks[move.to-1], supply);
    }
  }
}

// usage:
// cat ./input.txt| ./main [9000_mode]
int main(int argc, char** argv) {
  int mover_9001_mode = 0;
  if (argc > 1) mover_9001_mode = 1;

  crate* stacks[9]; // if there's more than 9 stacks I'll eat a lemon
  for (int i = 0; i < 9; i++) {
    stacks[i] = malloc(sizeof (crate));
    stacks[i]->supply = EMPTY_STACK;
  }
  parse_and_fill_stacks(9, stacks);

  int iter_continue = 1;
  move next_move;
  while (iter_continue) {
    next_move = parse_next_move(&iter_continue);
    handle_next_move(next_move, stacks, mover_9001_mode);
  }

  for (int i = 0; i < 9; i++) {
    printf("%c", stacks[i]->supply);
  }
  return 0;

}
