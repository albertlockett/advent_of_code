#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

typedef int item;

#define EMPTY_ITEM -1

typedef struct ring_buffer {
  short max_size;
  short  size;
  short  start;
  short  end;
  item*  buffer;
} ring_buffer;

ring_buffer new_ring_buffer() {
  ring_buffer rb;
  rb.start = 0;
  rb.end = 0;
  rb.size = 0;
  rb.max_size = 1;
  rb.buffer = malloc(rb.max_size * sizeof(item));
  return rb;
}

void grow_ring_buffer(ring_buffer* rb) {
  rb->max_size *= 2;
  item* new_buffer = malloc(rb->max_size);
  for (int i = 0; i < rb->size; i++) {
    new_buffer[i] = rb->buffer[(i + rb->start) % rb->size];
  }
  free(rb->buffer);
  rb->buffer = new_buffer;
  rb->start = 0;
  rb->end = rb->size -1;
}

void push(ring_buffer* rb, item wl) {
  if (rb->size == rb->max_size) {
    grow_ring_buffer(rb);
  }

  rb->size++;
  rb->end++;
  if (rb->end >= rb->max_size) {
    rb->end = 0;
  }
  rb->buffer[rb->end] = wl;
}

item pop(ring_buffer* rb) {
  if (rb->size == 0) {
    return EMPTY_ITEM;
  }

  item wl = rb->buffer[rb->start];
  rb->size--;
  rb->start++;
  if (rb->start >= rb->max_size) {
    rb->start = 0;
  }

  return wl;
}

typedef struct worry_transform {
  int op;
  int level;
  int self;
} worry_transform;

#define OP_PLUS 0
#define OP_MULT 1

typedef struct monkey {
  int mod;
  int fail_target;
  int pass_target;
  int count_inspected;

  worry_transform transform;
  ring_buffer items;
} monkey;


item transform(worry_transform* transform, item wl) {
  return transform->op == OP_PLUS 
      ? wl + transform->level 
      : wl * transform->level;
}

void parse_to(char c) {
  char ch;
  while(read(STDIN_FILENO, &ch, 1) > 0)
    if (ch == c) return;
}

#define INCR(target, digit_char) \
    target = target * 10 + digit_char - '0'


void parse_items_list(monkey* m) {
  item value = 0;
  char ch;
  while(read(STDIN_FILENO, &ch, 1) > 0) {
    if (ch >= '0' && ch <= '9') {
      INCR(value, ch);
    }
    if (ch == ',') {
      push(&m->items, value);
      value = 0;
    }
    if (ch == '\n') {
      return;
    }
  }
}

void parse_tranform(monkey* m) {
  char ch;
  
  parse_to('d'); // parse to letter ('d' in old)
  read(STDIN_FILENO, &ch, 1); // space
  read(STDIN_FILENO, &ch, 1); // op
  
  if ('+' == ch) {
    m->transform.op = OP_PLUS;
  } else {
    m->transform.op = OP_MULT;
  }

  read(STDIN_FILENO, &ch, 1); // space

  m->transform.level = 0;
  m->transform.self = 0;
  while(read(STDIN_FILENO, &ch, 1) > 0) {
    if (ch == 'o') {
      parse_to('\n');
      m->transform.self = 1;
    }
    
    if (ch >= '0' && ch <= '9') {
      INCR(m->transform.level, ch);
    }
    if ('\n' == ch) {
      return;
    }
  }
}

int parse_number_after_letter_y() {
  int val = 0;
  parse_to('y'); // the 'y' in by or monkey
  char ch;
  read(STDIN_FILENO, &ch, 1); // parse the space

  while(read(STDIN_FILENO, &ch, 1) > 0) {
    if (ch >= '0' && ch <= '9') {
      INCR(val, ch);
    }
    if (ch == '\n') return val;
  }
  return val;
}

monkey parse_monkey() {
  monkey m;
  m.items = new_ring_buffer();

  parse_to('\n'); // parse title
  parse_to(':');  // parse to : before list of items
  parse_items_list(&m);
  parse_tranform(&m);
  m.mod = parse_number_after_letter_y();
  m.pass_target = parse_number_after_letter_y();
  m.fail_target = parse_number_after_letter_y();
  parse_to('\n'); // cleanup line after

  return m;
}



int main() {
  int rounds = 20;
  
  // TODO hardcode number of the monkeys?
  int num_monkeys = 7;
  monkey monkeys[num_monkeys];

  for (int i = 0; i < num_monkeys; i++) {
    monkeys[i] = parse_monkey();
  }

  for (int round = 0; round < rounds; round++) {
    for (int m = 0; m < num_monkeys; m++) {
      while (monkeys[m].items.size > 0) {
        monkeys[m].count_inspected++;
        item wl = pop(&monkeys[m].items);
        wl = transform(&monkeys[m].transform, wl);
        wl /= 3;

        int target = monkeys[m].pass_target;
        if (wl % monkeys[m].mod) {
          target = monkeys[m].fail_target;
        }

        push(&monkeys[target].items, wl);
      }
    }
  }

  // TODO find the monkeys with the most items inspected

  return 0;
}
