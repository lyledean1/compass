// Test Java file with intentional issues

public class Test {
    public static void main(String[] args) {
        System.out.println("Hello"); // Should trigger system.out rule

        int result = calculate(10);
        System.out.println(result);
    }

    public static int calculate(int x) {
        return x * 42; // Magic number should be detected
    }

    public static String getName() {
        return null; // Should trigger null return rule
    }
}
