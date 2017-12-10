var fs = require('fs');

let raw = fs.readFileSync("./d2.tsv", { encoding: "utf8" });

let input = raw.split("\n")
    .map(line => line.split("\t")
        .map(nr => parseInt(nr)));


let chkSum = input.reduce((acc, curr) => {
    let maxmin = curr.reduce((stat, val, index) => {
        if(val > stat.max)
            stat.max = val;
        else if(val < stat.min)
            stat.min = val;
        
        return stat;
    }, { min: curr[0], max: curr[0] });
    let lineDiff = maxmin.max - maxmin.min;
    return acc += lineDiff;
}, 0);

console.log(chkSum);