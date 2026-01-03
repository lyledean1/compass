// Test JavaScript file with intentional issues

function test() {
    var x = 10; // Should trigger var keyword rule
    console.log("Debug message"); // Should trigger console.log rule
    return x;
}

test();
