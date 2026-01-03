// Test Swift file with intentional issues

import Foundation

class TestViewModel {
    var userName: String?
    var userId: Int?

    func printUserInfo() {
        // Should trigger print_statement
        print("User: \(userName!)")  // Should also trigger force_unwrap

        // Should trigger dispatch_queue_main_async
        DispatchQueue.main.async {
            print("Updated UI")
        }
    }

    func unsafeOperation() {
        // Should trigger fatal_error
        fatalError("This should never happen")
    }

    func forceCast(value: Any) {
        // Should trigger force_cast
        let number = value as! Int
        print(number)
    }

    func magicNumbers() {
        // Should trigger magic_numbers
        let timeout = 30
        let maxRetries = 5
        let bufferSize = 1024
    }

    // TODO: Implement this function
    // FIXME: This needs to be optimized
    func todoExample() {
        let value: String? = nil
        // Multiple force unwraps
        let result = value!
        print(result!)
    }
}
