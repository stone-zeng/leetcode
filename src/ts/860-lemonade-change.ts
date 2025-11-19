// 860. Lemonade Change
// https://leetcode.com/problems/lemonade-change/

function lemonadeChange(bills: number[]): boolean {
    const income = { 5: 0, 10: 0 };
    for (const bill of bills) {
        if (bill === 5) {
            income[5]++;
        } else {
            income[5]--;
            if (bill === 10) {
                income[10]++;
            } else {
                if (income[10] > 0) income[10]--;
                else income[5] -= 2;
            }
            if (income[5] < 0 || income[10] < 0) return false;
        }
    }
    return true;
}

console.log(lemonadeChange([5, 5, 5, 10, 20]));
console.log(lemonadeChange([5, 5, 10, 10, 20]));
console.log(lemonadeChange([5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5]));
