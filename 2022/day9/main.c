#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

// we'll use a k-way tree to keep track of which locations the knot has visited
typedef struct kway_node {
  short*        values;
  short         level;
  struct kway_node*    left_child;
  struct kway_node*    right_child;
} kway_node;

typedef struct kway_tree {
  int         size;
  short       k;
  kway_node*  root;
} kway_tree;

#define EMPTY (kway_node*) -1

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
  for (int i = 0; i < k; i++) node->values[i] = values[i];
  return node;
}

int is_same_point(short k, short* left, short* right) {
  for (int i = 0; i < k; i++) {
    if (left[i] != right[i]) return 0;
  }
  return 1;
}

int compare(kway_tree* tree, short* values, kway_node* node) {
  int dim = node->level % tree->k;
  return values[dim] - node->values[dim];
}


kway_node* insert_node(kway_tree* tree, kway_node* node, short* values, short level) {
  if (node == EMPTY) {
    tree->size++;
    return new_kway_node(tree->k, values, level);
  }

  if (is_same_point(tree->k, node->values, values)) {
    return node;
  }

  if (compare(tree, values, node) < 0) {
    node->left_child = insert_node(tree, node->left_child, values, node->level + 1);
  } else {
    node->right_child = insert_node(tree, node->right_child, values, node->level + 1);
  }
  return node;
}

void insert(kway_tree* tree, short* values) {
  kway_node* node = insert_node(tree, tree->root, values, 0);
  if (tree->root == EMPTY) {
    tree->root = node;
  }
}

#define U 'U'
#define D 'D'
#define L 'L'
#define R 'R'

typedef struct vector {
  char  diretion;
  short magnitude;
} vector;

#define INCR(target, digit_char) \
    target = target * 10 + digit_char - '0'

vector parse_next_movement() {
  vector mv;
  mv.magnitude = 0;

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
      head[0] += 1;
      break;
    case D:
      head[0] -= 1;
      break;
    case L:
      head[1] -= 1;
      break;
    case R:
      head[1] += 1;
      break;
  }
}

#define MOVE_N 0
#define MOVE_Y 2


int should_move(short* head, short* tail) {
  int v_dist = head[0] - tail[0];
  if (v_dist < 0) v_dist *= -1;

  int h_dist = head[1] - tail[1];
  if (h_dist < 0) h_dist *= -1;

  int total_dist = v_dist + h_dist;

  if (total_dist <= 1) {
    return MOVE_N;
  }

  if (total_dist == 2) {
    if (h_dist == v_dist == 1) {
      return MOVE_N;
    }
  }

  return MOVE_Y;
}

void move_tail(short *head, short* tail) {
    int v_dist = head[0] - tail[0];
    int h_dist = head[1] - tail[1];

    if (v_dist > 0) {
      tail[0]+=1;
    }
    if (v_dist < 0) {
      tail[0] -= 1;
    }
    if (h_dist > 0) {
      tail[1] += 1;
    }
    if (h_dist < 0) {
      tail[1] -= 1;
    }
}

int main() {
  kway_tree tree = new_kway_tree(2);

  short head[2];
  head[0] = 0;
  head[1] = 0;

  short tail[2];
  tail[0] = 0;
  tail[1] = 0;
  insert(&tree, tail);

  vector move;
  do {
    move = parse_next_movement();
    for (int i = 0; i < move.magnitude; i++) {
      move_head(head, &move);
      while (should_move(head, tail)) {
        move_tail(head, tail);
        insert(&tree, tail);
      }
    }

  } while(move.magnitude > 0);

  printf("part 1 = %d\n", tree.size);

  return 0;
}
