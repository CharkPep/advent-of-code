import * as fs from 'fs';

function loadInput(path: string): string {
    return fs.readFileSync(path, 'utf8');
}

const input: string = loadInput('./2023/input/3.1.txt');
let sum: number = 0;
const inputRows: string[] = input.split('\n');
const digits: string[] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

// ---- Solution for Part 1 ----

// inputRows.forEach((row, index) => {
//     let number = '';
//     let toInclude = false;
//     for (let j = 0;j < row.length;j++) {
//        if (digits.includes(row[j])) {
//            number += row[j];
//            if (
//                (j - 1 >= 0 && !digits.includes(inputRows[index][j - 1]) && inputRows[index][j - 1] !== '.') ||
//                (j + 1 < row.length && !digits.includes(inputRows[index][j + 1]) && inputRows[index][j + 1] !== '.') ||
//                (index - 1 >= 0 && !digits.includes(inputRows[index - 1][j]) && inputRows[index - 1][j] !== '.') ||
//                (index + 1 < inputRows.length && !digits.includes(inputRows[index + 1][j]) && inputRows[index + 1][j] !== '.') || 
//                (index - 1 >= 0 && j - 1 >= 0 && !digits.includes(inputRows[index - 1][j - 1]) && inputRows[index - 1][j - 1] !== '.') ||
//                (index - 1 >= 0 && j + 1 < row.length && !digits.includes(inputRows[index - 1][j + 1]) && inputRows[index - 1][j + 1] !== '.') ||
//                (index + 1 < inputRows.length && j - 1 >= 0 && !digits.includes(inputRows[index + 1][j - 1]) && inputRows[index + 1][j - 1] !== '.') ||
//                (index + 1 < inputRows.length && j + 1 < row.length && !digits.includes(inputRows[index + 1][j + 1]) && inputRows[index + 1][j + 1] !== '.')
//            ) {
//                toInclude = true;
//            }
//            continue;
//        }
//
//        if (toInclude) {
//            console.log(number);
//            sum += parseInt(number);
//        }
//        number = '';
//        toInclude = false;
//
//     }
//
//     if (toInclude) {
//         console.log(number);
//         sum += parseInt(number);
//         number = '';
//         toInclude = false;
//     }
//
// })

// ---- Solution for Part 2 ----
// (Keep this part active for the second task)

function concatenateNumber(row: number, col: number, isIncreasing: boolean): string[] {
    if (col < 0 || !digits.includes(inputRows[row][col])) {
        return [];
    }

    let result: string[];
    if (isIncreasing) {
        result = concatenateNumber(row, col + 1, isIncreasing);
    } else {
        result = concatenateNumber(row, col - 1, isIncreasing);
    }

    result.push(inputRows[row][col]);
    return result;
}

inputRows.forEach((row: string, rowIndex: number) => {
    for (let colIndex: number = 0; colIndex < row.length; colIndex++) {
        const currentChar: string = row[colIndex];
        if (currentChar != '.' && !digits.includes(currentChar)) {
            let gearPartCount: number = 0;
            let gearPartProduct: number = 1;
            const allDirections: string[][] = [
                concatenateNumber(rowIndex, colIndex - 1, false),
                concatenateNumber(rowIndex, colIndex + 1, true).reverse(),
            ];

            if (inputRows[rowIndex - 1][colIndex] === '.') {
                const leftPart: string[] = concatenateNumber(rowIndex - 1, colIndex - 1, false);
                const rightPart: string[] = concatenateNumber(rowIndex - 1, colIndex + 1, true).reverse();
                allDirections.push(leftPart);
                allDirections.push(rightPart);
            } else {
                const leftPart: string[] = concatenateNumber(rowIndex - 1, colIndex, false);
                const rightPart: string[] = concatenateNumber(rowIndex - 1, colIndex + 1, true).reverse();
                allDirections.push(leftPart.concat(rightPart));
            }

            if (inputRows[rowIndex + 1][colIndex] === '.') {
                const leftPart: string[] = concatenateNumber(rowIndex + 1, colIndex - 1, false);
                const rightPart: string[] = concatenateNumber(rowIndex + 1, colIndex + 1, true).reverse();
                allDirections.push(leftPart);
                allDirections.push(rightPart);
            } else {
                const leftPart: string[] = concatenateNumber(rowIndex + 1, colIndex, false);
                const rightPart: string[] = concatenateNumber(rowIndex + 1, colIndex + 1, true).reverse();
                allDirections.push(leftPart.concat(rightPart));
            }

            allDirections.forEach((direction: string[]) => {
                if (direction && direction.length > 0) {
                    gearPartCount++;
                    gearPartProduct *= parseInt(direction.join(''));
                }
            });

            if (gearPartCount === 2) {
                sum += gearPartProduct;
            }
        }
    }
});
console.log(sum);
