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
    const starts = [] 
    const contains = []

    let first = ''

    const chars = line.split('')
    for (let i in chars) {
        const letter = chars[i]

        for(let j in candidates) {
            let [word] = candidates[j]

            if (letter === word[0]) {
                if (word === "nine" && starts[j] == i - 2) {
                // handle special case of fucking 9
                } else {
                    starts[j] = i
                }
            }

            if (starts[j]) {
                if (i - starts[j] === word.length - 1) {
                    if (letter == word[word.length -1]) {
                        if (Object.keys(contains).length === 0) {
                            first = j
                        }
                        contains[j] = starts[j]
                    }
                }
                if (letter != word[i - starts[j]]) {
                    delete starts[j]
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

    const score = candidates[first][1] * 10 +  candidates[last][1]

    totalScore += score
}


console.log({ totalScore })