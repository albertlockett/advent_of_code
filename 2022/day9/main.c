#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

// we'll use a k-way tree to keep track of which locations the knot has visited
typedef struct kway_node {
  short*        values;
  short         level;
  kway_node*    left_child;
  kway_node*    right_child;
} kway_node;

typedef struct kway_tree {
  int         size;
  short       k;
  kway_node*  root;
} kway_tree;

#define EMPTY -1
#define INSERT_SUCCESS 1
#define NOT_INSERTED 0

kway_tree new_kway_tree(short k) {
  kway_tree tree;
  tree.size = 0;
  tree.k = k;
  tree.root = EMPTY;
  return tree;
}

kway_node* new_kway_node(short k, short* values, short level) {
  kway_node* node = malloc(sizeof(kway_node));
  node->level = level;
  node->left_child = EMPTY;
  node->right_child = EMPTY;

  node->values = malloc(k * sizeof(short));
  for (int i = 0; i < k; i++) node->values[i] = values[k]; // TODO copy?
  return node;
}

int is_same_point(short k, short* left, short* right) {
  // TODO this only works if k = 2
  int l = (u_int16_t*) left;
  int r = (u_int16_t*) right;
  return !(r & l);
}

int compare(kway_tree* tree, kway_node* node, short* values) {
  int dim = node->level % tree->k;
  return values[dim] - node->values[dim];
}

kway_node* insert(kway_tree* tree, kway_node* node, short* values, short level) {
  if (node == EMPTY) {
    tree->size++;
    return new_kway_node(tree->k, values, level);
  }

  if (is_same_point(tree->k, node->values, values)) {
    return node;
  }

  if (compare(tree, values, node->values) < 0) {
    node->left_child = insert(tree, node->left_child, values, node->level + 1);
  } else {
    node->right_child = insert(tree, node->right_child, values, node->level + 1);
  }
  return node;
}

#define U 'U'
#define D 'D'
#define L 'L'
#define R 'R'

typedef struct vector {
  short diretion;
  short magnitude;
} vector;

#define INCR(target, digit_char) \
    target = target * 10 + digit_char - '0'

vector parse_next_movement() {
  vector mv;
  mv.magnitude = -1;

  char ch;
  while (read(STDIN_FILENO, &ch, 1) > 0) {
    if (U == ch || D == ch || R == ch || L == ch) {
      mv.diretion = ch;
      continue;
    }

    if (' ' == ch) {
      continue;
    }

    if (ch >= '0' && ch <= '9') {
      INCR(mv.magnitude, ch);
      continue;
    }

    if (ch == '\n') {
      break;
    }
  }

  return mv;
}

void move_head(short* head, vector* move) {
  switch (move->diretion) {
    case U:
      head[0] += move->diretion;
      break;
    case D:
      head[0] -= move->diretion;
      break;
    case L:
      head[1] += move->diretion;
      break;
    case R:
      head[1] -= move->diretion;
      break;
  }
}

void move_tail(short* head, short* tail) {
  // TODO
}

int main() {
  short head[2];
  head[0] = 0;
  head[1] = 0;

  short tail_pos[2];
  tail_pos[0] = 0;
  tail_pos[1] = 0;

  vector move;
  do {
    move_head(head, &move);

  } while(move.magnitude > 0);

  return 0;
}
