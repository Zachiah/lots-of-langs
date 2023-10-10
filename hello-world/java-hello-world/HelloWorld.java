class HelloWorld {
    public static void main(String[] args) {
        System.out.println(String.format("Hello %s", args.length > 0 ? args[0] : "Nobody"));
    }
}