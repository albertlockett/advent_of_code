const len = 14;
const [_, r] = (
    x = require('fs').readFileSync('input.txt').toString().split(''), 
    x.reduce(([seq, found], curr, idx) => {
        if (seq.push(curr) < len+1) return [seq, found]
        
        seq.shift()
        for (let i = 0;   i < len -1; i++)
        for (let j = i+1; j < len;    j++) 
            if (seq[i] == seq[j]) return [seq, found]
        
        x.splice(1)
        return [seq, idx + 1]
    }, [[], -1])
)
console.log(r)