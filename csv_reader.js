let readline = require('readline');
//let stream = require('stream');
let fs = require('fs');

let file = 'input.csv';
let rl = readline.createInterface({
    input: fs.createReadStream(file),
    output: process.stdout,
    terminal: false
});
//let s = new stream.PassThrough();
//let rl = readline.createInterface(process.stdin, s);

let entries = new Map();
rl.on('line', function(line) {
    let [uuid, value] = line.split(",");
    value = parseFloat(value);

    if (!entries.has(uuid)) entries.set(uuid, []);
    entries.get(uuid).push(value);
});

rl.on('close', function () {
    let summaries = [];
    for (let [uuid, values] of entries) {
        let n = values.length;
        let sum = values.reduce((a, b) => a + b, 0);
        let mean = sum / n;

        let s = values.reduce((a, b) => a + (b - mean) ** 2, 0);
        let stdev = Math.sqrt(s / n);

        summaries.push([uuid, n, mean, stdev]);
    }
    summaries.sort((a, b) => b[1] - a[1]);

    for (let [uuid, n, mean, stdev] of summaries) {
        console.log(uuid + "\t" + n + "\t" + mean + "\t" + stdev);
    }
});
