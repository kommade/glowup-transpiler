// Variables
let a = 10;
var b = 20;
const c = 30;

// Data Types
let num = 42;
let str = "Hello, World!";
let bool = true;
let arr = [1, 2, 3];
let obj = { key: "value" };
let nul = null;
let undef;

// Control Structures
if (bool) {
    console.log("True");
} else {
    console.log("False");
}

for (let i = 0; i < 5; i++) {
    console.log(i);
}

while (num > 0) {
    console.log(num);
    num--;
}

switch (str) {
    case "Hello":
        console.log("Greeting");
        break;
    default:
        console.log("Unknown");
}

// Functions
function add(x, y) {
    return x + y;
}

const multiply = (x, y) => x * y;

// Objects
let person = {
    name: "John",
    age: 30,
    sayHello() {
        console.log("Hello!");
    }
};

// Classes
class Rectangle {
    constructor(width, height) {
        this.width = width;
        this.height = height;
    }

    area() {
        return this.width * this.height;
    }
}

// Promises
const promise = new Promise((resolve, reject) => {
    if (true) {
        resolve("Success");
    } else {
        reject("Error");
    }
});

// Modules
import { module1, module2 } from "./modules";
require("./modules");
module.exports = { module1, module2 };
export default { module1, module2 };
// Others
console.log(typeof str);
console.warn(delete person.age);
console.error(2 == 2);
console.info(void 0);
console.log(eval("2 + 2"));
console.log(3 === 3);
3 ? console.log("True") : console.log("False");
null ?? console.log("Null");
assert()
throw new Error("Error");

const e = async () => {
    try {
        await promise;
    } catch (error) {
        console.error(error);
    }
}