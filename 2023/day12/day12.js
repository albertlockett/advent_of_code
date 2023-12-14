var fs = require('fs');


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
    if (target_segments.length == 0) {
        return [1, 0]
    }

    const repair_segments = get_repair_segments(line);

    const base_line = line.replace(/\?/g, '.');

    const segment = target_segments[0];
    let segment_mask = '';
    for (let i = 0; i < segment; i++) {
        segment_mask += '#';
    }

    let results = 0;
    let farthest_start = 0;
    let farthest_child_start = line.length + 5;
    

    for (let [offset, length] of repair_segments) {
        for (let i = 0; i < length; i++) {
            if (offset + i + segment > farthest_child_start - 1) {
                break;
            }
            if (segment + offset + i > line.length) {
                break;
            }
            const test_line = base_line.slice(0, offset + i) 
                + segment_mask
                + base_line.slice(offset + i + segment);
            farthest_start = offset + i;

            let result_segs = get_segments(test_line);
            if (result_segs[0] == segment) {
                let children = findAllowedLocations(
                    target_segments.slice(1),
                    line.slice(offset + segment + 1)
                );
                results += children[0];
                farthest_child_start = children[1] + offset + segment + i + 1;
            } else {
                if (results > 0) {
                    // or break?
                    return [results, farthest_start]
                }
            }
        }
    }

    return [results, farthest_start]
}

let line = ''
let target_segments = []
let results = null

line = '???..###'
target_segments = [1, 1, 3]
results = findAllowedLocations(target_segments, line);

line = ".??..??...?##.";
target_segments = [1, 1, 3];
results = findAllowedLocations(target_segments, line);
console.log("results", results);
// expect 4?

// line = "?#?#?#?#?#?#?#"
// target_segments = [1,3,1,6]
// results = findAllowedLocations(target_segments, line);
// console.log("results", results);
// expect 1

const input = fs.readFileSync('input.txt', 'utf8');
const lines = input.split('\n');
console.log({ lines })

for (let i in lines) {
    const full_line = lines[i];
    if (full_line.length == 0) {
        continue;
    }
    const [line, t] = full_line.split(' ');
    const target_segments = t.split(',').map(x => parseInt(x));
    const [results] = findAllowedLocations(target_segments, line);
    console.log(line)
    console.log("results", results);
}
