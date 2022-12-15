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

int main() {
  return 0;
}
