import * as fs from 'fs';

function loadInput(path : string) {
    return fs.readFileSync(path, 'utf8');
}

const input = loadInput('./2023/input/2.1.txt');

// const allowedNumber = {
//     red : 12,
//     green : 13,
//     blue : 14,
// }
//
// let sum= 0;
// const colors = ['red', 'green', 'blue'];
// input.split('\n').forEach((row) => {
//     const gameId = parseInt(row.split(' ')[1]);
//     if (isNaN(gameId)) {
//         return;
//     }
//    
//     let isAllowed = true;
//     row.substring(row.indexOf(':') + 2).split(";").reduce((acc: string[], curr : string) => {
//         const currSplit = curr.trim().split(' ')
//         // console.log(currSplit);
//         for (let i = 0; i < currSplit.length - 1; i+=2) {
//             let color = currSplit[i + 1]; 
//             if (currSplit[i + 1][currSplit[i + 1].length - 1] == ',') {
//                 color = currSplit[i + 1].substring(0, currSplit[i + 1].length - 1);
//             }
//             // console.log(color);
//             if (allowedNumber[color as keyof typeof allowedNumber] < parseInt(currSplit[i])) {
//                 isAllowed = false;
//             }
//         }
//         
//         return acc;
//     }, []);
//     if (isAllowed) {
//         sum += gameId;
//     }
//    
//    
//    
// });


// ---- Solution 2 ----

let sum= 0;
const colors = ['red', 'green', 'blue'];
input.split('\n').forEach((row) => {
    const gameId = parseInt(row.split(' ')[1]);
    if (isNaN(gameId)) {
        return;
    }

    let isAllowed = true;
    const minColorNeeded = {
        red : 0,
        green : 0,
        blue : 0,
    } 
    row.substring(row.indexOf(':') + 2).split(";").reduce((acc: string[], curr : string) => {
        const currSplit = curr.trim().split(' ')
        // console.log(currSplit);
        for (let i = 0; i < currSplit.length - 1; i+=2) {
            let color = currSplit[i + 1]; 
            if (currSplit[i + 1][currSplit[i + 1].length - 1] == ',') {
                color = currSplit[i + 1].substring(0, currSplit[i + 1].length - 1);
            }
            minColorNeeded[color as keyof typeof minColorNeeded] = Math.max(minColorNeeded[color as keyof typeof minColorNeeded], parseInt(currSplit[i]));
        }

        return acc;
    }, []);
    if (isAllowed) {
        const power = Object.getOwnPropertyNames(minColorNeeded).reduce((acc, curr) => acc * minColorNeeded[curr as keyof typeof minColorNeeded], 1);
        sum += power;
    }

});

console.log(sum);
