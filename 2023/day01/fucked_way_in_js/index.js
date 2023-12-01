var fs = require('fs')

const values = fs.readFileSync('./input.txt').toString();

const candidates = [
    ["0", 0],
    ["1", 1],
    ["2", 2],
    ["3", 3],
    ["4", 4],
    ["5", 5],
    ["6", 6],
    ["7", 7],
    ["8", 8],
    ["9", 9],
    ["0", 0],
    
    // comment out this next 10 lines for part 1 only
    ["one", 1],
    ["two", 2],
    ["three", 3],
    ["four", 4],
    ["five", 5],
    ["six", 6],
    ["seven", 7],
    ["eight", 8],
    ["nine", 9],
    ["zero", 0],

]

const lines = values.split('\n')

let totalScore = 0

for(line of lines) {
    const starts = {}
    const contains = {}
    let first = ''

    const chars = line.split('')
    for (let i in chars) {
        const letter = chars[i]

        for(let [word, num] of candidates) {
            if (letter === word[0]) {
                if (word === "nine" && starts[word] == i - 2) {
                // handle special case of fucking 9
                } else {
                    starts[word] = i
                    // continue
                }
            }

            if (starts[word]) {
                if (i - starts[word] === word.length - 1) {
                    if (letter == word[word.length -1]) {
                        if (Object.keys(contains).length === 0) {
                            first = word
                        }
                        contains[word] = starts[word]
                    }
                }
                if (letter != word[i - starts[word]]) {
                    delete starts[word]
                }
                
            
            }
        }
    }

    let lastMax = -1
    let last = 0

    for (let num in contains) {
        let idx = contains[num]
        if (Number(idx) > Number(lastMax)) {
            lastMax = idx
            last = num
        }
    }

    const first_as_num = candidates.find(([ word ]) => word === first)[1]
    const last_as_num = candidates.find(([ word ]) => word === last)[1]

    const score = first_as_num * 10 +  last_as_num

    totalScore += score
}


console.log({ totalScore })