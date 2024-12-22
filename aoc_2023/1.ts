import * as fs from 'fs';

function loadInput(path : string) {
    return fs.readFileSync(path, 'utf8');
}


const input = loadInput('./2023/input/1.1.txt');
let sum = 0;

// ---- Solution 1 ----
// const numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
//
// input.split('\n').forEach((row) => {
//     let p1 = 0;
//     let p2 = row.length - 1;
//    
//     while (p1 <= p2) {
//         if (!numbers.includes(row[p1])) {
//             p1++;
//         }
//        
//         if (!numbers.includes(row[p2])) {
//             p2--;
//         }
//        
//         if (numbers.includes(row[p1]) && numbers.includes(row[p2])) {
//             break;
//         }
//     }
//    
//     if (!numbers.includes(row[p1]) || !numbers.includes(row[p2])) {
//         return;
//     }
//    
//     const number = parseInt(row[p1] + row[p2]);
//     // console.log(number);    
//     sum += number;
// });

// ---- Solution 2 ---- 

const numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine'];
const stringToNumber = {
    'one' : '1',
    'two' : '2',
    'three' : '3',
    'four' : '4',
    'five' : '5',
    'six' : '6',
    'seven' : '7',
    'eight' : '8',
    'nine' : '9',
    '1' : '1',
    '2' : '2',
    '3' : '3',
    '4' : '4',
    '5' : '5',
    '6' : '6',
    '7' : '7',
    '8' : '8',
    '9' : '9',
}
input.split('\n').forEach((row) => {
    let p1 = 0;
    let p2 = row.length - 1;
    while(p1 <= p2) {
        const res1 = numbers.some((number) => row.substring(0, p1).includes(number));
        if (!res1) {
            p1++;
        }
        
        const res2 = numbers.some((number) => row.substring(p2, row.length).includes(number));
        if (!res2) {
            p2--;
        }
        
        if (res1 && res2) {
            break;
        }
    }
    
    if (!numbers.some((number) => row.substring(0, p1).includes(number)) || !numbers.some((number) => row.substring(p2, row.length).includes(number))) {
        return;
    }
    
    let number1 : string;
    let number2 : string;
    numbers.forEach((number) => {
        if (!number1 && row.substring(0, p1).includes(number)) {
            number1 = stringToNumber[number as keyof typeof stringToNumber];
        }
        
        if (!number2 && row.substring(p2, row.length).includes(number)) {
            number2 = stringToNumber[number as keyof typeof stringToNumber];
        }
    })
    
    console.log(number1!, number2!);
    sum += parseInt( number1! + number2!);
});

console.log(sum);