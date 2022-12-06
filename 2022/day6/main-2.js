var len, input;
console.log((
    len = 14,
    input = require('fs').readFileSync('input.txt').toString().split(''), 
    input.reduce((seq, ch, idx) => {
        for (let i = seq.push(ch) - 2; i >= 0; i--)
            if (seq[i] == ch) 
                return seq.slice(i+1, len)
        return seq.length < len ?  seq : (input.splice(0), ++idx)
    }, [])
))