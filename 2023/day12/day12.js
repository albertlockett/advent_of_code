var fs = require('fs');
var _ = require('lodash');

function get_segments(line) {
    let segments = []
    let segment_length = 0;
    for (let i = 0; i < line.length; i++) {
        if (line[i] != '.') {
            segment_length++;
        } else {
            if (segment_length > 0) {
                segments.push(segment_length)
                segment_length = 0;
            }
        }
    }

    if (segment_length > 0) {
        segments.push(segment_length)
    }

    return segments;
}

function get_repair_segments(line) {
    let repair_segments = [];

    let segment_length = 0;
    for (let i = 0; i < line.length; i++) {
        if (line[i] != '.') {
            segment_length++;
        } else {
            if (segment_length > 0) {
                repair_segments.push([i - segment_length, segment_length]);
                segment_length = 0;
            }
        }
    }

    if (segment_length > 0) {
        repair_segments.push([line.length - segment_length, segment_length]);
        segment_length = 0;
    }

    return repair_segments
}

function findAllowedLocations(target_segments, line) {
    // line w/out any of the ? replaced
    const base_line = line.replace(/\?/g, '.');

    if (target_segments.length == 0) {
        const segments = get_segments(base_line);
        let farthest_child_start = line.length + 5; // no results so just assume child started @ infinity
        if (segments.length == 0) {
            return [1, farthest_child_start]
        }
        return [0, farthest_child_start]
    }

    const repair_segments = get_repair_segments(line);

    const segment = target_segments[0];

    // e.g. "###" if segment = 3
    let segment_mask = '';
    for (let i = 0; i < segment; i++) {
        segment_mask += '#';
    }

    let results = 0;
    let farthest_start = 0;
    
    // basic idea here is we'll pass the segment mask over segments that have repairs
    // and check if it fits the target segments

    for (let [offset, length] of repair_segments) {
        // keep track of where the child started so we know when we can stop
        // shifting the segment
        let farthest_child_start = -1

        for (let i = 0; i < length; i++) {
            // where our mask will be
            let offset_start_segment = offset + i;
            let offset_after_segment = offset + i + segment;

            // line w/ mask segment inserted
            const test_line = base_line.slice(0, offset_start_segment) 
                + segment_mask
                + base_line.slice(offset_after_segment);

            if (offset_start_segment > 1) {
                // if we shifted to create a new segment, break
                if ("#" == test_line[offset_start_segment - 2] && '.' == test_line[offset_start_segment - 1]) {
                    break;
                }
            }

            // if we're moving the segment into a position that the next segment
            // must occupy, break
            if (farthest_child_start > 0) {
                if (offset_start_segment > farthest_child_start) {
                    break;
                }
            }
            // if segment would extend past end of line, break
            if (offset_after_segment > offset + length) {
                break;
            }

            
            farthest_start = offset_start_segment;

            let result_segs = get_segments(test_line);
            if (result_segs[0] == segment) {
                // check further down the line
                let child_line = line.slice(offset_after_segment + 1)
                let children = findAllowedLocations(target_segments.slice(1), child_line);
                results += children[0];
                farthest_child_start = Math.max(farthest_child_start, children[1] + offset_after_segment + 1);
            } 
        }

        // check if contains # if so break
        if (line.slice(offset, offset + length).includes('#')) {
            break;
        }
    }

    if (results == 0) {
        // we had no results, so the furthest start is at "infinity"
        farthest_start = line.length + 5;
    }

    return [results, farthest_start]

}


const input = fs.readFileSync('input.txt', 'utf8');
const lines = input.split('\n');

let total = 0;
for (let i in lines) {
    const full_line = lines[i];
    if (full_line.length == 0) {
        continue;
    }
    const [line, t] = full_line.split(' ');
    const target_segments = t.split(',').map(x => parseInt(x));
    const [results] = findAllowedLocations(target_segments, line);
    total += results;
}

console.log("part 1 total", total);

let total2 = 0

findAllowedLocations = _.memoize(findAllowedLocations, (target_segments, line) => {
    return target_segments.join(',') + line;
})

for (let full_line of lines) {
    if (full_line.length == 0) {
        continue;
    }
    const [line, t] = full_line.split(' ');
    const target_segments = t.split(',').map(x => parseInt(x));

    let line_5 = []
    let targets_5 = []
    for (let i = 0; i < 5; i++) {
        line_5.push(line)
        targets_5 = [...targets_5, ...target_segments]
    }
    line_5 = line_5.join('?')

    const [results] = findAllowedLocations(targets_5, line_5);
    total2 += results;
}
console.log("part 2 total", total2);



function do_tests() {
    let line = ''
    let target_segments = []
    let results = null

    line = '???..###'
    target_segments = [1, 1, 3]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect 1");

    line = ".??..??...?##.";
    target_segments = [1, 1, 3];
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect 4");
    // expect 4?
    line = "?#?#?#?#?#?#?#?"
    target_segments = [1,3,1,6]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect 1");

    line = "?###????????"
    target_segments = [3,2,1]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "Expect 10"); // expect 10

    line = "?????"
    target_segments = [2,1]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect 3"); // expect 3

    line = '.?#.???#???'
    target_segments = [2,2]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect 2"); // expect 2

    line = '???#.?.?..'
    target_segments = [1,1,1]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect 5"); // expect 2

    line = '?#.?.?..'
    target_segments = [1,1]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect 2"); // expect 2

    line = '?....#?#??.'
    target_segments = [1,1]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect 1"); // expect 2

    line = '?..?#?.##?#?.??..?#?.##?#?.?'
    target_segments = [2,5,2,5]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect 1");

    line = '????.??.?#?.?????.??.?#?.'
    target_segments = [3,1,3,1]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect ?");

    line = '?#?.?????.??.?#?.'
    target_segments = [1,3,1]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect ?");

    line = '????.??.?#?.?????'
    target_segments = [3,1,3]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect ?");

    line = '.??.?#?.?????'
    target_segments = [1,3]
    results = findAllowedLocations(target_segments, line);
    console.log("results", results, "expect ?");
}
// do_tests();