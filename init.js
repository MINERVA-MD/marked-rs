import * as wasm from './pkg';

let result1 = wasm.parse("# This is a TEST H1 Heading");
let result2 = wasm.parse("## This is a TEST H2 Heading");
let result3 = wasm.parse("### This is a TEST H3 Heading");

let result4 = wasm.parse("**This Text should be bold**");
let result5 = wasm.parse("*This Text should be italicized*");


console.log(result1);
console.log(result2);
console.log(result3);
console.log(result4);
console.log(result5);
