let a: number = 0;
let b: number = 1; 

for (let i = 0; i <= 100; i++) {  
    let c: number = a + b;
    console.log(`Fib index ${i} is ${c}`);
    a = b; 
    b = c;
}