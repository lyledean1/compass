// Test Go file with intentional issues

package main

import "fmt"

func main() {
    result, err := doSomething() // Should trigger error check rule
    fmt.Println(result)
}

func doSomething() (int, error) {
    return 42, nil
}

func dangerous() {
    panic("oh no") // Should trigger panic rule
}
