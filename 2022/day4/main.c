#include <stdio.h>
#include <unistd.h>

// increment the value of some number by appending a char digit
#define INCR(target, digit_char) \
    target = target * 10 + digit_char - '0'

// continuing iterating standard in controlled by some flag being > 0
#define DO_CONTINIUE(continue_iter)         continue_iter > 0
#define STOP_ITER(contine_iter)             *continue_iter = 0
#define INIT_ITER_CONTINUE(contine_iter)    contine_iter = 1;

struct region_limits {
    u_int8_t lower;
    u_int8_t upper;
};

struct elf_region_pair {
    struct region_limits elf_one;
    struct region_limits elf_two;
};

struct elf_region_pair parse_next_pair(int* continue_iter) {
    struct elf_region_pair pair;
    pair.elf_one.lower = 0;
    pair.elf_one.upper = 0;
    pair.elf_two.lower = 0;
    pair.elf_two.upper = 0;
    int pair_offset = 0;
    char ch;

    while(read(0, &ch, 1) > 0){
        if ('-' == ch || ',' == ch || '-' == ch) {
            pair_offset++;
            continue;
        }

        if (ch >='0' && ch <= '9') {
            INCR(((u_int8_t*)&pair)[pair_offset], ch);
            continue;
        }

        if ('\n' == ch) {
            return pair;
        }
    }
    STOP_ITER(continue_iter);
}

int is_limits_contained(struct region_limits r1, struct region_limits r2) {
    if (r1.lower <= r2.lower && r1.upper >= r2.upper) {
        return 1;
    }
    return 0;
}

int is_region_contained(struct elf_region_pair pair) {
    struct region_limits r1 = pair.elf_one;
    struct region_limits r2 = pair.elf_two;
    int contained = is_limits_contained(r1, r2);
    if (contained) {
        return contained;
    }
    contained += is_limits_contained(r2, r1);
    return contained;
}

int do_limits_overlap(struct region_limits r1, struct region_limits r2) {
    if (r1.lower <= r2.lower && r2.lower <= r1.upper) {
        return 1;
    }
    return 0;
}

int do_regions_overlap(struct elf_region_pair pair) {
    struct region_limits r1 = pair.elf_one;
    struct region_limits r2 = pair.elf_two;

    int overlapping = do_limits_overlap(r1, r2);
    if (overlapping) {
        return overlapping;
    }
    overlapping += do_limits_overlap(r2, r1);
    return overlapping;
}

int main(int argc, char** argv) {
    int regions_contained = 0;
    int regions_overlapped = 0;

    struct elf_region_pair pair;
    int continue_iter;

    INIT_ITER_CONTINUE(continue_iter);
    while (DO_CONTINIUE(continue_iter)) {
        pair = parse_next_pair(&continue_iter);
        regions_contained += is_region_contained(pair);
        regions_overlapped += do_regions_overlap(pair);
    }
    
    printf("part 1 = there are %d contained regions\n", regions_contained);
    printf("part 2 = there are %d overlapping regions\n", regions_overlapped);
    return 0;
}