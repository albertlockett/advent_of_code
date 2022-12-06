console.log((
    len = 14,
    x = require('fs').readFileSync('input.txt').toString().split(''), 
    x.reduce((seq, ch, idx) => {
        if (seq.push(ch) < len+1) return seq
        
        seq.shift()
        for (let i = 0;   i < len -1; i++)
        for (let j = i+1; j < len;    j++) 
            if (seq[i] == seq[j]) return seq
        
        x.splice(1)
        return idx+1
    }, [])
))