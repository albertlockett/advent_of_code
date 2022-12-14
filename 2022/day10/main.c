#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

#define OP_NOOP 0
#define OP_ADDX 1
#define OP_END  2
#define INCR(target, digit_char) \
    target = target * 10 + digit_char - '0'

typedef struct instruction {
  short opcode;
  short val;
  short inst_len;
} instruction;

short parse_op_code() {
  char ch;
  while(read(STDIN_FILENO, &ch, 1)) {
    if (ch == 'n') {
      return OP_NOOP;
    }
    if (ch == 'a') {
      return OP_ADDX;
    }
  }
  return OP_END;
}

short parse_number() {
  char ch;
  short is_negative = 0;
  short value = 0;
  while(read(STDIN_FILENO, &ch, 1)) {
    if (ch == ' ') continue;

    if (ch == '-') is_negative = 1;

    if (ch >= '0' && ch <= '9') {
      INCR(value, ch);
    }

    if (ch == '\n') break;
  }

  if (is_negative) value *= -1;
  return value;
}

instruction parse_next_instruction() {
  instruction inst;
  inst.opcode = parse_op_code();
  switch (inst.opcode) {
    case OP_NOOP:
      inst.inst_len = 1;
      inst.val = 0;
      break;
    case OP_ADDX:
      inst.inst_len = 2;
      inst.val = parse_number();
      break;
    case OP_END:
      inst.opcode = OP_END;
      inst.inst_len = 0;
      inst.val = 0;
  }
  return inst;
}

void print_instruction(instruction* inst) {
  switch (inst->opcode) {
    case OP_NOOP:
      printf("opcode = NOOP\n");
      break;
    case OP_ADDX:
      printf("opcode=ADDX, val=%d\n", inst->val);
      break;
    case OP_END:
      printf("opcode=END\n");
      break;
    default:
      printf("EPRINT unknown opcode\n");
  }
}

int main() {
  instruction inst;
  int cycle_count = 0;
  int reg_x = 1;
  int signals = 0;

  printf("part 2 ...\n");
  do {
    inst = parse_next_instruction();
    for (int i = 0; i < inst.inst_len; i++) {
      int x_pos = cycle_count % 40;
      
      if (
        reg_x-1 == x_pos ||
        reg_x+0 == x_pos ||
        reg_x+1 == x_pos
      ) {
        printf("#");
      } else {
        printf(".");
      }
      cycle_count++;
      if (cycle_count % 40 == 0) {
        printf("\n");
      }

      if ((cycle_count - 20 )% 40 == 0) {
        signals += cycle_count * reg_x;
      }
    }
    reg_x += inst.val;
  } while(inst.opcode != OP_END);

  printf("\npart 1 = %d", signals);
}
