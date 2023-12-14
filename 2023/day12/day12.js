var fs = require('fs');
var _ = require('lodash');

var process = require('process');


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
    // console.log("target_segments", target_segments, "line", line)
    const base_line = line.replace(/\?/g, '.');
    // let farthest_child_start = line.length + 5;
    let farthest_child_start = -1

    if (target_segments.length == 0) {
        const segments = get_segments(base_line);
        let farthest_child_start = line.length + 5;
        if (segments.length == 0) {
            return [1, farthest_child_start]
        }
        return [0, farthest_child_start]
    }

    const repair_segments = get_repair_segments(line);

    const segment = target_segments[0];
    let segment_mask = '';
    for (let i = 0; i < segment; i++) {
        segment_mask += '#';
    }

    let results = 0;
    let farthest_start = 0;

    for (let [offset, length] of repair_segments) {
        let farthest_child_start = -1

        for (let i = 0; i < length; i++) {
            let offset_start_segment = offset + i;
            let offset_after_segment = offset + i + segment;
            const test_line = base_line.slice(0, offset_start_segment) 
                + segment_mask
                + base_line.slice(offset_after_segment);

            if (offset_start_segment > 1) {
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
        farthest_start = line.length + 5;
    }

    return [results, farthest_start]

}



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

const input = fs.readFileSync('input.txt', 'utf8');
const lines = input.split('\n');

// console.log({ lines })

let total = 0;
for (let i in lines) {
    const full_line = lines[i];
    if (full_line.length == 0) {
        continue;
    }
    const [line, t] = full_line.split(' ');
    const target_segments = t.split(',').map(x => parseInt(x));
    const [results, asdf, results_literal] = findAllowedLocations(target_segments, line);
    // console.log(`fl1 ${full_line} = ${results}`)
    total += results;
}

console.log("total", total, "expect 21");

let total2 = 0

findAllowedLocations = _.memoize(findAllowedLocations, (target_segments, line) => {
    return target_segments.join(',') + line;
})

// let start = parseInt(process.argv[2]);
// let end = parseInt(process.argv[3]);
// console.log({ start, end })

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

    // const [results, asdf, results_literal] = findAllowedLocations(target_segments, line);
    const [results, asdf, results_literal] = findAllowedLocations(targets_5, line_5);
    let num_results = _.uniq(results_literal).length;
    // console.log(`fl2 ${full_line} = ${results}`)
    total2 += results;
}
console.log("total2", total2);


